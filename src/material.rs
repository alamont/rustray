use image::{
    imageops::{resize, FilterType, colorops::{grayscale}},
    ImageBuffer, Luma, Rgb,
};
use nalgebra::{Rotation3, Vector2, Vector3};
use rand::{thread_rng, Rng};
use std::{f32, sync::Arc};
use std::{fs, io};

use crate::hittable::HitRecord;
use crate::pdf::{CosinePdf, Pdf, UniformPdf};
use crate::ray::Ray;
use crate::texture::{ConstantTex, ImageTexture, Sampler::*, Texture, WrapMode::*};
use crate::vec::{
    deg_to_rad, get_sphere_uv, onb_local, random_cosine_direction, vec, vec3, vec_one, vec_zero,
};
use crate::vec::{random_unit_vec, random_vec_in_unit_sphere};

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0 * v.dot(&n) * n
}

pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = (-uv).dot(&n).min(1.0);
    let r_out_parallel = etai_over_etat * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.magnitude_squared()).sqrt() * n;
    r_out_parallel + r_out_perp
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub struct ScatterRecord {
    pub specular_ray: Option<Ray>,
    pub attenuation: Vector3<f32>,
    pub pdf: Option<Arc<dyn Pdf>>,
}

pub trait Material: Sync + Send {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<ScatterRecord> {
        // (Ray, Scatter direction, pdf)
        None
    }
    fn scattering_pdf(&self, _ray: &Ray, _hit: &HitRecord, _ray_scatterd: &Ray) -> f32 {
        0.0
    }
    fn emitted(&self, _ray_in: &Ray, _hit: &HitRecord) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
    fn is_solid(&self) -> bool {
        true
    }
}

pub struct EmptyMaterial {}
impl Material for EmptyMaterial {}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let w = hit.normal.normalize();
        let pdf = CosinePdf { w };

        // let scatter_direction = hit.normal + random_unit_vec();
        // let scattered = Ray::new(hit.p, scatter_direction);
        Some(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.value(hit.uv, hit.p),
            pdf: Some(Arc::new(pdf)),
        })
    }
    fn scattering_pdf(&self, _ray_in: &Ray, hit: &HitRecord, ray_scatterd: &Ray) -> f32 {
        let cosine = hit.normal.dot(&ray_scatterd.direction().normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }
}

pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(&ray.direction().normalize(), &hit.normal);
        let specular_ray = Ray::new(hit.p, reflected + self.fuzz * random_vec_in_unit_sphere());
        let attenuation = self.albedo.value(hit.uv, hit.p);
        Some(ScatterRecord {
            specular_ray: Some(specular_ray),
            attenuation,
            pdf: None,
        })
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
    pub color: Vector3<f32>,
    pub roughness: Arc<dyn Texture>,
    pub density: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let attenuation: Vector3<f32>;
        let etai_over_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let normal = (hit.normal
            + self.roughness.value(hit.uv, hit.p).x * random_vec_in_unit_sphere())
        .normalize();

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        if !hit.front_face {
            // Color
            let distance = (ray.origin() - hit.p).magnitude();
            attenuation = (-self.color.map(|x| 1.0 / x) * self.density * distance).map(f32::exp);
        } else {
            attenuation = vec(1.0, 1.0, 1.0);
        }

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &normal);
            Ray::new(hit.p, reflected)
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            let mut rng = thread_rng();
            let refracted_or_reflected = if rng.gen::<f32>() < reflect_prob  {
                reflect(&unit_direction, &normal)               
            } else {
                refract(&unit_direction, &normal, etai_over_etat)
            };
            Ray::new(hit.p, refracted_or_reflected)
        };

        Some(ScatterRecord {
            specular_ray: Some(scattered),
            attenuation,
            pdf: None,
        })
    }
}

impl Default for Dielectric {
    fn default() -> Dielectric {
        Dielectric {
            ref_idx: 1.52,
            color: vec(1.0, 1.0, 1.0),
            roughness: Arc::new(ConstantTex { color: vec_zero() }),
            density: 0.0, //TODO: rename to absorption coefficient or something like that
        }
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, _ray: &Ray, hit: &HitRecord) -> Vector3<f32> {
        if hit.front_face {
            self.emit.value(hit.uv, hit.p)
        } else {
            vec_zero()
        }
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let pdf = UniformPdf {};
        Some(ScatterRecord {
            specular_ray: None,
            attenuation: self.albedo.value(hit.uv, hit.p),
            pdf: Some(Arc::new(pdf)),
        })
    }
    fn is_solid(&self) -> bool {
        false
    }
}

pub struct DielectricSurfaceLambert {
    pub ref_idx: f32,
    pub albedo: Arc<dyn Texture>,
    pub roughness: Arc<dyn Texture>,
}

impl Material for DielectricSurfaceLambert {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = vec_one();
        let etai_over_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let normal = (hit.normal
            + self.roughness.value(hit.uv, hit.p).x * random_vec_in_unit_sphere())
        .normalize();

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(&unit_direction, &normal);
            Ray::new(hit.p, reflected)
        } else {
            let reflect_prob = schlick(cos_theta, etai_over_etat);
            let mut rng = thread_rng();
            let refracted_or_reflected = if rng.gen::<f32>() < reflect_prob {
                reflect(&unit_direction, &normal)
            } else {
                // Instead of refracting we do Lambertian
                let w = hit.normal.normalize();
                let pdf = CosinePdf { w };
                return Some(ScatterRecord {
                    specular_ray: None,
                    attenuation: self.albedo.value(hit.uv, hit.p),
                    pdf: Some(Arc::new(pdf)),
                });
            };
            Ray::new(hit.p, refracted_or_reflected)
        };

        Some(ScatterRecord {
            specular_ray: Some(scattered),
            attenuation,
            pdf: None,
        })
    }
    fn scattering_pdf(&self, _ray_in: &Ray, hit: &HitRecord, ray_scatterd: &Ray) -> f32 {
        let cosine = hit.normal.dot(&ray_scatterd.direction().normalize());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / f32::consts::PI
        }
    }
}

impl Default for DielectricSurfaceLambert {
    fn default() -> DielectricSurfaceLambert {
        DielectricSurfaceLambert {
            ref_idx: 1.52,
            albedo: Arc::new(ConstantTex { color: vec_one() }),
            roughness: Arc::new(ConstantTex { color: vec_zero() }),
        }
    }
}

pub trait EnvironmentMaterial: Sync + Send {
    fn emit(&self, ray: &Ray) -> Vector3<f32>;
    fn pdf_value(&self, _direction: &Vector3<f32>) -> f32 {
        1.0 / (4.0  * f32::consts::PI )
    }
    fn random(&self) -> Vector3<f32> {
        random_unit_vec()
    }
}

pub struct SimpleEnvironment {}

impl EnvironmentMaterial for SimpleEnvironment {
    fn emit(&self, ray: &Ray) -> Vector3<f32> {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

pub struct ConstantEnvironment {
    pub emit: Vector3<f32>,
}

impl EnvironmentMaterial for ConstantEnvironment {
    fn emit(&self, _ray: &Ray) -> Vector3<f32> {
        self.emit
    }
}

pub struct Environment {
    pub emit_tex: Arc<dyn Texture>,
    pub pdf: Vec<f32>,
    pub width: f32,
    pub height: f32,
    pub cum_pdf: Vec<f32>
}

const PI: f32 = f32::consts::PI;

impl EnvironmentMaterial for Environment {
    fn emit(&self, ray: &Ray) -> Vector3<f32> {
        let mut uv = get_sphere_uv(&ray.direction().normalize());
        uv.x = 1.0 - uv.x;
        self.emit_tex.value(uv, ray.direction())
    }
    fn pdf_value(&self, direction: &Vector3<f32>) -> f32 {
        let mut uv = get_sphere_uv(&direction.normalize());
        uv.x = 1.0 - uv.x;
        let x = (uv.x * (self.width - 1.0)) as u32;
        let y = ((1.0 - uv.y) * (self.height - 1.0)) as u32;
        self.pdf[xy_to_n(x, y, self.width as u32)]
    }
    fn random(&self) -> Vector3<f32> {
        let mut rng = thread_rng();
        let rnd_num = rng.gen::<f32>();

        let location = self.cum_pdf.binary_search_by(|v| {           
            v.partial_cmp(&rnd_num).unwrap()
        });
        let n = match location {
            Ok(i) => i,
            Err(i) => i
        };
        // let n = self.cum_pdf[idx];

        let (x, y) = n_to_xy(n, self.width as u32, self.height as u32);

        let u = x as f32 / (self.width - 1.0);
        let v = 1.0 - y as f32 / (self.height - 1.0);

        // println!("rnd: {}, n: {}, x:, {}, y: {}, u: {}, v: {}", rnd_num, n, x, y, u, v);

        let phi = u * 2.0 * PI - PI;
        let theta = v * PI - PI / 2.0;

        vec3(
            theta.cos() * phi.cos(),
            theta.sin(),
            theta.cos() * phi.sin(),
        )
    }
}

impl Environment {
    pub fn new(image_path: String) -> Self {
        let decoder =
            image::hdr::HdrDecoder::new(io::BufReader::new(fs::File::open(image_path).unwrap()))
                .unwrap();

        let env_image_buffer = ImageBuffer::from_raw(
            decoder.metadata().width,
            decoder.metadata().height,
            decoder
                .read_image_hdr()
                .unwrap()
                .iter()
                .flat_map(|p| vec![p[0], p[1], p[2]])
                .collect::<Vec<f32>>(),
        )
        .unwrap();

        let emit_tex = Arc::new(
            ImageTexture::new(env_image_buffer)
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        );

        let width = emit_tex.width as f32;
        let height = emit_tex.height as f32;

        let image_vec = emit_tex
            .clone()
            .image_buffer
            .enumerate_pixels()
            .map(|(_x, y, p)| {
                let a = (1.0 / height)
                    * PI
                    * ((1.0 / width) * 2.0 * PI)
                    * (((y as f32) / height - 0.5) * PI).cos();

                let image::Rgb(pixel_data) = p;
                let v = (pixel_data[0] + pixel_data[1] + pixel_data[2]) / 3.0;
                v * a
            })
            .collect::<Vec<f32>>();

        let sum: f32 = image_vec.iter().sum();
        let pdf = image_vec
            .iter()
            .map(|v| v / sum * image_vec.len() as f32 / 4.0 / PI)
            .collect::<Vec<f32>>();
        let pdf_sum: f32 = pdf.iter().sum();

        let cum_pdf = pdf.iter().fold(vec![], |mut acc, w| {
            if acc.len() > 0 {
                acc.push(acc.last().unwrap() + w / pdf_sum)
            } else {
                acc.push(*w / pdf_sum)
            }
            acc
        });
        // cum_pdf = cum_pdf.iter().map(|v| v / pdf_sum).collect::<Vec<f32>>();


        // let mut cum_pdf_sorted = cum_pdf.iter().enumerate().map(|(i, v)| (i, *v)).collect::<Vec<(usize, f32)>>();
        // cum_pdf_sorted.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // println!("{} {} {}", cum_pdf.first().unwrap(), cum_pdf[10000], cum_pdf.last().unwrap());

        Self {
            emit_tex,
            pdf,
            width,
            height,
            cum_pdf,
        }
    }
}

pub fn n_to_xy(n: usize, w: u32, h: u32) -> (u32, u32) {
    let x = ((n as f32 / w as f32).fract() * w as f32) as u32;
    let y = (n as f32 / w as f32).trunc() as u32;
    (x, y)
}

pub fn xy_to_n(x: u32, y: u32, w: u32) -> usize {
    (y * w + x) as usize
}

pub fn cum_sum(pdf: &Vec<f32>) -> Vec<f32> {
    pdf.iter().fold(vec![], |mut acc, w| {
        if acc.len() > 0 {
            acc.push(acc.last().unwrap() + w)
        } else {
            acc.push(*w)
        }
        acc
    })
}
use nalgebra::{Vector2, Vector3};
use rand::{thread_rng, Rng};
use std::{f32, sync::Arc};

use crate::hittable::{HitRecord};
use crate::ray::Ray;
use crate::vec::{random_unit_vec, random_vec_in_unit_sphere};
use crate::texture::{ConstantTex, Texture};
use crate::vec::{vec, vec_zero, vec_one};

pub fn reflect(v: Vector3<f32>, n: Vector3<f32>) -> Vector3<f32> {
    v - 2.0*v.dot(&n)*n
}

pub fn refract(uv: Vector3<f32>, n: Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = (-uv).dot(&n);
    let r_out_parallel = etai_over_etat * (uv + cos_theta*n);
    let r_out_perp = -(1.0 - r_out_parallel.magnitude_squared()).sqrt() * n;
    r_out_parallel + r_out_perp
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0*r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
    fn emitted(&self, _ray: &Ray, _hit: &HitRecord) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }
    fn is_solid(&self) -> bool {
        true
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let scatter_direction = hit.normal + random_unit_vec();
        let scattered = Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo.value(hit.uv, hit.p)))
    }
}

pub struct Metal {
    pub albedo: Arc<dyn Texture>,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(ray.direction().normalize(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_vec_in_unit_sphere());
        Some((scattered, self.albedo.value(hit.uv, hit.p)))
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
    pub color: Vector3<f32>,
    pub roughness: Arc<dyn Texture>,
    pub density: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let attenuation: Vector3<f32>;
        let etai_over_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let normal = (hit.normal + self.roughness.value(hit.uv, hit.p).x * random_vec_in_unit_sphere()).normalize();

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        if !hit.front_face {
            // Color
            let distance = (ray.origin() - hit.p).magnitude();
            attenuation = (-self.color.map(|x| 1.0/x) * self.density * distance).map(f32::exp);
        } else {
            attenuation = vec(1.0, 1.0, 1.0);
        }

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, normal);
            Ray::new(hit.p, reflected)
        } else {
            let reflect_prob = schlick(cos_theta, self.ref_idx);
            let mut rng = thread_rng();
            let refracted_or_reflected = if rng.gen::<f32>() < reflect_prob {
                reflect(unit_direction, normal)
            } else {                                
                refract(unit_direction, normal, etai_over_etat)
            };
            Ray::new(hit.p, refracted_or_reflected)
        };

        Some((scattered, attenuation))
    }
}

impl Default for Dielectric {
    fn default() -> Dielectric {
        Dielectric {
            ref_idx: 1.52,
            color: vec(1.0, 1.0, 1.0),
            roughness: Arc::new(ConstantTex { color: vec_zero() }),
            density: 0.0 //TODO: rename to absorption coefficient or something like that
        }
    }
}

pub struct DiffuseLight {
    pub emit: Arc<dyn Texture>
}

impl Material for DiffuseLight {
    fn scatter(&self, _ray: &Ray, _hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        None
    }

    fn emitted(&self, _ray: &Ray, hit: &HitRecord) -> Vector3<f32> {
        self.emit.value(hit.uv, hit.p)
    }
}


pub trait EnvironmentMaterial: Sync + Send {
    fn emit(&self, ray: &Ray) -> Vector3<f32>;
}

pub struct SimpleEnvironment {

}

impl EnvironmentMaterial for SimpleEnvironment {
    fn emit(&self, ray: &Ray) -> Vector3<f32> {
        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

pub struct Environment {
    pub emit: Arc<dyn Texture>
}

impl EnvironmentMaterial for Environment {
    fn emit(&self, ray: &Ray) -> Vector3<f32> {
        let uv = get_sphere_uv(ray.direction().normalize());
        self.emit.value(uv, ray.direction())
    }
}

pub struct Isotropic {
    pub albedo: Arc<dyn Texture>
}

impl Material for Isotropic {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        Some((
            Ray::new(hit.p, random_vec_in_unit_sphere()), 
            self.albedo.value(hit.uv, hit.p)
        ))
    }
    fn is_solid(&self) -> bool {
        false
    }
}

fn get_sphere_uv(p: Vector3<f32>) -> Vector2<f32> {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::PI / 2.0) / f32::consts::PI;
    Vector2::new(u, v)
}



pub struct DielectricSurfaceLambert {
    pub ref_idx: f32,
    pub albedo: Arc<dyn Texture>,
    pub roughness: Arc<dyn Texture>,
}

impl Material for DielectricSurfaceLambert {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let mut attenuation = vec_one();
        let etai_over_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let normal = (hit.normal + self.roughness.value(hit.uv, hit.p).x * random_vec_in_unit_sphere()).normalize();

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, normal);
            Ray::new(hit.p, reflected)
        } else {
            let reflect_prob = schlick(cos_theta, self.ref_idx);
            let mut rng = thread_rng();
            let refracted_or_reflected = if rng.gen::<f32>() < reflect_prob {
                reflect(unit_direction, normal)
            } else {                                
                // Instead of refracting we fo Lambertian               
                attenuation = self.albedo.value(hit.uv, hit.p);
                hit.normal + random_unit_vec()
            };
            Ray::new(hit.p, refracted_or_reflected)
        };

        Some((scattered, attenuation))
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
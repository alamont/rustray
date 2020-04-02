use nalgebra::Vector3;

use crate::hittable::{HitRecord};
use crate::ray::Ray;
use crate::vec::{random_unit_vec, random_vec_in_unit_sphere};

use rand::{thread_rng, Rng};


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
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let scatter_direction = hit.normal + random_unit_vec();
        let scattered = Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}

pub struct Metal {
    pub albedo: Vector3<f32>,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let reflected = reflect(ray.direction().normalize(), hit.normal);
        let scattered = Ray::new(hit.p, reflected + self.fuzz * random_vec_in_unit_sphere());
        Some((scattered, self.albedo))
    }
}

pub struct Dielectric {
    pub ref_idx: f32,
    pub reflection_color: Vector3<f32>,
    pub refraction_color: Vector3<f32>,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let attenuation: Vector3<f32>;
        let etai_over_etat = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        let scattered = if etai_over_etat * sin_theta > 1.0 {
            let reflected = reflect(unit_direction, hit.normal);
            attenuation = self.reflection_color;
            Ray::new(hit.p, reflected)
        } else {
            let reflect_prob = schlick(cos_theta, self.ref_idx);
            let mut rng = thread_rng();
            let refracted_or_reflected = if rng.gen::<f32>() < reflect_prob {
                attenuation = self.reflection_color;
                reflect(unit_direction, hit.normal)
            } else {
                attenuation = self.refraction_color;
                refract(unit_direction, hit.normal, etai_over_etat)
            };
            Ray::new(hit.p, refracted_or_reflected)
        };

        Some((scattered, attenuation))
    }
}
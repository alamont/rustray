use nalgebra::Vector3;

use crate::hitable::{HitRecord};
use crate::ray::Ray;
use crate::vec::random_unit_vec;

pub trait Material: Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)>;
}

pub struct Lambertian {
    pub albedo: Vector3<f32>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vector3<f32>)> {
        let scatter_direction = hit.normal + random_unit_vec();
        let scattered = Ray::new(hit.p, scatter_direction);
        Some((scattered, self.albedo))
    }
}
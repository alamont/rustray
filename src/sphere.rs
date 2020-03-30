use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::material::Material;
use nalgebra::Vector3;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();        
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root)/a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let outward_normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(temp, p, outward_normal, ray, &self.material));
            }
            temp = (-half_b + root)/a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let outward_normal = (p - self.center) / self.radius;
                return Some(HitRecord::new(temp, p, outward_normal, ray, &self.material));
            }
        }
        None
    }
}

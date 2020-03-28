use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use nalgebra::Vector3;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().dot(&ray.direction());
        let b = oc.dot(&ray.direction());
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b * a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t: temp, p, normal });
            }
            temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let normal = (p - self.center) / self.radius;
                return Some(HitRecord { t: temp, p, normal });
            }
        }
        None
    }
}

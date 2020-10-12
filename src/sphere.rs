use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::vec;

use nalgebra::{Vector2, Vector3};
use std::sync::Arc;
use std::f32;

pub struct Sphere {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32, material: Box<dyn Material>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().magnitude_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.magnitude_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);
                let outward_normal = (p - self.center) / self.radius;
                let uv = get_sphere_uv(outward_normal);
                return Some(HitRecord::new(
                    temp,
                    p,
                    outward_normal,
                    ray,
                    &self.material,
                    uv
                ));
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                let p = ray.point_at_parameter(temp);                
                let outward_normal = (p - self.center) / self.radius;
                let uv = get_sphere_uv(outward_normal);
                return Some(HitRecord::new(
                    temp,
                    p,
                    outward_normal,
                    ray,
                    &self.material,
                    uv
                ));
            }
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB {
            min: self.center - vec(self.radius, self.radius, self.radius),
            max: self.center + vec(self.radius, self.radius, self.radius),
        })
    }
}

fn get_sphere_uv(p: Vector3<f32>) -> Vector2<f32> {
    let phi = p.z.atan2(p.x);
    let theta = p.y.asin();
    let u = 1.0 - (phi + f32::consts::PI) / (2.0 * f32::consts::PI);
    let v = (theta + f32::consts::PI / 2.0) / f32::consts::PI;
    Vector2::new(u, v)
}

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::vec3;

use nalgebra::{Vector2, Vector3};
use std::sync::Arc;
use std::f32;

const EPSILON:f32 = 0.0000001;

pub struct Triangle<'a> {
    pub v0: Vector3<f32>,
    pub v1: Vector3<f32>,
    pub v2: Vector3<f32>,
    pub material: &'a Box<dyn Material>,
    pub n0: Vector3<f32>,
    pub n1: Vector3<f32>,
    pub n2: Vector3<f32>
}

impl<'a> Triangle<'a> {
    pub fn new(v0: Vector3<f32>, v1: Vector3<f32>, v2: Vector3<f32>, material: &'a Box<dyn Material>) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = edge1.cross(&edge2);
        Self {
            v0,
            v1,
            v2,
            material,
            n0: normal,
            n1: normal,
            n2: normal,
        }
    }
}

impl<'a> Hittable for Triangle<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;

        let h = ray.direction().cross(&edge2);
        let a = edge1.dot(&h);

        if a.abs() < EPSILON {
            return None; // Parallel to triangle
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.v0;
        let u = f * s.dot(&h);
       
        if u < 0.0 || u > 1.0 {
            return None;
        }
        
        let q = s.cross(&edge1);
        let v = f * ray.direction().dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t = f * edge2.dot(&q);
        if t < t_max && t > t_min { // ray intersection
            let p = ray.at(t);
            // let normal = edge2.cross(&edge1).normalize();
            // let a = 1.0 / (p - self.v0).magnitude();
            // let b = 1.0 / (p - self.v1).magnitude();
            // let c = 1.0 / (p - self.v2).magnitude();

            let normal = (u * self.n1 + v * self.n2 + (1.0 - u - v) * self.n0).normalize();
            
            return Some(HitRecord::new(
                t,
                p,
                normal,
                ray,
                &self.material,
                Vector2::new(u, v)
            ));
        }
        None
    }

    fn bounding_box(&self) -> Option<AABB> {
        let x = vec3(self.v0.x, self.v1.x, self.v2.x);
        let y = vec3(self.v0.y, self.v1.y, self.v2.y);
        let z = vec3(self.v0.z, self.v1.z, self.v2.z);
        let min = vec3(x.min(), y.min(), z.min());
        let max = vec3(x.max(), y.max(), z.max());
        Some (AABB { min, max })
    }
}

use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{vec, vec3};

use nalgebra::{Vector2, Vector3};
use std::f32;
use std::sync::Arc;

pub enum AARectType {
    XY,
    XZ,
    YZ,
}

pub struct AARect {
    pub xy0: Vector2<f32>,
    pub xy1: Vector2<f32>,
    pub k: f32,
    pub material: Arc<dyn Material>,
    pub rect_type: AARectType
}

impl Hittable for AARect {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        use AARectType::*;
        let t = match &self.rect_type {
            XY => (self.k - ray.origin().z) / ray.direction().z,
            XZ => (self.k - ray.origin().y) / ray.direction().y,
            YZ => (self.k - ray.origin().x) / ray.direction().x,
        };
        if t < t_min || t > t_max {
            return None;
        }
        let xy = match &self.rect_type {
            XY => ray.origin().xy() + t * ray.direction().xy(),
            XZ => ray.origin().xz() + t * ray.direction().xz(),
            YZ => ray.origin().yz() + t * ray.direction().yz(),
        };
        if xy.x < self.xy0.x || xy.x > self.xy1.x || xy.y < self.xy0.y || xy.y > self.xy1.y {
            return None;
        }
        let uv = (xy - self.xy0).component_div(&(self.xy1 - self.xy0));
        let p = ray.at(t);
        let outward_normal = match &self.rect_type {
            XY => vec(0.0, 0.0, 1.0),
            XZ => vec(0.0, -1.0, 0.0),
            YZ => vec(1.0, 0.0, 0.0),
        };
        Some(HitRecord::new(
            t,
            p,
            outward_normal,
            ray,
            Arc::clone(&self.material),
            uv
        ))
    }

    fn bounding_box(&self) -> Option<AABB> {
        use AARectType::*;
        let min = vec(self.xy0.x, self.xy0.y, self.k - 0.0001);
        let max = vec(self.xy1.x, self.xy1.y, self.k + 0.0001);
        match &self.rect_type {
            XY => Some(AABB { min, max }),
            XZ => Some(AABB { min: min.xzy(), max: max.xzy() }),
            YZ => Some(AABB { min: min.zxy(), max: max.zxy() }),
        }
    }
}
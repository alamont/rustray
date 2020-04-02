
use std::f32;

use nalgebra::Vector3;

use crate::ray::Ray;
use crate::vec::{vec, vec_zero, vec_one};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        let inv_d = vec_one().component_div(&ray.direction());
        let t0 = (self.min - ray.origin()).component_mul(&inv_d);
        let t1 = (self.max - ray.origin()).component_mul(&inv_d);

        let t_small = t0.zip_map(&t1, |a, b| a.min(b));
        let t_big = t0.zip_map(&t1, |a, b| a.max(b));
        
        t_small.max() <= t_big.min()
    }

    pub fn zero() -> Self {
        AABB {
            min: vec_zero(),
            max: vec_zero()
        }
    }
}

pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
    let min = vec(
        box0.min.x.min(box1.min.x),
        box0.min.y.min(box1.min.y),
        box0.min.z.min(box1.min.z),
    );
    let max = vec(
        box0.max.x.max(box1.max.x),
        box0.max.y.max(box1.max.y),
        box0.max.z.max(box1.max.z),
    );
    AABB { min, max }
}
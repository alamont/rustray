
use std::f32;

use nalgebra::Vector3;

use crate::ray::Ray;
use crate::vec::{vec, fmin, fmax, vec_zero};

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {

        let inv_d = ray.direction().map(|x| 1.0 / x);
        let t0 = (self.min - ray.origin()).component_mul(&inv_d);
        let t1 = (self.max - ray.origin()).component_mul(&inv_d);
        let (t0, t1) = (
            inv_d.map(|i| if i < 0.0 { t1 } else { t0 }),
            inv_d.map(|i| if i < 0.0 { t0 } else { t1 })            
        );

        let start = t_min.max(t0.map(|x| x.max()).max());
        let end = t_max.min(t1.map(|x| x.min()).min());
        end > start
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
        fmin(box0.min.x, box1.min.x),
        fmin(box0.min.y, box1.min.y),
        fmin(box0.min.z, box1.min.z),
    );
    let max = vec(
        fmax(box0.max.x, box1.max.x),
        fmax(box0.max.y, box1.max.y),
        fmax(box0.max.z, box1.max.z),
    );
    AABB { min, max }
}
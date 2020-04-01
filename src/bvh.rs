use rand::{thread_rng, Rng};

use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::vec_zero;

pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn build(mut objects: Vec<Box<dyn Hittable>>) -> Box<dyn Hittable> {
        
        let mut rng = thread_rng();
        let axis: usize = rng.gen_range(0, 3);

        // let mut objects_with_bb: Vec<Box<dyn Hittable>> = objects.drain_filter(|a| a.bounding_box().is_some()).collect();
        let mut objects_with_bb = objects;

        objects_with_bb.sort_by(|a, b| {
            let left_hit = if let Some(bb) = a.bounding_box() {
                bb.min
            } else { vec_zero() };
            let right_hit = if let Some(bb) = b.bounding_box() {
                bb.min
            } else { vec_zero() };
            left_hit[axis].partial_cmp(&right_hit[axis]).unwrap()
        });

        match objects_with_bb.len() {
            0 => panic!("length mismatch"),
            1 => {
                objects_with_bb.remove(0)
                // Box::new(BVHNode { left, right: left, left.bounding_box() })
            }
            2 => {
                let left = objects_with_bb.remove(1);
                let right = objects_with_bb.remove(0);
                let left_bbox = if let Some(bb) = left.bounding_box() { bb } else { AABB::zero() };
                let right_bbox = if let Some(bb) = right.bounding_box() { bb } else { AABB::zero() };
                let bbox = surrounding_box(left_bbox, right_bbox);
                Box::new(BVHNode { left, right, bbox })
            }
            _ => {
                let mut a = objects_with_bb;
                let b = a.split_off(a.len() / 2);
                let left = Self::build(b);
                let right = Self::build(a);
                let left_bbox = if let Some(bb) = left.bounding_box() { bb } else { AABB::zero() };
                let right_bbox = if let Some(bb) = right.bounding_box() { bb } else { AABB::zero() };
                let bbox = surrounding_box(left_bbox, right_bbox);
                Box::new(BVHNode { left, right, bbox })
            }
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            let left_hit = self.left.hit(ray, t_min, t_max);
            let right_hit = self.right.hit(ray, t_min, t_max);
            match (left_hit, right_hit) {
                (None, None) => None,
                (None, Some(hit_rec)) => Some(hit_rec),
                (Some(hit_rec), None) => Some(hit_rec),
                (Some(left_hit), Some(right_hit)) => {
                    if left_hit.t < right_hit.t {
                        Some(left_hit)
                    } else {
                        Some(right_hit)
                    }
                }
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}

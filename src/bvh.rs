use nalgebra::Vector3;
use std::{sync::Arc, f32};

use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::ray::Ray;

const MAX_LEAF: usize = 2;

pub struct BVHNode<'a> {
    left: &'a Box<dyn Hittable>,
    right: &'a Box<dyn Hittable>,
    bbox: AABB,
}

#[allow(unreachable_patterns)]
impl<'a> BVHNode<'a> {
    pub fn build(mut objects: Vec<Box<dyn Hittable>>, depth: u32) ->Box<dyn Hittable> {
        fn axis_range(objects: &Vec<Box<dyn Hittable>>, axis: usize) -> f32 {
            let range = objects.iter().fold(f32::MAX..f32::MIN, |range, obj| {
                let bb = obj.bounding_box().unwrap();
                let min = bb.min[axis].min(bb.max[axis]);
                let max = bb.min[axis].max(bb.max[axis]);
                range.start.min(min)..range.end.max(max)
            });
            // println!("range: {}..{}", range.start, range.end);
            (range.end - range.start).abs()
        }

        let axis = Vector3::new(
            axis_range(&objects, 0),
            axis_range(&objects, 1),
            axis_range(&objects, 2),
        )
        .imax();

        // sort_objects(&mut objects, axis as usize);
        objects.sort_unstable_by(|a, b| {
            let left_bb = a.bounding_box().unwrap();
            let right_bb = b.bounding_box().unwrap();
            let left_hit = left_bb.min[axis] + left_bb.max[axis];
            let right_hit = right_bb.min[axis] + right_bb.max[axis];
            left_hit.partial_cmp(&right_hit).unwrap()
        });

        // println!("axis: {}, len: {}, depth: {}", axis, objects.len(), depth);

        match objects.len() {
            0 => panic!("length mismatch"),
            1 => {
                objects.remove(0)
            }
            2 => {
                let left = objects.remove(1);
                let right = objects.remove(0);
                let left_bbox = if let Some(bb) = left.bounding_box() { bb } else { AABB::zero() };
                let right_bbox = if let Some(bb) = right.bounding_box() { bb } else { AABB::zero() };
                let bbox = surrounding_box(left_bbox, right_bbox);
                Box::new(BVHNode { left: &left, right: &right, bbox })
            }
            2..=MAX_LEAF => {
                Box::new(HittableList { objects })
            }
            _ => {
                let mut a = objects;
                let b = a.split_off(a.len() / 2);
                let left = Self::build(b, depth + 1);
                let right = Self::build(a, depth + 1);
                let left_bbox = if let Some(bb) = left.bounding_box() {
                    bb
                } else {
                    AABB::zero()
                };
                let right_bbox = if let Some(bb) = right.bounding_box() {
                    bb
                } else {
                    AABB::zero()
                };
                let bbox = surrounding_box(left_bbox, right_bbox);
                Box::new(BVHNode { left: &left, right: &right, bbox })
            }
        }
    }
}

impl<'a> Hittable for BVHNode<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(ray, t_min, t_max) {
            let left_hit = self.left.hit(ray, t_min, t_max);

            if let Some(h) = &left_hit {
                t_max = h.t;
            }

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

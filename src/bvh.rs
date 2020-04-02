use rand::{thread_rng, Rng};
use std::f32;
use std::sync::Arc;
use std::ops::Range;
use nalgebra::Vector3;

use crate::aabb::{surrounding_box, AABB};
use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::vec_zero;

pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bbox: AABB
}

impl BVHNode {
    pub fn build(mut objects: Vec<Box<dyn Hittable>>, depth: u32) -> Box<dyn Hittable> {
        
        // let mut rng = thread_rng();
        // let axis: usize = rng.gen_range(0, 3);

        // // let mut objects_with_bb: Vec<Box<dyn Hittable>> = objects.drain_filter(|a| a.bounding_box().is_some()).collect();
        // let mut objects_with_bb = objects;

        // objects.sort_by(|a, b| {
        //     let left_hit = if let Some(bb) = a.bounding_box() {
        //         bb.min
        //     } else { vec_zero() };
        //     let right_hit = if let Some(bb) = b.bounding_box() {
        //         bb.min
        //     } else { vec_zero() };
        //     left_hit[axis].partial_cmp(&right_hit[axis]).unwrap()
        // });

        // fn sort_objects(objects: &mut Vec<Box<dyn Hittable>>, axis: usize) {
        //     objects.sort_unstable_by(|a, b| {
        //         let left_bb = a.bounding_box().unwrap();
        //         let right_bb = a.bounding_box().unwrap();
        //         let left_hit = left_bb.min[axis] + left_bb.max[axis];
        //         let right_hit = right_bb.min[axis] + right_bb.max[axis];
        //         left_hit.partial_cmp(&right_hit).unwrap()
        //     });
        // }

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
        ).imax();

        // sort_objects(&mut objects, axis as usize);
        objects.sort_unstable_by(|a, b| {
            let left_bb = a.bounding_box().unwrap();
            let right_bb = a.bounding_box().unwrap();
            let left_hit = left_bb.min[axis] + left_bb.max[axis];
            let right_hit = right_bb.min[axis] + right_bb.max[axis];
            left_hit.partial_cmp(&right_hit).unwrap()
        });       

        println!("axis: {}, len: {}, depth: {}", axis, objects.len(), depth);

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
                Box::new(BVHNode { left, right, bbox })
            }
            _ => {
                let mut a = objects;                
                let b = a.split_off(a.len() / 2);
                let left = Self::build(b, depth+1);
                let right = Self::build(a, depth+1);
                let left_bbox = if let Some(bb) = left.bounding_box() { bb } else { AABB::zero() };
                let right_bbox = if let Some(bb) = right.bounding_box() { bb } else { AABB::zero() };
                let bbox = surrounding_box(left_bbox, right_bbox);
                Box::new(BVHNode { left, right, bbox })
            }
        }
    }
}

impl Hittable for BVHNode {
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

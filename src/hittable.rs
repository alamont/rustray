use nalgebra::{Vector2, Vector3};
use std::sync::Arc;

use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{vec, vec_zero};

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
    pub uv: Vector2<f32>
}

impl HitRecord {
    pub fn new(
        t: f32,
        p: Vector3<f32>,
        outward_normal: Vector3<f32>,
        ray: &Ray,
        material: Arc<dyn Material>,
        uv: Vector2<f32>,
    ) -> HitRecord {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            t,
            p,
            front_face,
            normal,
            material: material,
            uv
        }
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable));
    }

    pub fn push_without_box(&mut self, hittable: Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_closest: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for hittable_obj in self.objects.iter() {
            if let Some(hit) = hittable_obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_closest = Some(hit);
            }
        }
        return hit_closest;
    }
    fn bounding_box(&self) -> Option<AABB> {
        if !self.objects.is_empty() {
            if let Some(mut output_box) = self.objects[0].bounding_box() {
                for object in &self.objects[1..] {
                    if let Some(bb) = object.bounding_box() {
                        output_box = surrounding_box(output_box, bb);
                    }
                }
                Some(output_box)
            } else {
                None
            }
        } else {
            None
        }
    }
}

use nalgebra::{Vector2, Vector3, Rotation3};
use std::{sync::Arc, f32};

use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{vec, vec_zero, deg_to_rad};

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

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3<f32>) {
        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;

    fn bounding_box(&self) -> Option<AABB>;
}

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Arc::new(hittable));
    }

    pub fn push_without_box(&mut self, hittable: Arc<dyn Hittable>) {
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

pub struct FlipFace {
    pub object: Arc<dyn Hittable>
}

impl FlipFace {
    pub fn new(obj: impl Hittable + 'static) -> Self {
        Self {
            object:Arc::new(obj)
        }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if let Some(mut hit_rec) = self.object.hit(ray, t_min, t_max) {
            hit_rec.front_face = !hit_rec.front_face;
            Some(hit_rec)
        } else {
            None
        }
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.object.bounding_box()
    }
}

pub struct Transform {
    pub object: Arc<dyn Hittable>,
    pub offset: Vector3<f32>,
    pub rotation: Rotation3<f32>,
    pub bbox: AABB
}

impl Transform {
    pub fn new(obj: impl Hittable + 'static, offset: Vector3<f32>, rotation_deg: Vector3<f32>) -> Self {
        let rotation = Rotation3::from_euler_angles(deg_to_rad(rotation_deg.x), deg_to_rad(rotation_deg.y), deg_to_rad(rotation_deg.z));
        let bb_min_rot = rotation * obj.bounding_box().unwrap().min + offset;
        let bb_max_rot = rotation * obj.bounding_box().unwrap().max + offset;
        
        let bb_min = Vector3::new(
            bb_min_rot.x.min(bb_max_rot.x),
            bb_min_rot.y.min(bb_max_rot.y),
            bb_min_rot.z.min(bb_max_rot.z)
        );

        let bb_max = Vector3::new(
            bb_min_rot.x.max(bb_max_rot.x),
            bb_min_rot.y.max(bb_max_rot.y),
            bb_min_rot.z.max(bb_max_rot.z)
        );

        Self {
            object:Arc::new(obj),
            offset,
            rotation,
            bbox: AABB {
                min: bb_min,
                max: bb_max
            }
        }
    }

    pub fn new_b(obj: Arc<dyn Hittable>, offset: Vector3<f32>, rotation_deg: Vector3<f32>) -> Self {
        let rotation = Rotation3::from_euler_angles(deg_to_rad(rotation_deg.x), deg_to_rad(rotation_deg.y), deg_to_rad(rotation_deg.z));
        let bb_min_rot = rotation * obj.bounding_box().unwrap().min + offset;
        let bb_max_rot = rotation * obj.bounding_box().unwrap().max + offset;
        
        let bb_min = Vector3::new(
            bb_min_rot.x.min(bb_max_rot.x),
            bb_min_rot.y.min(bb_max_rot.y),
            bb_min_rot.z.min(bb_max_rot.z)
        );

        let bb_max = Vector3::new(
            bb_min_rot.x.max(bb_max_rot.x),
            bb_min_rot.y.max(bb_max_rot.y),
            bb_min_rot.z.max(bb_max_rot.z)
        );

        Self {
            object: obj,
            offset,
            rotation,
            bbox: AABB {
                min: bb_min,
                max: bb_max
            }
        }
    }
}

impl Hittable for Transform {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let inv_rot = self.rotation.inverse();
        let mut moved_ray = Ray::new(inv_rot * (ray.origin() - self.offset), inv_rot * ray.direction());
        moved_ray.albedo_normal_ray = ray.albedo_normal_ray;

        if let Some(mut hit_rec) = self.object.hit(&moved_ray, t_min, t_max) {
            hit_rec.p = self.rotation * hit_rec.p + self.offset;
            hit_rec.normal = self.rotation * hit_rec.normal;            
            Some(hit_rec)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Option<AABB> {
        Some(self.bbox)
    }
}
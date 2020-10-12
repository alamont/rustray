use nalgebra::{Vector2, Vector3, Rotation3};
use std::{sync::Arc, f32};

use crate::aabb::{surrounding_box, AABB};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{vec, vec_zero, deg_to_rad, vec3};

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>,
    pub uv: Vector2<f32>
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f32,
        p: Vector3<f32>,
        outward_normal: Vector3<f32>,
        ray: &Ray,
        material: &'a Box<dyn Material>,
        uv: Vector2<f32>,
    ) -> HitRecord<'a> {
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
    fn pdf_value(&self, _origin: &Vector3<f32>, _direction: &Vector3<f32>) -> f32 {
        0.0
    }
    fn random(&self, _origin: &Vector3<f32>) -> Vector3<f32> {
        vec3(1.0, 0.0, 0.0)
    }
}

#[derive(Default)]
pub struct HittableList<'a> {
    pub objects: Vec<&'a Box<dyn Hittable>>,
}

impl<'a> HittableList<'a> {
    // pub fn push(&mut self, hittable: impl Hittable) {
    //     self.objects.push(&'a Box::new(hittable));
    // }

    pub fn push(&mut self, hittable: &'a Box<dyn Hittable>) {
        self.objects.push(hittable);
    }
}

impl<'a> Hittable for HittableList<'a> {
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

pub struct FlipFace<'a> {
    pub object: &'a Box<dyn Hittable>
}

impl<'a> FlipFace<'a> {
    pub fn new(object: &'a Box<dyn Hittable>) -> Self {
        Self {
            object
        }
    }
    pub fn boxed(self) -> Box<Self> {
        Box::from(self)
    }
}

impl<'a> Hittable for FlipFace<'a> {
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

pub struct Transform<'a> {
    pub object: &'a Box<dyn Hittable>,
    pub offset: Vector3<f32>,
    pub rotation: Rotation3<f32>,
    pub bbox: AABB
}

impl<'a> Transform<'a> {
    pub fn new(object: &'a Box<dyn Hittable>, offset: Vector3<f32>, rotation_deg: Vector3<f32>) -> Self {
        let rotation = Rotation3::from_euler_angles(deg_to_rad(rotation_deg.x), deg_to_rad(rotation_deg.y), deg_to_rad(rotation_deg.z));
        let bb_min_rot = rotation * object.bounding_box().unwrap().min + offset;
        let bb_max_rot = rotation * object.bounding_box().unwrap().max + offset;
        
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
            object,
            offset,
            rotation,
            bbox: AABB {
                min: bb_min,
                max: bb_max
            }
        }
    }

    // pub fn new_b(obj: Box<dyn Hittable>, offset: Vector3<f32>, rotation_deg: Vector3<f32>) -> Self {
    //     let rotation = Rotation3::from_euler_angles(deg_to_rad(rotation_deg.x), deg_to_rad(rotation_deg.y), deg_to_rad(rotation_deg.z));
    //     let bb_min_rot = rotation * obj.bounding_box().unwrap().min + offset;
    //     let bb_max_rot = rotation * obj.bounding_box().unwrap().max + offset;
        
    //     let bb_min = Vector3::new(
    //         bb_min_rot.x.min(bb_max_rot.x),
    //         bb_min_rot.y.min(bb_max_rot.y),
    //         bb_min_rot.z.min(bb_max_rot.z)
    //     );

    //     let bb_max = Vector3::new(
    //         bb_min_rot.x.max(bb_max_rot.x),
    //         bb_min_rot.y.max(bb_max_rot.y),
    //         bb_min_rot.z.max(bb_max_rot.z)
    //     );

    //     Self {
    //         object: obj,
    //         offset,
    //         rotation,
    //         bbox: AABB {
    //             min: bb_min,
    //             max: bb_max
    //         }
    //     }
    // }
}

impl<'a> Hittable for Transform<'a> {
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
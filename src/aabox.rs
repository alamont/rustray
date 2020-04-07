use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList, FlipFace};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{vec3};
use crate::aarect::{AARect, AARectType::*};

use nalgebra::{Vector2, Vector3};
use std::f32;
use std::sync::Arc;

pub struct AABox {
    pub box_min: Vector3<f32>,
    pub box_max: Vector3<f32>,
    pub sides: HittableList,
}

impl AABox {
    // pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, material: Arc<dyn Material>) -> Self{
    pub fn new(scale: Vector3<f32>, material: Arc<dyn Material>) -> Self{
        let p = vec3(0.0, 0.0, 0.0);
        let half_scale = scale / 2.0;

        let box_min = p - half_scale;
        let box_max = p + half_scale;

        let mut sides = HittableList::default();
        
        sides.push(AARect { 
            xy0: p.xy() - half_scale.xy(),
            xy1: p.xy() + half_scale.xy(),
            k: p.z + half_scale.z,
            material: material.clone(),
            rect_type: XY
        });
        sides.push(FlipFace::new(AARect { 
            xy0: p.xy() - half_scale.xy(),
            xy1: p.xy() + half_scale.xy(),
            k: p.z - half_scale.z,
            material: material.clone(),
            rect_type: XY
        }));
        sides.push(AARect { 
            xy0: p.xz() - half_scale.xz(),
            xy1: p.xz() + half_scale.xz(),
            k: p.y + half_scale.y,
            material: material.clone(),
            rect_type: XZ
        });
        sides.push(FlipFace::new(AARect { 
            xy0: p.xz() - half_scale.xz(),
            xy1: p.xz() + half_scale.xz(),
            k: p.y - half_scale.y,
            material: material.clone(),
            rect_type: XZ
        }));
        sides.push(AARect { 
            xy0: p.yz() - half_scale.yz(),
            xy1: p.yz() + half_scale.yz(),
            k: p.x + half_scale.x,
            material: material.clone(),
            rect_type: YZ
        });
        sides.push(FlipFace::new(AARect { 
            xy0: p.yz() - half_scale.yz(),
            xy1: p.yz() + half_scale.yz(),
            k: p.x - half_scale.x,
            material: material.clone(),
            rect_type: YZ
        }));

        Self {
            box_min,
            box_max,
            sides
        }
    }
}

impl Hittable for AABox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.sides.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<AABB> {
        Some(AABB { min: self.box_min, max: self.box_max })
    }
}
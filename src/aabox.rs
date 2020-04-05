use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList, FlipFace};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::vec;
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
    pub fn new(p0: Vector3<f32>, p1: Vector3<f32>, material: Arc<dyn Material>) -> Self{
        let box_min = p0;
        let box_max = p1;

        let mut sides = HittableList::default();
        
        sides.push(AARect { 
            xy0: p0.xy(),
            xy1: p1.xy(),
            k: p1.z,
            material: material.clone(),
            rect_type: XY
        });
        sides.push(FlipFace::new(AARect { 
            xy0: p0.xy(),
            xy1: p1.xy(),
            k: p0.z,
            material: material.clone(),
            rect_type: XY
        }));
        sides.push(AARect { 
            xy0: p0.xz(),
            xy1: p1.xz(),
            k: p1.y,
            material: material.clone(),
            rect_type: XZ
        });
        sides.push(FlipFace::new(AARect { 
            xy0: p0.xz(),
            xy1: p1.xz(),
            k: p0.y,
            material: material.clone(),
            rect_type: XZ
        }));
        sides.push(AARect { 
            xy0: p0.yz(),
            xy1: p1.yz(),
            k: p1.x,
            material: material.clone(),
            rect_type: YZ
        });
        sides.push(FlipFace::new(AARect { 
            xy0: p0.yz(),
            xy1: p1.yz(),
            k: p1.y,
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
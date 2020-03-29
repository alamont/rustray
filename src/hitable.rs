use nalgebra::Vector3;

use crate::ray::Ray;
use crate::material::Material;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub front_face: bool,
    pub material: &'a Box<dyn Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f32, p:Vector3<f32>, outward_normal: Vector3<f32>, ray: &Ray, material: &'a Box<dyn Material>) -> HitRecord<'a> {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face {outward_normal} else {-outward_normal};
        HitRecord { t, p, front_face, normal, material: material }
    }
}

pub trait Hitable: Sync{
    fn hit(&self, ray:&Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<dyn Hitable>>
}

impl HitableList {
    pub fn push(&mut self, hitable: impl Hitable + 'static) {
        self.list.push(Box::new(hitable));
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_closest: Option<HitRecord> = None; 
        let mut closest_so_far = t_max;
        for hitable_obj in self.list.iter() {
            if let Some(hit) = hitable_obj.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_closest = Some(hit);
            }         
        }
        return hit_closest;
    }
}
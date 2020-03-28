use crate::ray::Ray;
use nalgebra::Vector3;

pub struct HitRecord {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>
}

pub trait Hitable:Sync {
    fn hit(&self, ray:&Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HitableList {
    list: Vec<Box<Hitable>>
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
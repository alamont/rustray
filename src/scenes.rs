// pub mod simple_scene;
// pub mod random_scene_bvh;
// pub mod random_scene_no_bvh;
// pub mod random_scene;
pub mod dielectric_scene;
pub mod earth_scene;
pub mod random_scene;

use std::sync::Arc;
use crate::hittable::Hittable;
use crate::camera::Camera;
use crate::material::EnvironmentMaterial;

pub struct Scene {
    pub objects: Box<dyn Hittable>,
    pub environment: Arc<dyn EnvironmentMaterial>,
    pub camera: Camera
}
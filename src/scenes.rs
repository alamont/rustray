// pub mod simple_scene;
// pub mod random_scene_bvh;
// pub mod random_scene_no_bvh;
// pub mod random_scene;
// pub mod dielectric_scene;

// pub mod earth_scene;
// pub mod random_scene;
// pub mod random_scene_light;
// pub mod cornell_box_scene;
// pub mod cornell_box_vol;
// pub mod cornell_box_mesh;
// pub mod cornell_box_texture_filtering;
// pub mod env_scene;
pub mod cornell_box_scene;

pub mod prefabs;


use std::sync::Arc;
use crate::hittable::Hittable;
use crate::camera::Camera;
use crate::material::EnvironmentMaterial;

pub struct Scene {
    pub objects:Arc<dyn Hittable>,
    pub environment: Arc<dyn EnvironmentMaterial>,
    pub camera: Camera,
    pub lights: Arc<dyn Hittable>,
}
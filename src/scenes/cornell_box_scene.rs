use nalgebra::{Vector2, Vector3};
use std::sync::Arc;

use crate::hittable::{HittableList, Hittable, FlipFace, Transform};
use crate::camera::Camera;
use crate::material::{Lambertian, Metal, Environment, DiffuseLight};
use crate::aarect::{AARect, AARectType::*};
use crate::aabox::AABox;
use crate::vec::{vec, vec2, vec3, vec_one, random_vec, random_vec_range, vec_zero};
use crate::bvh::BVHNode;
use crate::texture::{ConstantTex, CheckerTex, ImageTexture};
use crate::scenes::Scene;
use crate::scenes::prefabs::cornell_box::{cornell_box, cornell_box_camera, cornell_box_environment};


pub fn scene() -> Scene {
    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();
    let (objs, lights) = cornell_box();
    objects.push(objs);

    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let aluminium  = Arc::new(Metal { albedo: Arc::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.0});

    let box1 = AABox::new(
        vec(165.0, 165.0, 165.0),
        white.clone()
    );
    let box2 = AABox::new(
        vec(165.0, 330.0, 165.0),
        aluminium.clone()
    );

    objects.push(Arc::new(Transform::new(
        box1,
        vec3(85.0, 165.0/2.0, 120.0),
        vec3(0.0, -18.0, 0.0)
    )));

    objects.push(Arc::new(Transform::new(
        box2,
        vec3(-85.0, 165.0, -120.0),
        vec3(0.0, 15.0, 0.0)
    )));

    Scene {
        camera: cornell_box_camera(),
        objects: BVHNode::build(objects, 0),
        environment: cornell_box_environment(),
        lights: lights
    }
}
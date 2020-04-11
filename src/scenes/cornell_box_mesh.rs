use nalgebra::Vector3;
use std::sync::Arc;

use crate::hittable::{Hittable, FlipFace, Transform};
use crate::camera::Camera;
use crate::material::{
    Dielectric, 
    Lambertian, 
    Metal, 
    Environment, 
    DiffuseLight,
    Isotropic
};
use crate::volume::{ConstantMedium, NonUniformMedium};
use crate::aarect::{AARect, AARectType::*};
use crate::vec::{vec, vec2, vec3, vec_one, random_vec, random_vec_range, vec_zero};
use crate::bvh::BVHNode;
use crate::texture::{ConstantTex, CheckerTex, ImageTexture};
use crate::scenes::Scene;
use crate::sphere::Sphere;
use crate::mesh::Mesh;
use crate::scenes::prefabs::cornell_box::{cornell_box, cornell_box_camera, cornell_box_environment};

pub fn cornell_box_mesh() -> Scene {

    let aluminium  = Arc::new(Metal { albedo: Arc::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.25});
    let glass = Arc::new(Dielectric {
        color: vec(1.0, 1.0, 1.0),
        ..Dielectric::default()
    });


    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(cornell_box());

    let mesh = Mesh::new(String::from("assets/teapot2.obj"), aluminium, Vector3::new(10.0, 10.0, 10.0));
    objects.push(Box::new(
        Transform::new(
            mesh,
            Vector3::new(0.0, 100.0, 0.0),
            Vector3::new(-90.0, 0.0, 0.0)
        ))
    );

    objects.push(Box::new(Transform::new(
        Sphere::new(vec_zero(), 50.0, glass.clone()),
        vec3(100.0, 50.0, 100.0),
        vec_zero(),
    )));
   

    Scene {
        camera: cornell_box_camera(),
        objects: BVHNode::build(objects, 0),
        environment: cornell_box_environment()
    }
}
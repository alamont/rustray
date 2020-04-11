use nalgebra::{Vector2, Vector3};
use std::sync::Arc;
use rand::{thread_rng, Rng};

use crate::hittable::{HittableList, Hittable, FlipFace, Transform};
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
use crate::aabox::AABox;
use crate::vec::{vec, vec2, vec3, vec_one, random_vec, random_vec_range, vec_zero};
use crate::bvh::BVHNode;
use crate::texture::{ConstantTex, CheckerTex, ImageTexture};
use crate::scenes::Scene;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use crate::mesh::Mesh;

pub fn cornell_box_mesh(aspect: f32) -> Scene {

    let red = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.65, 0.05, 0.05) })});
    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let green = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.12, 0.45, 0.15) })});
    // let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(14.0, 14.0, 14.0) })});
    let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(7.0, 7.0, 7.0) })});
    let aluminium  = Arc::new(Metal { albedo: Arc::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.25});
    let glass = Arc::new(Dielectric {
        color: vec(1.0, 1.0, 1.0),
        ..Dielectric::default()
    });


    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, -278.0), 
        xy1: vec2(555.0, 278.0),
        k: -278.0,
        material: green.clone(),
        rect_type: YZ
    })));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, -278.0), 
        xy1: vec2(555.0, 278.0),
        k: 278.0,
        material: red.clone(),
        rect_type: YZ
    }));
    objects.push(Box::new(AARect { 
        xy0: vec2(-278.0, -278.0), 
        xy1: vec2(278.0, 278.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(-113.0, -113.0), 
        xy1: vec2(113.0, 113.0),
        k: 554.0,
        material: light.clone(),
        rect_type: XZ
    })));
    objects.push(Box::new(AARect { 
        xy0: vec2(-278.0, -278.0), 
        xy1: vec2(278.0, 278.0),
        k: 0.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(-278.0, 0.0), 
        xy1: vec2(278.0, 555.0),
        k: -278.0,
        material: white.clone(),
        rect_type: XY
    })));

    let mesh = Mesh::new(String::from("assets/teapot2.obj"), aluminium, Vector3::new(10.0, 10.0, 10.0));
    objects.push(Box::new(
        Transform::new(
            mesh,
            Vector3::new(0.0, 100.0, 0.0),
            Vector3::new(-90.0, 0.0, 0.0)
        ))
    );

    // let box1 = Transform::new(
    //     AABox::new(
    //         vec3(165.0, 165.0, 165.0), 
    //         white.clone()
    //     ),
    //     Vector3::new(0.0, 200.0, 0.0),
    //     vec3(0.0, 0.0, 0.0)
    // );
    // objects.push(Box::new(box1));

    objects.push(Box::new(Transform::new(
        Sphere::new(vec_zero(), 50.0, glass.clone()),
        vec3(100.0, 50.0, 100.0),
        vec_zero(),
    )));
   

    let lookfrom = vec3(0.0, 278.0, 1078.0);
    let lookat = vec3(0.0, 278.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;
    let vfov = 40.0;

    Scene {
        camera: Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus),
        objects: BVHNode::build(objects, 0),
        environment: Arc::new(Environment { emit: Arc::new(ConstantTex { color: vec_zero() })})
    }
}
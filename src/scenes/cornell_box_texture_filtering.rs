use nalgebra::Vector3;
use std::sync::Arc;

use crate::aarect::{AARect, AARectType::*};
use crate::bvh::BVHNode;
use crate::hittable::{FlipFace, Hittable, Transform};
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::mesh::Mesh;
use crate::scenes::prefabs::cornell_box::{
    cornell_box, cornell_box_camera, cornell_box_environment,
};
use crate::scenes::Scene;
use crate::sphere::Sphere;
use crate::texture::{ConstantTex, ImageTexture, Sampler::*, WrapMode::*};
use crate::vec::{vec, vec2, vec3, vec_zero};

pub fn scene() -> Scene {
    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(cornell_box());
    let glass = Arc::new(Dielectric {
        color: vec(1.0, 1.0, 1.0),
        // ref_idx: 1.1,
        ..Dielectric::default()
    });
    let earth_material = Arc::new(Lambertian {
        albedo: Arc::new(
            ImageTexture::new(String::from("assets/topo.jpg"))
                .sampler(Bilinear)
                .wrap_mode(Clamp),
        ),
    });
    let small_light = Arc::new(DiffuseLight {
        emit: Arc::new(ConstantTex {
            color: vec3(6.0, 6.0, 6.0),
        }),
    });

    // objects.push(Box::new(Transform::new_b(
    //     Box::new(FlipFace::new(AARect {
    //         xy0: vec2(-50.0, -50.0),
    //         xy1: vec2(50.0, 50.0),
    //         k: 554.0,
    //         material: small_light.clone(),
    //         rect_type: XZ,
    //     })),
    //     vec3(-200.0, 0.0, -200.0),
    //     vec_zero(),
    // )));
    // objects.push(Box::new(Transform::new_b(
    //     Box::new(FlipFace::new(AARect {
    //         xy0: vec2(-50.0, -50.0),
    //         xy1: vec2(50.0, 50.0),
    //         k: 554.0,
    //         material: small_light.clone(),
    //         rect_type: XZ,
    //     })),
    //     vec3(-200., 0.00, 200.0),
    //     vec_zero(),
    // )));
    // objects.push(Box::new(Transform::new_b(
    //     Box::new(FlipFace::new(AARect {
    //         xy0: vec2(-50.0, -50.0),
    //         xy1: vec2(50.0, 50.0),
    //         k: 554.0,
    //         material: small_light.clone(),
    //         rect_type: XZ,
    //     })),
    //     vec3(200.0, 0.0, -200.0),
    //     vec_zero(),
    // )));
    // objects.push(Box::new(Transform::new_b(
    //     Box::new(FlipFace::new(AARect {
    //         xy0: vec2(-50.0, -50.0),
    //         xy1: vec2(50.0, 50.0),
    //         k: 554.0,
    //         material: small_light.clone(),
    //         rect_type: XZ,
    //     })),
    //     vec3(200., 0.00, 200.0),
    //     vec_zero(),
    // )));

        objects.push(Box::new(Transform::new_b(
            Box::new(FlipFace::new(AARect {
                xy0: vec2(-50.0, -50.0),
                xy1: vec2(50.0, 50.0),
                k: 0.0,
                material: small_light.clone(),
                rect_type: XY,
            })),
            vec3(-200.0, 50.0, 500.0),
            vec3(20.0, -20.0 ,0.0),
        )));

        objects.push(Box::new(Transform::new_b(
            Box::new(FlipFace::new(AARect {
                xy0: vec2(-50.0, -50.0),
                xy1: vec2(50.0, 50.0),
                k: 0.0,
                material: small_light.clone(),
                rect_type: XY,
            })),
            vec3(200.0, 500.0, 500.0),
            vec3(-20.0, 20.0 ,0.0),
        )));

    objects.push(Box::new(Transform::new(
        Sphere::new(vec_zero(), 199.999, earth_material),
        vec3(0.0, 200.0, -100.0),
        vec_zero(),
    )));

    objects.push(Box::new(Transform::new(
        Sphere::new(vec_zero(), 200.0, glass),
        vec3(0.0, 200.0, -100.0),
        vec_zero(),
    )));

    Scene {
        camera: cornell_box_camera(),
        objects: BVHNode::build(objects, 0),
        environment: cornell_box_environment(),
    }
}

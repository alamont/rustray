use nalgebra::{Vector2, Vector3};
use std::sync::Arc;

use crate::hittable::{HittableList, Hittable, FlipFace, Transform};
use crate::camera::Camera;
use crate::material::{Dielectric, Lambertian, Metal, Environment, DiffuseLight};
use crate::aarect::{AARect, AARectType::*};
use crate::aabox::AABox;
use crate::vec::{vec, vec2, vec3, vec_one, random_vec, random_vec_range, vec_zero};
use crate::bvh::BVHNode;
use crate::texture::{ConstantTex, CheckerTex, ImageTexture};
use crate::scenes::Scene;

pub fn cornell_box(aspect: f32) -> Scene {

    let red = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.65, 0.05, 0.05) })});
    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let green = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.12, 0.45, 0.15) })});
    let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(14.0, 14.0, 14.0) })});
    let aluminium  = Arc::new(Metal { albedo: Arc::new(ConstantTex { color: vec3(0.8, 0.85, 0.85) } ), fuzz: 0.0});

    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    objects.push(Arc::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: green.clone(),
        rect_type: YZ
    })));
    objects.push(Arc::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 0.0,
        material: red.clone(),
        rect_type: YZ
    }));
    objects.push(Arc::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Arc::new(FlipFace::new(AARect { 
        xy0: vec2(213.0, 227.0), 
        xy1: vec2(343.0, 332.0),
        k: 554.0,
        material: light.clone(),
        rect_type: XZ
    })));
    // objects.push(Arc::new(FlipFace::new(AARect { 
    //     xy0: vec2(113.0, 127.0), 
    //     xy1: vec2(443.0, 442.0),
    //     k: 554.0,
    //     material: light.clone(),
    //     rect_type: XZ
    // })));
    objects.push(Arc::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 0.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Arc::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XY
    })));

    let box1 = Transform::new(
        AABox::new(
            vec3(165.0, 165.0, 165.0), 
            white.clone()
        ),
        vec3(115.0 + 165.0/2.0, 165.0/2.0, 65.0 + 165.0/2.0),
        vec3(0.0, -18.0, 0.0)
    );
    // let box2 = Transform::new(
    //     AABox::new(
    //         vec(165.0, 330.0, 165.0),
    //         white.clone()
    //     ),
    //     vec3(280.0 + 165.0/2.0, 330.0/2.0, 295.0 + 165.0/2.0),
    //     vec3(0.0, 15.0, 0.0)
    // );
    let box2 = Transform::new(
        AABox::new(
            vec(165.0, 330.0, 165.0),
            aluminium.clone()
        ),
        vec3(280.0 + 165.0/2.0, 330.0/2.0, 295.0 + 165.0/2.0),
        vec3(0.0, 15.0, 0.0)
    );

    objects.push(Arc::new(box1));
    objects.push(Arc::new(box2));



    let lookfrom = vec3(278.0, 278.0, -800.0);
    let lookat = vec3(278.0, 278.0, 0.0);
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
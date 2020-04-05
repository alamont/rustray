use nalgebra::{Vector2, Vector3};
use std::sync::Arc;

use crate::hittable::{HittableList, Hittable, FlipFace};
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
    let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(15.0, 15.0, 15.0) })});

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();

    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: green.clone(),
        rect_type: YZ
    })));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 0.0,
        material: red.clone(),
        rect_type: YZ
    }));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(213.0, 227.0), 
        xy1: vec2(343.0, 332.0),
        k: 554.0,
        material: light.clone(),
        rect_type: XZ
    })));
    objects.push(Box::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 0.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Box::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, 0.0), 
        xy1: vec2(555.0, 555.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XY
    })));

    objects.push(Box::new(AABox::new(
        vec3(130.0, 0.0, 65.0), 
        vec3(295.0, 165.0, 230.0), 
        white.clone()
    )));
    objects.push(Box::new(AABox::new(
        vec3(265.0, 0.0, 295.0), 
        vec3(430.0, 330.0, 460.0), 
        white.clone()
    )));


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
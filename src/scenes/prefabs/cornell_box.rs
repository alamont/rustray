use std::sync::Arc;

use crate::hittable::{Hittable, FlipFace, Transform};
use crate::material::{Lambertian, DiffuseLight, Environment};
use crate::aarect::{AARect, AARectType::*};
use crate::texture::{ConstantTex};
use crate::vec::{vec2, vec3, vec_zero};
use crate::bvh::BVHNode;
use crate::camera::Camera;


pub fn cornell_box() -> (Arc<dyn Hittable>, Arc<dyn Hittable>) {
    let red = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.65, 0.05, 0.05) })});
    let white = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.73, 0.73, 0.73) })});
    let green = Arc::new(Lambertian { albedo: Arc::new(ConstantTex { color: vec3(0.12, 0.45, 0.15) })});    
    let light = Arc::new(DiffuseLight { emit: Arc::new(ConstantTex { color: vec3(14.0, 14.0, 14.0) })});

    let mut objects: Vec<Arc<dyn Hittable>> = Vec::new();

    objects.push(Arc::new(FlipFace::new(AARect { 
        xy0: vec2(0.0, -278.0), 
        xy1: vec2(555.0, 278.0),
        k: -278.0,
        material: green.clone(),
        rect_type: YZ
    })));
    objects.push(Arc::new(AARect { 
        xy0: vec2(0.0, -278.0), 
        xy1: vec2(555.0, 278.0),
        k: 278.0,
        material: red.clone(),
        rect_type: YZ
    }));
    objects.push(Arc::new(AARect { 
        xy0: vec2(-278.0, -278.0), 
        xy1: vec2(278.0, 278.0),
        k: 555.0,
        material: white.clone(),
        rect_type: XZ
    }));
    let light = Arc::new(AARect { 
        xy0: vec2(-65.0, -65.0), 
        xy1: vec2(65.0, 65.0),
        k: 554.0,
        material: light.clone(),
        rect_type: XZ
    });
    objects.push(light.clone());
    objects.push(Arc::new(AARect { 
        xy0: vec2(-278.0, -278.0), 
        xy1: vec2(278.0, 278.0),
        k: 0.0,
        material: white.clone(),
        rect_type: XZ
    }));
    objects.push(Arc::new(FlipFace::new(AARect { 
        xy0: vec2(-278.0, 0.0), 
        xy1: vec2(278.0, 555.0),
        k: -278.0,
        material: white.clone(),
        rect_type: XY
    })));

    (BVHNode::build(objects, 0), light.clone())
}

pub fn cornell_box_camera() -> Camera {
    let lookfrom = vec3(0.0, 278.0, 1078.0);
    let lookat = vec3(0.0, 278.0, 0.0);
    let vup = vec3(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom-lookat).magnitude();
    let aperture = 0.0;
    let vfov = 40.0;
    let aspect = 1.0;

    Camera::new(lookfrom, lookat, vup, vfov, aspect, aperture, dist_to_focus)
}

pub fn cornell_box_environment() -> Arc<Environment> {
    Arc::new(Environment { emit: Arc::new(ConstantTex { color: vec_zero() })})
}
mod ray;
mod hitable;
mod sphere;
mod camera;
mod vec;
mod material;

#[macro_use]
extern crate slice_as_array;

use crate::ray::Ray;
use crate::hitable::{Hitable, HitRecord, HitableList};
use crate::material::{Lambertian, Metal};

use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use itertools::izip;
use nalgebra::Vector3;
use rayon::prelude::*;
use std::{fs, f32};
use sphere::Sphere;
use camera::Camera;
use rand::{thread_rng, Rng};
use vec::{random_unit_vec};

fn ray_color(ray: &Ray, world: &HitableList, depth: u32) -> Vector3<f32> {

    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0)
    }

    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {

        if let Some((new_ray, attenuation)) = hit_rec.material.scatter(&ray, &hit_rec) {
            return attenuation.component_mul(&ray_color(&new_ray, world, depth-1));
        }
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        let unit_direction: Vector3<f32> = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}



fn main() {

    let nx: u32 = 200;
    let ny: u32 = 100;
    let ns = 100;
    let max_depth = 50;

    let cam = Camera::default();

    let mut world = HitableList::default();
    world.push(Sphere{
        center: Vector3::new(0.0, 0.0, -1.0), 
        radius: 0.5, 
        material: Box::new(Lambertian{albedo: Vector3::new(0.7, 0.3, 0.3)})
    });
    world.push(Sphere{
        center: Vector3::new(0.0, -100.5, -1.0), 
        radius: 100.0, 
        material: Box::new(Lambertian{albedo: Vector3::new(0.8, 0.8, 0.8)})
    });

    world.push(Sphere{
        center: Vector3::new(1.0 ,0.0, -1.0), 
        radius: 0.5, 
        material: Box::new(Metal{albedo: Vector3::new(0.8, 0.6, 0.2), fuzz: 1.0})
    });
    world.push(Sphere{
        center: Vector3::new(-1.0 ,0.0, -1.0), 
        radius: 0.5, 
        material: Box::new(Metal{albedo: Vector3::new(0.8, 0.8, 0.8), fuzz: 0.3})
    });

    let image = (0..ny).into_par_iter().rev()
        .map(|y| {
            let mut rng = thread_rng();
            (0..nx).map(|x| {
                let col: Vector3<f32> = (0..ns).map(|_|{
                    let u = (x as f32 + rng.gen::<f32>())/ nx as f32;
                    let v = (ny as f32 - (y as f32 + rng.gen::<f32>())) / ny as f32;
                    let ray = cam.get_ray(u, v);
                    ray_color(&ray, &world, max_depth)
                }).sum();
                col / (ns as f32)
            }).collect::<Vec<Vector3<f32>>>()
        }).collect::<Vec<Vec<Vector3<f32>>>>();

    let mut imgbuf = image::ImageBuffer::new(nx, ny);

    for (y, row) in izip!(0..ny, image) {
        for (x, image_pix) in izip!(0..nx, row) {
            let pixel = imgbuf.get_pixel_mut(x, ny - y - 1);
            let image_pix_rgb = (255.99 * image_pix)
                .iter()
                .map(|c| *c as u8)
                .collect::<Vec<u8>>();
            *pixel = Rgb([image_pix_rgb[0], image_pix_rgb[1], image_pix_rgb[2]]);
        }
    }


    
    let paths = fs::read_dir("output/").unwrap();
    let mut names =
    paths.filter_map(|entry| {
    entry.ok().and_then(|e|
        e.path().file_name()
        .and_then(|n| n.to_str().map(|s| String::from(s)))
    )
    }).collect::<Vec<String>>();

    names.sort();

    if let Some(name) = names.last() {
        let s: String = name.chars().take(name.len() - 4).collect();
        let new_output_image = format!("{:03}", (s.parse::<i32>().unwrap() + 1)).to_string() + ".png";
        imgbuf.save("output/".to_string() + &new_output_image).unwrap();
    }

}


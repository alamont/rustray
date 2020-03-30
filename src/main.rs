mod camera;
mod hitable;
mod material;
mod ray;
mod scenes;
mod sphere;
mod vec;

extern crate minifb;
extern crate slice_as_array;

use crate::hitable::{HitRecord, Hitable, HitableList};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::ray::Ray;

use camera::Camera;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};
use itertools::izip;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use scenes::{random_scene, simple_scene};
use sphere::Sphere;
use std::time::Instant;
use std::{f32, fs};
use vec::{random_unit_vec, vec, vec_zero};

const WIDTH: usize = 600;
const HEIGHT: usize = 300;

fn ray_color(ray: &Ray, world: &HitableList, depth: u32) -> Vector3<f32> {
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        if let Some((new_ray, attenuation)) = hit_rec.material.scatter(&ray, &hit_rec) {
            return attenuation.component_mul(&ray_color(&new_ray, world, depth - 1));
        }
        return Vector3::new(0.0, 0.0, 0.0);
    } else {
        let unit_direction: Vector3<f32> = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn display() -> Window {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale_mode: ScaleMode::AspectRatioStretch,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window
}

fn main() {
    let now = Instant::now();

    let nx: u32 = WIDTH as u32;
    let ny: u32 = HEIGHT as u32;
    let ns = 1000;
    let max_depth = 50;

    let mut window = display();
    // let mut imgbuf = image::ImageBuffer::new(nx, ny);
    let mut u32_buffer: Vec<u32>;
    let lookfrom = vec(12.0, 2.0, 3.0);
    let lookat = vec_zero();
    let vup = vec(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(lookfrom, lookat, vup, 20.0, 2.0, aperture, dist_to_focus);
    let world = random_scene();

    // let image = (0..ny).into_par_iter().rev()
    //     .map(|y| {
    //         let mut rng = thread_rng();
    //         (0..nx).map(|x| {
    //             let col: Vector3<f32> = (0..ns).map(|_|{
    //                 let u = (x as f32 + rng.gen::<f32>())/ nx as f32;
    //                 let v = (ny as f32 - (y as f32 + rng.gen::<f32>())) / ny as f32;
    //                 let ray = cam.get_ray(u, v);
    //                 ray_color(&ray, &world, max_depth)
    //             }).sum();
    //             col / (ns as f32)
    //         }).collect::<Vec<Vector3<f32>>>()
    //     }).collect::<Vec<Vec<Vector3<f32>>>>();
    let mut image_buf: Vec<f32> = vec![0.0; (nx * ny * 3) as usize];

    for n in (0..ns) {

        image_buf = (0..ny)
            .into_par_iter()
            .flat_map(|y| {
                let mut rng = thread_rng();
                (0..nx)
                    .flat_map(|x| {
                        let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                        let v = (ny as f32 - (y as f32 + rng.gen::<f32>())) / ny as f32;
                        let ray = cam.get_ray(u, v);
                        let col = ray_color(&ray, &world, max_depth);
                        let offset = ((y * nx + x) * 3) as usize;
                        vec![
                            col.x + image_buf[offset],
                            col.y + image_buf[offset + 1],
                            col.z + image_buf[offset + 2],
                        ]
                    })
                    .collect::<Vec<f32>>()
            })
            .collect::<Vec<f32>>();

        u32_buffer = image_buf
            .iter()
            .map(|sp| ((*sp / ((n+1) as f32) * 255.99) as u8))
            // .map(|sp| (*sp as f32 * 255.99) as u8)
            .collect::<Vec<u8>>()
            .chunks(3)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();

        window
            .update_with_buffer(&u32_buffer, WIDTH, HEIGHT)
            .unwrap();

        println!("sample: {}", n);
    }


    // let paths = fs::read_dir("output/").unwrap();
    // let mut names =
    // paths.filter_map(|entry| {
    // entry.ok().and_then(|e|
    //     e.path().file_name()
    //     .and_then(|n| n.to_str().map(|s| String::from(s)))
    // )
    // }).collect::<Vec<String>>();

    // names.sort();

    // if let Some(name) = names.last() {
    //     let s: String = name.chars().take(name.len() - 4).collect();
    //     let new_output_image = format!("{:03}", (s.parse::<i32>().unwrap() + 1)).to_string() + ".png";
    //     imgbuf.save("output/".to_string() + &new_output_image).unwrap();
    // }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}

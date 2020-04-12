#![allow(dead_code)]
#![allow(unused_imports)]
mod camera;
mod hittable;
mod material;
mod ray;
mod scenes;
mod sphere;
mod vec;
mod aabb;
mod bvh;
mod texture;
mod world;
mod aarect;
mod aabox;
mod volume;
mod triangle;
mod mesh;
mod utils;

use cmd_lib::run_cmd;
use hittable::{Hittable};
use ray::Ray;
use vec::vec_zero;
use image::{ImageBuffer, hdr::{HDREncoder}, Rgb};
use material::EnvironmentMaterial;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use scenes::{
    Scene,
    // random_scene_bvh::random_scene_bvh,
    // random_scene::random_scene,
    // // dielectric_scene::dielectric_scene,
    // earth_scene::earth_scene,
    // random_scene_light::random_scene_light,
    // cornell_box_scene::cornell_box,
    // cornell_box_vol::cornell_box_vol,
    // cornell_box_mesh::cornell_box_mesh
    cornell_box_texture_filtering::scene
};
use std::{f32, fs, sync::Arc, io, time::Instant};

static mut RAY_COUNT: u32 = 0;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;
const HDR_OUTPUT: bool = true;
const DENOISE: bool = true;

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; }
    if x > max { return max; } 
    x
}

fn ray_color(ray: &Ray, world: &Arc<dyn Hittable>, environment: &Arc<dyn EnvironmentMaterial>, depth: u32) -> Vector3<f32> {
    unsafe {
        RAY_COUNT += 1;
    }
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        if let Some((new_ray, attenuation)) = hit_rec.material.scatter(&ray, &hit_rec) {
            return attenuation.component_mul(&ray_color(&new_ray, world, environment, depth - 1));
        }
        let emitted = hit_rec.material.emitted(ray, &hit_rec);
        return emitted;
    } else {
        environment.emit(ray)
    }
}

fn ray_albedo(ray: &Ray, world: &Arc<dyn Hittable>) -> Vector3<f32> {
    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        if let Some((_new_ray, attenuation)) = hit_rec.material.scatter(&ray, &hit_rec) {
            return attenuation;
        }
    }
    vec_zero()
}

fn ray_normal(ray: &Ray, world: &Arc<dyn Hittable>) -> Vector3<f32> {
    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        return hit_rec.normal.normalize();
    }
    vec_zero()
}

fn display() -> Window {
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

    let nx: u32 = WIDTH as u32;
    let ny: u32 = HEIGHT as u32;
    let ns = 10000;
    let max_depth = 50;

    let mut window = display();  
    let mut u32_buffer: Vec<u32>;
    let mut completed_samples = 0;
    let mut save_images = false;


    let mut image_buf: Vec<f32> = vec![0.0; (nx * ny * 3) as usize];

    // let aspect = nx as f32 / ny as f32;
    let scene = scene();

    let world = scene.objects;
    let environment = scene.environment;
    let cam = scene.camera;

    
    let now = Instant::now();

    // This incremental method is actually twice as fast as the more functional approach.
    for n in 0..ns {
        image_buf = (0..ny)
            .into_par_iter()
            .flat_map(|y| {
                let mut rng = thread_rng();
                (0..nx)
                    .flat_map(|x| {
                        let u = (x as f32 + rng.gen::<f32>()) / nx as f32;
                        let v = (ny as f32 - (y as f32 + rng.gen::<f32>())) / ny as f32;
                        let ray = cam.get_ray(u, v);
                        let col = ray_color(&ray, &world, &environment, max_depth);
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

        let pixel_scale = 1.0 / ((n+1) as f32);
        u32_buffer = image_buf
            .iter()
            .map(|sp| clamp((*sp * pixel_scale).sqrt() * 255.99, 0.0, 255.0) as u8)
            .collect::<Vec<u8>>()
            .chunks(3)
            .map(|v| ((v[0] as u32) << 16) | ((v[1] as u32) << 8) | v[2] as u32)
            .collect();

        window
            .update_with_buffer(&u32_buffer, WIDTH, HEIGHT)
            .unwrap();

        unsafe {
            println!("samples: {}, rays: {:.2} M", n, RAY_COUNT as f32 / 1e6);
        }
        completed_samples += 1;

        if !window.is_open() || window.is_key_down(Key::Escape) || window.is_key_released(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::S) || window.is_key_released(Key::S) {
            save_images = true;
            break;
        }
    }
    
    let albedo_buf = (0..ny)
        .into_par_iter()
        .flat_map(|y| {
            (0..nx)
                .flat_map(|x| {
                    let u = (x as f32) / nx as f32;
                    let v = (ny as f32 - (y as f32)) / ny as f32;
                    let ray = cam.get_ray_an(u, v);
                    let col = ray_albedo(&ray, &world);
                    vec![col.x, col.y, col.z]
                }).collect::<Vec<f32>>()
        }).collect::<Vec<f32>>();

    let normal_buf = (0..ny)
        .into_par_iter()
        .flat_map(|y| {
            (0..nx)
                .flat_map(|x| {
                    let u = (x as f32) / nx as f32;
                    let v = (ny as f32 - (y as f32)) / ny as f32;
                    let ray = cam.get_ray_an(u, v);
                    let col = ray_normal(&ray, &world);
                    vec![col.x, col.y, col.z]
                }).collect::<Vec<f32>>()
        }).collect::<Vec<f32>>();

    if completed_samples == ns {
        save_images = true;
    }
    
    let elapsed = now.elapsed();
    unsafe {
        println!("Elapsed time: {:.2?}, total samples per pixel: {}, total rays: {:.2} M", elapsed, completed_samples, RAY_COUNT as f32 / 1e6);
    }


    if save_images {

        let mut imgbuf = ImageBuffer::new(nx, ny);
        let pixel_scale = 1.0 / completed_samples as f32;
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let offset = ((y * nx + x) * 3) as usize;
            let r = clamp((image_buf[offset] * pixel_scale as f32).sqrt() * 255.99, 0.0, 255.0) as u8;
            let g = clamp((image_buf[offset + 1] * pixel_scale as f32).sqrt() * 255.99, 0.0, 255.0) as u8;
            let b = clamp((image_buf[offset + 2] * pixel_scale as f32).sqrt() * 255.99, 0.0, 255.0) as u8;

            *pixel = image::Rgb([r, g, b]);
        }


        let paths = fs::read_dir("output/png/").unwrap();
        let mut names =
        paths.filter_map(|entry| {
        entry.ok().and_then(|e|
            e.path().file_name()
            .and_then(|n| n.to_str().map(|s| String::from(s)))
        )
        }).collect::<Vec<String>>();

        names.sort();
        let mut output_image_name = String::new();

        if let Some(name) = names.last() {
            let s: String = name.chars().take(name.len() - 4).collect();
            output_image_name = format!("{:03}", (s.parse::<i32>().unwrap() + 1));
            let output_path = "output/png/".to_string() + &output_image_name + ".png";
            println!("Saved image to {}", output_path);
            imgbuf.save(output_path).unwrap();
        }


        if HDR_OUTPUT {
            let image_buf_rgb = image_buf.chunks(3).map(|pix| {
                image::Rgb([
                    pix[0] / completed_samples as f32, 
                    pix[1] / completed_samples as f32, 
                    pix[2] / completed_samples as f32])
            }).collect::<Vec<Rgb<f32>>>();

            let file = fs::File::create(format!("output/hdr/{}.hdr", output_image_name)).unwrap();
            let encoder = HDREncoder::new(io::BufWriter::new(file));

            encoder.encode(&image_buf_rgb[..], WIDTH, HEIGHT).unwrap();

            let _ = fs::remove_file("output/temp/albedo.png");
            let _ = fs::remove_file("output/temp/normal.png");


            // Albedo
            let mut imgbuf_albedo = ImageBuffer::new(nx, ny);
            for (x, y, pixel) in imgbuf_albedo.enumerate_pixels_mut() {
                let offset = ((y * nx + x) * 3) as usize;
                let r = clamp(albedo_buf[offset] * 255.99, 0.0, 255.0) as u8;
                let g = clamp(albedo_buf[offset + 1] * 255.99, 0.0, 255.0) as u8;
                let b = clamp(albedo_buf[offset + 2] * 255.99, 0.0, 255.0) as u8;
                *pixel = image::Rgb([r, g, b]);
            }
            imgbuf_albedo.save("output/temp/albedo.png").unwrap();

            //Normal
            let mut imgbuf_normal = ImageBuffer::new(nx, ny);
            for (x, y, pixel) in imgbuf_normal.enumerate_pixels_mut() {
                let offset = ((y * nx + x) * 3) as usize;
                let r = clamp((normal_buf[offset] + 1.0) / 2.0 * 255.99, 0.0, 255.0) as u8;
                let g = clamp((normal_buf[offset + 1] + 1.0) / 2.0 * 255.99, 0.0, 255.0) as u8;
                let b = clamp((normal_buf[offset + 2] + 1.0) / 2.0 * 255.99, 0.0, 255.0) as u8;
                *pixel = image::Rgb([r, g, b]);
            }
            imgbuf_normal.save("output/temp/normal.png").unwrap();
        }

        if DENOISE {
            let _ = run_cmd!("Denoiser.exe -i output/hdr/{}.hdr -a output/temp/albedo.png -n  output/temp/normal.png -o output/hdr-denoised/{}.hdr", output_image_name, output_image_name);
            let _ = run_cmd!("Denoiser.exe -i output/png/{}.png -a output/temp/albedo.png -n  output/temp/normal.png -o output/png-denoised/{}.png", output_image_name, output_image_name);
        }
    }

}

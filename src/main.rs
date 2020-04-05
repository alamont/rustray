#![allow(dead_code)]
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

use cmd_lib::run_cmd;
use hittable::{Hittable};
use ray::Ray;
use image::{ImageBuffer, hdr::{HDREncoder}, Rgb};
use material::EnvironmentMaterial;
use minifb::{Key, ScaleMode, Window, WindowOptions};
use nalgebra::Vector3;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use scenes::{
    Scene,
    // random_scene_bvh::random_scene_bvh,
    random_scene::random_scene,
    // dielectric_scene::dielectric_scene,
    earth_scene::earth_scene,
    random_scene_light::random_scene_light,
    cornell_box::cornell_box,
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

fn ray_color(ray: &Ray, world: &Box<dyn Hittable>, environment: &Arc<dyn EnvironmentMaterial>, depth: u32) -> Vector3<f32> {
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

    let mut ray_count: u32 = 0;

    let mut window = display();
    
    let mut u32_buffer: Vec<u32>;

    let aspect = nx as f32 / ny as f32;

    let scene = cornell_box(aspect);
    let world = scene.objects;
    let environment = scene.environment;
    let cam = scene.camera;

    let mut image_buf: Vec<f32> = vec![0.0; (nx * ny * 3) as usize];

    let mut completed_samples = 0;

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
    }

    
    let elapsed = now.elapsed();
    unsafe {
        println!("Elapsed time: {:.2?}, total samples per pixel: {}, total rays: {:.2} M", elapsed, completed_samples, RAY_COUNT as f32 / 1e6);
    }




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
        let image_bug_rgb = image_buf.chunks(3).map(|pix| {
            image::Rgb([
                pix[0] / completed_samples as f32, 
                pix[1] / completed_samples as f32, 
                pix[2] / completed_samples as f32])
        }).collect::<Vec<Rgb<f32>>>();

        let file = fs::File::create(format!("output/hdr/{}.hdr", output_image_name)).unwrap();
        let encoder = HDREncoder::new(io::BufWriter::new(file));

        encoder.encode(&image_bug_rgb[..], WIDTH, HEIGHT).unwrap();
    }

    if DENOISE {
        run_cmd!("Denoiser.exe -i output/hdr/{}.hdr -o output/hdr-denoised/{}.hdr", output_image_name, output_image_name);
        run_cmd!("Denoiser.exe -i output/png/{}.png -o output/png-denoised/{}.png", output_image_name, output_image_name);
    }

}

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
mod pdf;

use cmd_lib::run_cmd;
use hittable::{Hittable};
use ray::Ray;
use vec::{vec_zero, has_nan, vec2, vec3};
use image::{ImageBuffer, hdr::{HDREncoder}, Rgb};
use material::EnvironmentMaterial;
use pdf::{Pdf, CosinePdf, HittablePdf, MixturePdf, EnvPdf};
use aarect::{AARectType, AARect};
use material::{EmptyMaterial};
use minifb::{Key, ScaleMode, Window, WindowOptions, MouseMode, MouseButton};
use nalgebra::{Vector3, UnitQuaternion, Point3, Translation3};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use kiss3d;
use scenes::{
    Scene,
    // random_scene_bvh::random_scene_bvh,
    // random_scene::random_scene,
    // dielectric_scene::dielectric_scene,
    // earth_scene::earth_scene,
    // random_scene_light::random_scene_light,
    // cornell_box_scene::cornell_box,
    // cornell_box_vol::cornell_box_vol,
    // cornell_box_mesh::cornell_box_mesh
    // cornell_box_texture_filtering::scene
    env_scene::scene
    // cornell_box_scene::scene
};
use std::{f32, fs, sync::{Arc, mpsc, mpsc::{Sender, Receiver}}, io, time::{Instant, Duration}, thread};

static mut RAY_COUNT: u32 = 0;

const WIDTH: usize = 1000;
const HEIGHT: usize = 500;
const HDR_OUTPUT: bool = true;
const DENOISE: bool = true;
const DEBUG_WINDOW: bool = false;

#[derive(Clone)]
pub enum PathDebugMsg  {
    Ray(Vector3<f32>, Vector3<f32>),
    Hit(Vector3<f32>, Vector3<f32>)
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min { return min; }
    if x > max { return max; } 
    x
}

fn ray_color(
    ray: &Ray, 
    world: Arc<dyn Hittable>, 
    environment: Arc<dyn EnvironmentMaterial>,
    mis_objects: &Vec<Arc<dyn Hittable>>,
    depth: u32,
    _tx: Option<&Sender<PathDebugMsg>>) -> Vector3<f32> {
        
    unsafe {
        RAY_COUNT += 1;
    }

    // Bounce limit exceeded
    if depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }


    if let Some(hit_rec) = world.hit(&ray, 0.001, f32::MAX) {

        if let Some(tx) = _tx {
            tx.send(PathDebugMsg::Ray(ray.origin(), hit_rec.p));
            tx.send(PathDebugMsg::Hit(hit_rec.p, hit_rec.normal));
        }
    

        let mut emitted = hit_rec.material.emitted(&ray, &hit_rec);
        if has_nan(&emitted) {
            emitted = vec_zero();
        }

        // Scatter
        if let Some(scatter_record) = hit_rec.material.scatter(&ray, &hit_rec) {
            // If the ray is specualar don't do the multiple importance sampling
            if let Some(mut specular_ray) = scatter_record.specular_ray {
                specular_ray.debug = ray.debug;
                return scatter_record.attenuation.component_mul(&ray_color(&specular_ray, world, environment, mis_objects, depth - 1, _tx));
            }
            let attenuation = scatter_record.attenuation;
            if has_nan(&attenuation) {
                return emitted;
            }
            let mut pdfs: Vec<Arc<dyn Pdf>> = Vec::new();
            if mis_objects.len() > 0 {                
                for mo in mis_objects.iter() {
                    pdfs.push(Arc::new(HittablePdf { origin: hit_rec.p, hittable: mo.clone() }));
                }
            }
            if let Some(pdf) = scatter_record.pdf {
                pdfs.push(pdf);
            }
            // pdfs.push(Arc::new(EnvPdf { environment: environment.clone() }));
            
            let mixture_pdf = MixturePdf::new_uniform(pdfs);
            let mut scattered_ray = Ray::new(hit_rec.p, mixture_pdf.generate());
            scattered_ray.debug = ray.debug;
            let pdf_val = mixture_pdf.value(scattered_ray.direction());            

            return emitted + (attenuation * hit_rec.material.scattering_pdf(&ray, &hit_rec, &scattered_ray))
                .component_mul(&ray_color(&scattered_ray, world, environment.clone(), mis_objects, depth - 1, _tx)) / pdf_val;
        }
        emitted

    } else {
        // Return environment if ray hits nothing        
        let emitted = environment.emit(&ray);
        if has_nan(&emitted) {
            return vec_zero();
        }
        emitted
    }
}


fn ray_albedo(ray: &Ray, world: &Arc<dyn Hittable>) -> Vector3<f32> {
    if let Some(hit_rec) = world.hit(ray, 0.001, f32::MAX) {
        if let Some(scatter_record) = hit_rec.material.scatter(&ray, &hit_rec) {
            let attenuation = scatter_record.attenuation;
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

fn debug_display(rx: Receiver<PathDebugMsg>) {
    thread::spawn(move || {
        // some work here
    
        let mut kiss_window = kiss3d::window::Window::new("Kiss3d: cube");
        // let mut c = kiss_window.add_cube(1.0, 1.0, 1.0);
        // kiss_window.add_sphere(r)

        // c.set_color(1.0, 0.0, 0.0);

        let mut sphere = kiss_window.add_sphere(200.0);
        sphere.append_translation(&Translation3::new(0.0, 200.0, -100.0));

        sphere.set_color(0.2, 0.2, 0.2);
        sphere.set_points_size(10.0);
        sphere.set_lines_width(1.0);
        sphere.set_surface_rendering_activation(false);


        kiss_window.set_light(kiss3d::light::Light::StickToCamera);
        kiss_window.set_point_size(10.0);

        // let rot: UnitQuaternion<f32> = UnitQuaternion::from_axis_angle(&Vector3::y_axis(), 0.014);
        let mut lines = Vec::new();
        let mut hits = Vec::new();

        while kiss_window.render() {
            // c.prepend_to_local_rotation(&rot);
            if let Ok(received) = rx.try_recv() {
                match received {
                    PathDebugMsg::Ray(o, d) => {
                        lines.push((Point3::from(o), Point3::from(d), Point3::new(0.0, 1.0, 0.0)));                        
                    },
                    PathDebugMsg::Hit(p, n) => {
                        hits.push((Point3::from(p), Point3::from(n)));
                    }
                }                
            }
            lines.iter().for_each(|line| {
                kiss_window.draw_line(&line.0, &line.1, &line.2);
            });
            hits.iter().for_each(|h| {
                kiss_window.draw_point(&h.0, &Point3::new(1.0, 0.0, 0.0));
                kiss_window.draw_line(&h.0, &Point3::from(h.0 + Vector3::new(h.1.x, h.1.y, h.1.z) * 10.0), &Point3::new(0.0, 0.0, 1.0));          
            });
        }
    });
}

fn main() {

    let nx: u32 = WIDTH as u32;
    let ny: u32 = HEIGHT as u32;
    let ns = 10000;
    let max_depth = 50;
    let (tx, rx): (Sender<PathDebugMsg>, Receiver<PathDebugMsg>) = mpsc::channel();

    if DEBUG_WINDOW {   
        debug_display(rx);
    }

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
    let mis_objects = scene.mis_objects;

    
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
                        let col = ray_color(&ray, world.clone(), environment.clone(), &mis_objects, max_depth, None);
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

        unsafe {
            println!("samples: {}, rays: {:.2} M", n, RAY_COUNT as f32 / 1e6);
        }
        completed_samples += 1;
        let mut paused = false;
        let mut exit = false;

        loop {
            window
                .update_with_buffer(&u32_buffer, WIDTH, HEIGHT)
                .unwrap();        

            if !window.is_open() || window.is_key_down(Key::Escape) || window.is_key_released(Key::Escape) {
                exit = true;
                paused = false;
            }

            if window.is_key_down(Key::S) || window.is_key_released(Key::S) {
                save_images = true;
                exit = true;
                paused = false;
            }

            window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
                if window.get_mouse_down(MouseButton::Left) {
                    paused = true;
                    // tx.send(format!("PATH mx:{}, my:{} - ", x, y)).unwrap();
    
                    let u = (x as f32) / WIDTH as f32;
                    let v = (ny as f32 - (y as f32)) / HEIGHT as f32;
                    let mut ray = cam.get_ray(u, v);
                    ray.debug = true;
                    let col = ray_color(&ray, world.clone(), environment.clone(), &mis_objects, max_depth, Some(&tx));
 
                }
                    
                if window.get_mouse_down(MouseButton::Right) {
                    paused = false;
                }                 
            });

            if !paused {
                break;
            }
        }

        if exit == true {
            break;
        }

        // Debug rays at mouse position
        
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

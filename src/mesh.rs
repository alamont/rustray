use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable, HittableList, FlipFace};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{vec3, vec_one};
use crate::bvh::BVHNode;
use crate::triangle::Triangle;

use nalgebra::{Vector2, Vector3};
use std::f32;
use std::sync::Arc;

use std::path::Path;
use tobj;

pub struct Mesh {
    pub triangles:Arc<dyn Hittable>,
    pub material: Arc<dyn Material>,
}

impl Mesh {
    pub fn new(mesh_path: String, material:  Arc<dyn Material>, scale: Vector3<f32>) -> Self {

        // let triangles: Vec<Arc<dyn Hittable>> = Vec::new();

        let obj = tobj::load_obj(&Path::new(&mesh_path));
        assert!(obj.is_ok());
        let (models, _materials) = obj.unwrap();

        let mesh = &models[0].mesh;
        println!("MODELS: {}", &models.len());

        let triangles = mesh.indices.chunks(3).map(|iii| {
            let v = iii.into_iter().map(|i| {
                Vector3::new(
                    mesh.positions[(*i * 3 + 0) as usize],
                    mesh.positions[(*i * 3 + 1) as usize],
                    mesh.positions[(*i * 3 + 2) as usize]
                )
            }).collect::<Vec<Vector3<f32>>>();
            let n = if mesh.normals.len() > 0 {
                iii.into_iter().map(|i| {
                    Vector3::new(
                        mesh.normals[(*i * 3 + 0) as usize],
                        mesh.normals[(*i * 3 + 1) as usize],
                        mesh.normals[(*i * 3 + 2) as usize]
                    )
                }).collect::<Vec<Vector3<f32>>>()
            } else {
                let edge1 = v[1] - v[0];
                let edge2 = v[2] - v[0];
                let normal = edge1.cross(&edge2).normalize();
                vec![normal, normal, normal]
            };


            Arc::new(Triangle {
                v0: v[0] * scale.x,
                v1: v[1] * scale.y,
                v2: v[2] * scale.z,
                material: material.clone(),
                n0: n[0],
                n1: n[1],
                n2: n[2],
            }) as Arc<dyn Hittable>
        }).collect::<Vec<Arc<dyn Hittable>>>();

        Self {
            triangles: BVHNode::build(triangles, 0),
            material: material.clone()
        }

    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.triangles.hit(ray, t_min, t_max)
    }
    fn bounding_box(&self) -> Option<AABB> {
        self.triangles.bounding_box()
    }
}
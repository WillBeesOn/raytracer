use crate::{Light, Ray, Scene, Vec3};
use crate::materials::{Flat, Hall, Phong};
use crate::traits::HitData;

pub trait Mat {
    fn get_color(&self, scene: &Scene, incoming_ray: Ray, hit: &HitData, reflect_depth: u32) -> Vec3;
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Flat(Flat),
    Phong(Phong),
    Hall(Hall)
}

impl Mat for Material {
    fn get_color(&self, scene: &Scene, incoming_ray: Ray, hit: &HitData, reflect_depth: u32) -> Vec3 {
        match self {
            Material::Flat(mat) => mat.get_color(scene, incoming_ray, hit, reflect_depth),
            Material::Phong(mat) => mat.get_color(scene, incoming_ray, hit, reflect_depth),
            Material::Hall(mat) => mat.get_color(scene, incoming_ray, hit, reflect_depth)
        }
    }
}

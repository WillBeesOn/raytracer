use crate::{Light, Vec3};
use crate::materials::{Flat, Phong};
use crate::traits::HitData;

pub trait Mat {
    fn get_color(&self, lights: &Vec<Light>, hit: &HitData) -> Vec3;
    fn set_color(&mut self, color: Vec3);
}

#[derive(Debug, Copy, Clone)]
pub enum Material {
    Flat(Flat),
    Phong(Phong)
}

impl Mat for Material {
    fn get_color(&self, lights: &Vec<Light>, hit: &HitData) -> Vec3 {
        match self {
            Material::Flat(mat) => mat.get_color(lights, hit),
            Material::Phong(mat) => mat.get_color(lights, hit)
        }
    }

    fn set_color(&mut self, color: Vec3) {
        match self {
            Material::Flat(mat) => mat.set_color(color),
            Material::Phong(mat) => mat.set_color(color)
        }
    }
}

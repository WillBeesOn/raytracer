use crate::data_structures::{Vector, Vec3};
use crate::{vec3};

pub trait WorldLight {
    fn get_light_color(self, surface_point: Vec3) -> Vec3;
    fn get_position(self) -> Vec3;
    fn set_color(&mut self, color: Vec3);
    fn set_position(&mut self, position: Vec3);
}

#[derive(Debug, Copy, Clone)]
pub enum Light {
    PointLight(PointLight)
}

impl WorldLight for Light {
    fn get_light_color(self, surface_point: Vec3) -> Vec3 {
        match self {
            Light::PointLight(light) => light.get_light_color(surface_point)
        }
    }

    fn get_position(self) -> Vec3 {
        match self {
            Light::PointLight(light) => light.get_position()
        }
    }

    fn set_color(&mut self, color: Vec3) {
        match self {
            Light::PointLight(light) => light.set_color(color)
        }
    }

    fn set_position(&mut self, position: Vec3) {
        match self {
            Light::PointLight(light) => light.set_position(position)
        }
    }
}

// A light that emits into all directions. A spherical light.
#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub color: Vec3,
    pub position: Vec3,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(color: Vec3, intensity: f64) -> Self {
        PointLight {
            color,
            intensity,
            position: vec3![0.0, 0.0, 0.0],
        }
    }
}

impl WorldLight for PointLight {
    // The amount of light hitting a surface is inversely proportional to the distance^2 between them (attenuation).
    // But it is also proportional to the angle between surface normal and ray from surface to light.
    fn get_light_color(self, surface_point: Vec3) -> Vec3 {
        let distance = (self.position - surface_point).length(); // Ray from light to hit point
        (self.color * self.intensity) / (distance * distance)
    }

    fn get_position(self) -> Vec3 {
        self.position
    }

    fn set_color(&mut self, color: Vec3) {
        self.color = color;
    }

    fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
}
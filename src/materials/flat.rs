use crate::{Light, Vec3, vec3, WorldLight};
use crate::materials::Mat;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Flat {
    albedo: Vec3
}

impl Flat {
    pub fn new(albedo: Vec3) -> Self {
        Flat { albedo }
    }
}

impl Mat for Flat {
    fn get_color(&self, lights: &Vec<Light>, hit: &HitData) -> Vec3 {
        // Collect the effects all lights have on the surface at the hit point
        let mut all_light = vec3![0.0, 0.0, 0.0];
        for l in lights.iter() {
            all_light += l.get_light_color(hit.hit_point);
        }
        all_light + self.albedo
    }

    fn set_color(&mut self, color: Vec3) {
        self.albedo = color;
    }
}

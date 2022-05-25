use crate::{Light, Ray, Scene, Vec3, vec3, WorldLight};
use crate::materials::Mat;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Flat {
    albedo: Vec3,
    reflect_factor: f64
}

impl Flat {
    pub fn new(albedo: Vec3, reflect_factor: f64) -> Self {
        Flat { albedo, reflect_factor }
    }
}

impl Mat for Flat {
    fn get_color(&self, scene: &Scene, incoming_ray: Ray, hit: &HitData, reflect_depth: u32) -> Vec3 {
        // Collect the effects all lights have on the surface at the hit point
        let mut all_light = vec3![0.0, 0.0, 0.0];
        for l in scene.get_lights().iter() {
            all_light += l.get_light_color(hit.hit_point);
        }
        all_light + self.albedo
    }
}

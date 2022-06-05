use crate::{Ray, Scene, Vec3, vec3, WorldLight};
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
    fn get_color(&self, scene: &Scene, _: Ray, hit: &HitData, _: u32) -> Vec3 {
        // Collect the effects all lights have on the surface at the hit point
        let mut all_light = vec3![0.0, 0.0, 0.0];
        for l in scene.get_lights().iter() {
            all_light += l.get_light_color(hit.hit_point);
        }
        all_light + self.albedo
    }
}

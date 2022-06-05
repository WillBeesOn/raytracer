use crate::{Ray, Scene, Vec3, vec3, WorldLight};
use crate::data_structures::Vector;
use crate::materials::Mat;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Phong {
    albedo: Vec3,
    diffuse_factor: f64,
    specular_factor: f64,
    shine_factor: f64,
    reflect_factor: f64,
    ambient_factor: f64
}

impl Phong {
    pub fn new(albedo: Vec3, diffuse_factor: f64, specular_factor: f64, shine_factor: f64, reflect_factor: f64, ambient_factor: f64) -> Self {
        Phong { albedo, diffuse_factor, specular_factor, shine_factor, reflect_factor, ambient_factor }
    }
}

impl Mat for Phong {
    // Gets the color for a point on a surface using the Phong shading model.
    // diffuse_factor * diffuse + specular_factor * specular
    fn get_color(&self, scene: &Scene, _incoming_ray: Ray, hit: &HitData, _reflect_depth: u32) -> Vec3 {
        let mut final_diffuse = vec3![0.0, 0.0, 0.0];
        let mut final_specular = vec3![0.0, 0.0, 0.0];

        for light in scene.get_lights() {
            let light_color = light.get_light_color(hit.hit_point);
            let light_dir = (light.get_position() - hit.hit_point).unit();

            // Calculate diffuse component. Determines how matte the visible color of the surface is. Lower = less shiny, more matte.
            let light_dot_norm = light_dir.dot(hit.normal).max(0.0);
            final_diffuse += light_color * light_dot_norm ;

            // Calculate specular component. Determines how far the specular highlight reaches.
            // Specular component is: specular factor * light color and intensity * (reflect dir dot view dir)^shininess component
            let reflect = (2.0 * light_dot_norm * hit.normal) - light_dir; // r = 2 * (norm dot light dir) * norm) - light dir
            let view_dir = (hit.ray.origin - hit.hit_point).unit();
            let specular = view_dir.dot(reflect).max(0.0).powf(self.shine_factor);
            final_specular += light_color * specular;
        }

        self.albedo * (self.diffuse_factor * final_diffuse + self.specular_factor * final_specular + self.ambient_factor)
    }
}

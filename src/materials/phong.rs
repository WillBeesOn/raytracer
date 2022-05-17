use crate::{Vec3, vec3, WorldLight};
use crate::data_structures::Vector;
use crate::materials::Mat;
use crate::objects::Light;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Phong {
    albedo: Vec3,
    diffuse_factor: f64,
    specular_factor: f64,
    shine_factor: f64,
    ambient_factor: f64
}

impl Phong {
    pub fn new(albedo: Vec3, diffuse_factor: f64, specular_factor: f64, shine_factor: f64, ambient_factor: f64) -> Self {
        Phong { albedo, diffuse_factor, specular_factor, shine_factor, ambient_factor }
    }
}

impl Mat for Phong {
    // Gets the color for a point on a surface using the Phong shading model.
    // diffuse_factor * diffuse + specular_factor * specular
    fn get_color(&self, lights: &Vec<Light>, hit: &HitData) -> Vec3 {
        let mut final_diffuse = vec3![0.0, 0.0, 0.0];
        let mut final_specular = vec3![0.0, 0.0, 0.0];

        for light in lights {
            let light_color = light.get_light_color(hit.hit_point);
            let light_dir = (light.get_position() - hit.hit_point).unit();

            // Calculate diffuse component. Determines how matte the visible color of the surface is. Lower = less shiny, more matte.
            let light_dot_norm = light_dir.dot(hit.normal).max(0.0);
            final_diffuse += light_color * light_dot_norm ;

            // Calculate specular component. Determines how far the specular highlight reaches.
            let reflect = (2.0 * light_dot_norm * hit.normal) - light_dir;
            let view_dir = (hit.ray.origin - hit.hit_point).unit();
            let specular = view_dir.dot(reflect).max(0.0).powf(self.shine_factor);
            final_specular += light_color * specular;
        }
        // TODO need to change ambient... Well, do I really?
        //  An ambient light will have it's own intensity, and the object has it's own factor which determines how much the ambient light affects it.
        //  However, the "correct" way is ultimately a single scalar, so why can't this be valid? The single scalar would have the ambient light intensity and the object's ambient factor encoded in it.
        self.albedo * (self.diffuse_factor * final_diffuse + self.specular_factor * final_specular + self.ambient_factor)
    }

    fn set_color(&mut self, color: Vec3) {
        self.albedo = color;
    }
}

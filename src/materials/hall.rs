use crate::{Ray, Scene, Vec3, vec3, WorldLight};
use crate::data_structures::Vector;
use crate::materials::Mat;
use crate::objects::Light;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Hall {
    d_color: Vec3,
    s_color: Vec3,
    diffuse_factor: f64,
    specular_factor: f64,
    reflect_factor: f64,
    shine_factor: f64,
    ambient_factor: f64
}

impl Hall {
    pub fn new(
        d_color: Vec3, s_color: Vec3,
        diffuse_factor: f64, specular_factor: f64,
        reflect_factor: f64, shine_factor: f64, ambient_factor: f64
    ) -> Self {
        Hall {
            d_color, s_color,
            diffuse_factor, specular_factor,
            reflect_factor, shine_factor, ambient_factor
        }
    }
}

impl Mat for Hall {
    // Gets the color for a point on a surface using the Hall shading model.
    // diffuse_factor * diffuse + specular_factor * specular
    fn get_color(&self, scene: &Scene, incoming_ray: Ray, hit: &HitData, reflect_depth: u32) -> Vec3 {
        let mut final_diffuse = vec3![0.0, 0.0, 0.0];
        let mut final_specular = vec3![0.0, 0.0, 0.0];

        for light in scene.get_lights() {
            let light_color = light.get_light_color(hit.hit_point);
            let light_dir = (light.get_position() - hit.hit_point).unit();

            // Calculate diffuse component. Determines how matte the visible color of the surface is. Lower = less shiny, more matte.
            let light_dot_norm = light_dir.dot(hit.normal).max(0.0);
            final_diffuse += light_color * light_dot_norm ;

            // Calculate specular component. Determines how far the specular highlight reaches.
            // Specular component is: specular factor * light color and intensity * (normal dir dot half vec dir)^shininess component
            let view_dir = (hit.ray.origin - hit.hit_point).unit();
            let light_plus_view = light_dir + view_dir;
            let half = light_plus_view / light_plus_view.length(); // half = (light dir + view dir) / (light dir + view dir).length
            let specular = hit.normal.dot(half).max(0.0).powf(self.shine_factor);
            final_specular += light_color * specular;
        }

        // Calculate ambient light
        let mut ambient = vec3![0.0, 0.0, 0.0];
        for a_light in scene.get_ambient_lights() {
            ambient += self.ambient_factor * a_light.intensity;
        }

        // If this material reflects and the max reflect depth hasn't been met, then reflect.
        let mut reflection = vec3![0.0, 0.0, 0.0];
        if self.reflect_factor > 0.0 && reflect_depth < scene.get_reflect_depth() {
            let new_ray_origin = hit.hit_point + (hit.normal * 10e-6);
            let reflect_vec = incoming_ray.direction.reflect(hit.normal);
            let reflect_ray = Ray::new(new_ray_origin, reflect_vec);
            let reflected_color = scene.get_color_from_ray(reflect_ray, reflect_depth + 1) / 255.0;
            reflection = reflected_color * self.reflect_factor;
        }

        self.d_color * (self.diffuse_factor * final_diffuse + ambient) +
            self.s_color * (self.specular_factor * final_specular + reflection)
    }
}

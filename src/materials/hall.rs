use crate::{Ray, Scene, Vec3, vec3, WorldLight};
use crate::data_structures::Vector;
use crate::materials::Mat;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Hall {
    d_color: Vec3,
    s_color: Vec3,
    t_color: Vec3,
    diffuse_factor: f64,
    specular_factor: f64,
    reflect_factor: f64,
    transmissive_factor: f64,
    refractive_index: f64,
    shine_factor: f64,
    ambient_factor: f64
}

impl Hall {
    pub fn new(
        d_color: Vec3, s_color: Vec3, t_color: Vec3,
        diffuse_factor: f64, specular_factor: f64,
        reflect_factor: f64, transmissive_factor: f64,
        refractive_index: f64, shine_factor: f64, ambient_factor: f64
    ) -> Self {
        Hall {
            d_color, s_color, t_color,
            diffuse_factor, specular_factor,
            reflect_factor, transmissive_factor,
            refractive_index, shine_factor, ambient_factor
        }
    }
}

impl Mat for Hall {
    // Gets the color for a point on a surface using the Hall shading model.
    // diffuse_factor * diffuse + specular_factor * specular
    fn get_color(&self, scene: &Scene, incoming_ray: Ray, hit: &HitData, ray_depth: u32) -> Vec3 {
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

        let ray_depth_max = scene.get_ray_depth();

        // If this material reflects and the max reflect depth hasn't been met, then reflect.
        let mut reflection = vec3![0.0, 0.0, 0.0];
        if self.reflect_factor > 0.0 && ray_depth < ray_depth_max {
            let new_ray_origin = hit.hit_point + (hit.normal * 1e-6);
            let reflect_dir = incoming_ray.direction.reflect(hit.normal);
            let reflect_ray = Ray::new(new_ray_origin, reflect_dir);
            let reflected_color = scene.get_color_from_ray(reflect_ray, ray_depth + 1) / 255.0;
            reflection = reflected_color * self.reflect_factor;
        }

        // If this material refracts, then refract
        let mut refraction = vec3![0.0, 0.0, 0.0];
        if self.transmissive_factor > 0.0 && ray_depth < ray_depth_max {
            let mut refract_dir = vec3![0.0, 0.0, 0.0];
            // View and normal are opposite, meaning external ray is entering object
            if hit.ray.direction.dot(hit.normal) < 0.0 {
                let ratio = scene.get_refractive_index() / self.refractive_index;
                refract_dir = hit.ray.direction.refract(hit.normal, ratio);
            } else {
                // Otherwise, it is exiting object
                let ratio = self.refractive_index / scene.get_refractive_index();
                refract_dir = hit.ray.direction.refract(-1.0 * hit.normal, ratio);
            }

            let refracted_origin = hit.hit_point + refract_dir * 1e-6;
            let refracted_ray = Ray::new(refracted_origin, refract_dir);
            let refracted_color = scene.get_color_from_ray(refracted_ray, ray_depth + 1);
            refraction = refracted_color * self.transmissive_factor * (self.t_color / 255.0);
        }

        // Diffuse factor + Specular factor + Transmissive factor = 1.0
        self.d_color * (self.diffuse_factor * final_diffuse + ambient) +
            self.s_color * (self.specular_factor * final_specular + reflection) +
            refraction
    }
}

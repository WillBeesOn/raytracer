use crate::data_structures::{Ray, Vec3};
use crate::{HittableList, SceneObject, vec3, Vector};
use crate::materials::Material;
use crate::objects::BoundingVolume;
use crate::traits::{HitData, Hittable};

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    position: Vec3,
    scale: Vec3,
    radius: f64,
    material: Material
}

impl Sphere {
    pub fn new(radius: f64, material: Material) -> Self {
        Sphere {
            radius,
            material,
            position: vec3![0.0, 0.0, 0.0],
            scale: vec3![1.0, 1.0, 1.0]
        }
    }
}

impl SceneObject for Sphere {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn translate(&mut self, translation: Vec3) {
        self.position += translation
    }

    // Scaling is just changing the radius. Make it so it only scales if scale contains the same values
    fn scale(&mut self, scale: Vec3) {
        if scale.x == scale.y && scale.y == scale.z {
            self.radius = scale.x;
        }
    }

    // Doesn't really make sense to rotate sphere. Unless there's a texture on it.
    fn rotate(&mut self, _: Vec3) { }

    fn decompose(&self) -> HittableList {
        let mut list = HittableList::new();
        list.push(Box::new(self.clone()));
        list
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        let omc = ray.origin - self.position;  // Ray origin minus c (center) of sphere
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(omc);
        let c = omc.dot(omc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        // Edit the hit data passed to us with data about the raycast hit
        // If discriminant is 0 -> 1 root, positive -> 2 roots, negative -> 0 roots
        // Even if there's 1 root (intersects with edge of sphere) don't render it since we wouldn't be able to see it
        if discriminant > 0.0 {
            // Calculate 1st (closest to ray origin) t.
            let t0 = (-b - discriminant.sqrt()) / 2.0;
            let t0_hit = t_max > t0 && t0 > t_min;

            // Calculate 2nd (farther from ray origin) t.
            let t1 = (-b + discriminant.sqrt()) / 2.0;
            let t1_hit = t_max > t1 && t1 > t_min;

            if t0_hit || t1_hit {
                let t = if t0_hit { t0 } else { t1 };
                let hit = ray.get_point_at(t);
                return HitData::from(
                    t,
                    true,
                    ray,
                    hit,
                    (hit - self.position).unit(),
                    self.material
                );
            }
        }
        return HitData::new();
    }

    fn get_bounding_vol(&self) -> BoundingVolume {
        let r_vec = vec3![self.radius, self.radius, self.radius];
        BoundingVolume::new(self.position - r_vec, self.position + r_vec)
    }
}
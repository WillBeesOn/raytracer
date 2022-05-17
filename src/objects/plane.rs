use crate::{Hittable, Material, Ray, Vec3, vec3};
use crate::objects::BoundingVolume;
use crate::traits::HitData;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub position: Vec3,
    width: Vec3,
    height: Vec3,
    material: Material
}

impl Plane {
    // Creates a XZ plane aligned plane.
    pub fn new(width: f64, height: f64, material: Material) -> Self {
        Plane {
            material,
            position: vec3![0.0, 0.0, 0.0],
            width: vec3![width, 0.0, 0.0],
            height: vec3![0.0, 0.0, height]
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        let t = (self.position.y - ray.origin.y) / ray.direction.y;

        // If intersection in Y plan is outside of t range, ray does not intersect
        if t < t_min || t > t_max {
            return HitData::new();
        }

        let ray_intersect = ray.get_point_at(t);
        let top_left_corner = self.position - self.width / 2.0 - self.height / 2.0;
        let bottom_right_corner = self.position + self.width / 2.0 + self.height / 2.0;

        // If ray at point where it intersects along Y axis with plane does not also intersect with
        // X or Z
        if ray_intersect.x < top_left_corner.x || ray_intersect.x > bottom_right_corner.x ||
            ray_intersect.z < top_left_corner.z || ray_intersect.z > bottom_right_corner.z {
            return HitData::new();
        }

        return HitData::from(t, true, ray,ray_intersect, vec3![0.0, 1.0, 0.0], self.material);
    }

    fn get_bounding_vol(&self) -> BoundingVolume {
        todo!()
    }
}
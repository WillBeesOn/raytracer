use crate::data_structures::vec3::Vec3;
use crate::data_structures::Vector;

// Simple struct for a ray, represented by where it starts (origin) and where it's going (direction)
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray {
            origin: orig,
            direction: dir
        }
    }

    // Gets a point in scene space along the ray at time t
    // t = 0 --> origin of ray, t = 1 --> end of ray.
    pub fn get_point_at(&self, t: f64) -> Vec3 {
        self.origin.lerp(self.direction, t)
    }
}
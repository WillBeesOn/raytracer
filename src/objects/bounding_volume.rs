use std::cmp::Ordering::Equal;
use std::ops::Bound;
use crate::{Ray, Vec3, vec3};

// Represents a rectangular bounding volume for a 3D object.
#[derive(Debug, Copy, Clone)]
pub struct BoundingVolume {
    pub min: Vec3, // Typically bottom left corner farthest from camera
    pub max: Vec3 // Typically top right corner closest to camera
}

impl BoundingVolume {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        BoundingVolume { min, max }
    }

    // Test if a ray goes through this volume
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> bool {
        // For each axis (x, y, z) find the min and max intersection of ray with the box
        for axis in 0..3 {
            // First find the axis_t0 (near) and axis_t1 (far) along the chosen axis.
            let axis_min = (self.min[axis] - ray.origin[axis]) / ray.direction[axis];
            let axis_max = (self.max[axis] - ray.origin[axis]) / ray.direction[axis];
            let axis_t0 = axis_min.min(axis_max).max(t_min);
            let axis_t1 = axis_min.max(axis_max).min(t_max);

            // Ray doesn't intersect if axis_t0 is closer than axis_t1 or if axis_t1 is behind camera
            if axis_t0 > axis_t1 || axis_t1 < 0.0 {
                return false;
            }
        }
        // If the ray intersects with all axes of the box, then it does intersect.
        return true;
    }

    pub fn get_position(&self) -> Vec3 {
        self.min + (self.max - self.min) / 2.0
    }

    pub fn sort_by_axis(vols: &mut Vec<BoundingVolume>, axis: u8) {
        vols.sort_by(|a, b| {
            let a_pos = a.get_position();
            let b_pos = b.get_position();
            match axis {
                // Sort objects by position (centroid) along x axis
                0 => {
                    a_pos.x.partial_cmp(&b_pos.x).unwrap_or(Equal)
                },

                // Sort objects by position (centroid) along y axis
                1 => {
                    a_pos.y.partial_cmp(&b_pos.y).unwrap_or(Equal)
                },

                // Sort objects by position (centroid) along z axis
                2 => {
                    a_pos.z.partial_cmp(&b_pos.z).unwrap_or(Equal)
                },

                // Default case
                _ => Equal
            }
        });
    }
}
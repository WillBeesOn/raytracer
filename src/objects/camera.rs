use crate::data_structures::{Ray, Vec3, Vector};
use crate::{vec3};
use crate::utils::deg_to_rad;

pub struct Camera {
    pub position: Vec3,
    view_dir: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

// TODO make positionable camera
impl Camera {
    pub fn new(resolution: (u32, u32), hfov: f64) -> Self {
        let aspect_ratio = resolution.0 as f64 / resolution.1 as f64;
        let view_dir = vec3![0.0, 0.0, -1.0];
        let vertical = vec3![0.0, 1.0, 0.0];

        // Find the horizontal width of the view plane.
        // Have horizontal scale with aspect ratio
        let horiz= -2.0 * view_dir.z * (deg_to_rad(hfov / 2.0)).tan() * aspect_ratio;
        let horizontal = vec3![horiz, 0.0, 0.0];

        Camera {
            position : vec3![0.0, 0.0, 0.0],
            view_dir,
            horizontal,
            vertical,
        }
    }

    // Gets a ray from the camera position to some normalized pixel location in the scene
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        // Origin of the ray should be coming from the camera, so it's the camera origin.
        // The direction is from the camera origin to some pixel location in the canvas.
        Ray::new(
            self.position,
            (self.view_dir + self.horizontal * (x - 0.5) + self.vertical * (y - 0.5)).unit()
        )
    }
}
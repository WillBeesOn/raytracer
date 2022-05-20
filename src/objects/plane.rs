use crate::{Hittable, HittableList, Material, Ray, SceneObject, Vec3, vec3, Vector};
use crate::objects::BoundingVolume;
use crate::traits::HitData;
const PARALLEL_TOLERANCE: f64 = 1e-8;

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub position: Vec3,
    normal: Vec3,
    width: f64,
    height: f64,
    material: Material
}

impl Plane {
    // Creates a XZ plane aligned plane.
    pub fn new(width: f64, height: f64, normal: Vec3, material: Material) -> Self {
        Plane {
            material,
            width,
            height,
            position: vec3![0.0, 0.0, 0.0],
            normal: normal.unit()
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        // Check if denominator of ray-plane t equation is above 0 or parallel tolerance
        let denom = ray.direction.dot(self.normal);
        if denom.abs() > PARALLEL_TOLERANCE {
            // Finish calculating t if so and see if t is valid given the t bounds
            let t = self.normal.dot(self.position - ray.origin) / denom;
            if t > t_min && t < t_max{
                let ray_intersect = ray.get_point_at(t);
                return HitData::from(t, true, ray, ray_intersect, self.normal, self.material);
            }
        }
        HitData::new()
    }

    // Gets bounding box based on plane's normal orientation. Only does it for axis aligned orientations. Not ideal,
    // but it's good enough since I just want to display planes aligned with the axes.
    fn get_bounding_vol(&self) -> BoundingVolume {
        let mut far_bottom_left_corner = vec3![0.0, 0.0, 0.0];
        let mut near_top_right_corner = vec3![0.0, 0.0, 0.0];

        // YZ plane
        if self.normal.abs() == vec3![1.0, 0.0, 0.0] {
            far_bottom_left_corner = self.position - vec3![0.0, 0.0, self.width / 2.0] - vec3![0.0, self.height / 2.0, 0.0];
            near_top_right_corner = self.position + vec3![0.0, 0.0, self.width / 2.0] + vec3![0.0, self.height / 2.0, 0.0];
        }

        // XY plane
        if self.normal.abs() == vec3![0.0, 0.0, 1.0] {
            far_bottom_left_corner = self.position - vec3![self.width / 2.0, 0.0, 0.0] - vec3![0.0, self.height / 2.0, 0.0];
            near_top_right_corner = self.position + vec3![self.width / 2.0, 0.0, 0.0] + vec3![0.0, self.height / 2.0, 0.0];
        }

        // XZ plane
        if self.normal.abs() == vec3![0.0, 1.0, 0.0] {
            far_bottom_left_corner = self.position - vec3![self.width / 2.0, 0.0, 0.0] - vec3![0.0, 0.0, self.height / 2.0];
            near_top_right_corner = self.position + vec3![self.width / 2.0, 0.0, 0.0] + vec3![0.0, 0.0, self.height / 2.0];
        }

        BoundingVolume::new(far_bottom_left_corner, near_top_right_corner)
    }
}

impl SceneObject for Plane {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_material(&mut self, material: Material) {
        todo!()
    }

    fn translate(&mut self, translation: Vec3) {
        self.position += translation
    }

    fn scale(&mut self, scale: Vec3) {
        todo!()
    }

    fn rotate(&mut self, rotation: Vec3) {
        todo!()
    }

    fn decompose(&self) -> HittableList {
        let mut list = HittableList::new();
        list.push(Box::new(self.clone()));
        list
    }
}
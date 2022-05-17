use std::cmp::Ordering::Equal;
use std::ops;
use std::slice::Iter;
use crate::materials::{Flat, Material};
use crate::objects::{BoundingVolume, SceneObject};
use crate::{Ray, Vec3, vec3};


// Trait used for things that can be hit by a ray
pub trait Hittable: Sync + Send {
    // Returns data of an object between locations t_min and t_max along ray.
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData;

    // Calculate the bounding box of an object.
    fn get_bounding_vol(&self) -> BoundingVolume;
}

// Represents data about a raycast hit.
pub struct HitData {
    pub t: f64,  // The location along the ray where the object intersects
    pub did_hit: bool,  // If there was a hit
    pub ray: Ray,  // Ray that made the hit
    pub hit_point: Vec3,  // The point in space where the ray intersects with the object
    pub normal: Vec3,  // Normal of the point at which the ray hits the object
    pub mat: Material // The material of the object hit so we can render it appropriately.
}

impl HitData {
    // Create new hit data with default data
    pub fn new() -> Self{
        HitData {
            t: f64::MAX,
            hit_point: vec3![0.0, 0.0, 0.0],
            ray: Ray::new(vec3![0.0, 0.0, 0.0], vec3![0.0, 0.0, 0.0]),
            normal: vec3![0.0, 0.0, 0.0],
            mat: Material::Flat(Flat::new(vec3![0.5, 0.5, 0.5])),
            did_hit: false
        }
    }

    // Crete new hit data with known data.
    pub fn from(t: f64, did_hit: bool, ray: Ray, hit_point: Vec3, normal: Vec3, mat: Material) -> Self {
        HitData { t, did_hit, ray, hit_point, normal, mat }
    }
}

// A struct to hold a list of Hittable objects.
pub struct HittableList {
    data: Vec<Box<dyn SceneObject>>
}

impl Clone for HittableList {
    fn clone(&self) -> Self {
        HittableList {
            data: self.data.to_vec()
        }
    }
}

// Implements Hittable Trait for HittableList.
impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        let mut h_data = HitData::new(); // Create hit data to eventually send back
        let mut closest = t_max; // Start ray checking from farthest away

        // Iterate over all hittable items in list to find the closest one.
        for hittable in self.data.iter() {
            let try_hit = hittable.hit(ray, t_min, closest);
            // If there was a hit, update hit data to the hit object.
            // Also update the time at which the ray it the object to narrow our ray searches as we check other objects.
            if try_hit.did_hit {
                closest = try_hit.t;
                h_data = try_hit;
            }
        }
        h_data
    }

    // Gets the bounding volume surrounding all of the objects in the hittable list
    fn get_bounding_vol(&self) -> BoundingVolume {
        let mut vol_max = vec3![f64::MIN, f64::MIN, f64::MIN]; // Largest vec found amongst items in list
        let mut vol_min = vec3![f64::MAX, f64::MAX, f64::MAX]; // Smallest vec found amongst items in list
        for hittable in self.data.iter() {
            let v = hittable.get_bounding_vol();
            vol_max = vec3![vol_max.x.max(v.max.x), vol_max.y.max(v.max.y), vol_max.z.max(v.max.z)];
            vol_min = vec3![vol_min.x.min(v.min.x), vol_min.y.min(v.min.y), vol_min.z.min(v.min.z)];
        }
        BoundingVolume::new(vol_min, vol_max)
    }
}

impl ops::Index<usize> for HittableList {
    type Output = Box<dyn SceneObject>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { data: vec![] }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push(&mut self, hittable: Box<dyn SceneObject>) {
        self.data.push(hittable);
    }

    pub fn extend(&mut self, objects: HittableList) {
        self.data.extend(objects.data);
    }

    pub fn iter(&self) -> Iter<'_, Box<dyn SceneObject>> {
        self.data.iter()
    }

    pub fn get_slice_copy(&self, start: usize, end: usize) -> Self {
        HittableList {
            data: self.data[start..end].to_vec()
        }
    }

    pub fn sort_by_axis(&mut self, axis: u8) {
        self.data.sort_by(|a, b| {
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
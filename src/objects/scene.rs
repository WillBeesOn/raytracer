use std::collections::HashMap;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc;
use crate::materials::Mat;
use crate::{Camera, Hittable, HittableList, Ray, Vec3, vec3, Light, Material, Vector, WorldLight};
use crate::data_structures::{BvhNode};
use crate::traits::HitData;

const PARALLEL_TOLERANCE: f64 = 1e-8;


// Define a super trait for objects in the world.
// Objects in the world should be hittable by the ray as well as have these other common functions
pub trait SceneObject: Hittable + CloneSceneObject {
    fn get_position(&self) -> Vec3;
    fn set_material(&mut self, material: Material);
    fn translate(&mut self, translation: Vec3);
    fn scale(&mut self, scale: Vec3);
    fn rotate(&mut self, rotation: Vec3);
    fn decompose(&self) -> HittableList;
}

// A bunch of stuff ot make SceneObject copyable/cloneable
pub trait CloneSceneObject {
    fn clone_box(&self) -> Box<dyn SceneObject>;
}

impl<T: 'static + SceneObject + Clone> CloneSceneObject for T {
    fn clone_box(&self) -> Box<dyn SceneObject> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn SceneObject> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// Represents a scene to render.
// Stores all the objects that exist in the scene and the camera by which to view the scene.
pub struct Scene {
    pub main_camera: Camera,
    render_shadows: bool,
    acc_obj_num: u64,
    bvh_root: BvhNode,
    render_distance: f64,
    objects: HittableList,
    background_color: Vec3,
    lights: Vec<Light>,
    render_resolution: (u32, u32),
}

impl Scene {
    pub fn new(render_resolution: (u32, u32), render_distance: f64, background_color: Vec3, hfov: f64, acc_obj_num: u64, render_shadows: bool) -> Self {
        Scene {
            render_resolution,
            render_distance,
            background_color,
            acc_obj_num,
            render_shadows,
            bvh_root: BvhNode::new(),
            main_camera: Camera::new(render_resolution, hfov),
            lights: vec![],
            objects: HittableList::new()
        }
    }

    // Add renderable objects to scene.
    pub fn add_objects(&mut self, objects: Vec<Box<dyn SceneObject>>) {
        for obj in objects {
            self.push_object(obj);
        }
    }

    // Add a single renderable object to the scene.
    pub fn push_object(&mut self, obj: Box<dyn SceneObject>) {
        self.objects.push(obj);
    }

    pub fn push_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    // Given normalized pixel location return the color to render.
    pub fn get_color_at_pixel(&self, x: f64, y: f64) -> Vec3 {
        // Get the ray going from the camera origin to the chosen pixel location
        self.get_color_from_ray(self.main_camera.get_ray(x, y))
    }

    // Given a ray extending into the scene, get the color of the object that the ray intersects.
    fn get_color_from_ray(&self, ray: Ray) -> Vec3 {
        // Get the closest scene object that is hit by the ray. Optionally use BVH for acceleration.
        let mut hit = HitData::new();
        if self.acc_obj_num > 0 {
            hit = self.bvh_root.hit(ray, 0.0, self.render_distance);
        } else {
            hit = self.objects.hit(ray, 0.0, self.render_distance);
        }

        // If it hit something, return the color of the object.
        if hit.did_hit {
            let base_surface_color = hit.mat.get_color(&self.lights, &hit); // Base color of the surface with no shadows
            let mut final_color = base_surface_color; // Keep track of the final surface color

            // Compute shadows.
            if self.render_shadows {
                for light in &self.lights {
                    // Check if there are any objects between surface and all lights.
                    // Raise ray origin a bit outside the object in case rounding error puts the hit_point inside the object
                    let shadow_ray_origin = hit.hit_point + (hit.normal * 10e-6);
                    let hit_to_light = light.get_position() - hit.hit_point;
                    let shadow_ray = Ray::new(shadow_ray_origin, hit_to_light.unit());

                    // Ensure shadow is only rendered if surface is facing the light source.
                    // The color of the side of objects pointing away from the light source are computed
                    // just fine with the lighting code. Otherwise these areas get unnatural looking
                    // shadows from self-intersection
                    let norm_dot_shadow_dir = hit.normal.dot(shadow_ray.direction);
                    if norm_dot_shadow_dir <= PARALLEL_TOLERANCE || norm_dot_shadow_dir.abs() <= PARALLEL_TOLERANCE {
                        continue;
                    }

                    // Send the new ray in the direction of the light to find if there's anything in between,
                    // and only go as far as the light is.
                    let mut shadow_hit = HitData::new();
                    if self.acc_obj_num > 0 {
                        shadow_hit = self.bvh_root.hit(shadow_ray, 0.0, hit_to_light.length());
                    } else {
                        shadow_hit = self.objects.hit(shadow_ray, 0.0, hit_to_light.length());
                    }

                    // For each light the produces a shadow (something in between surface and light),
                    // subtract a portion of the original surface color to create the shadow
                    if shadow_hit.did_hit && shadow_ray.direction.dot(shadow_hit.normal) <= PARALLEL_TOLERANCE {
                        final_color -= 0.3 * base_surface_color;
                    }
                }
            }
            final_color
        } else {
            // If nothing hit, then return the background color.
            self.background_color
        }
    }

    // Renders a frame of the scene, rendering objects within camera view.
    pub fn render_frame(&mut self) -> Vec<Vec3> {
        if self.acc_obj_num > 0 {
            self.bvh_root = BvhNode::from(&mut self.objects, 0, self.acc_obj_num);
        }

        let mut pixels = vec![];
        let x = self.render_resolution.0;
        let y = self.render_resolution.1;
        for i in (0..y).rev() {
            for j in 0..x {
                let px = j as f64 / x as f64;
                let py = i as f64 / y as f64;
                pixels.push(self.get_color_at_pixel(px, py));
            }
        }
        pixels
    }

    // Same as render_frame but splits computing horizontal pixels into threads.
    pub fn render_frame_threaded(mut self) -> Vec<Vec3> {
        if self.acc_obj_num > 0 {
            self.bvh_root = BvhNode::from(&mut self.objects, 0, self.acc_obj_num);
        }

        let x = self.render_resolution.0;
        let y = self.render_resolution.1;
        let mut handles = vec![]; // Keep track of threads
        let self_ptr = Arc::new(self); // Turn self into a pointer
        let (tx, rx) = mpsc::channel(); // Create channel for collecting pixels

        // Create a thread for each row of pixels to compute.
        for i in (0..y).rev() {
            let self_clone = self_ptr.clone();
            let tx_clone = tx.clone();

            // Create a thread at each y position to render horizontal pixels for that y
            handles.push(thread::spawn(move || {
                let mut pix_row = vec![];
                for j in 0..x {
                    let px = j as f64 / x as f64;
                    let py = i as f64 / y as f64;
                    let color = self_clone.get_color_at_pixel(px, py);
                    pix_row.push(color); // Get the  color of pixel
                }
                // Send the index where this row should start, and of course the row of pixels.
                tx_clone.send((x * (y - i - 1), pix_row)).unwrap();
            }));
        }

        // Collect all rows
        let mut computed_pixels = vec![vec3![0.0, 0.0, 0.0]; (x * y) as usize];
        for _ in 0..y {
            let (index, row) = rx.recv().unwrap();
            computed_pixels.splice(index as usize..(index + x) as usize, row);
        }

        // Make sure all threads are joined.
        for h in handles {
            h.join().unwrap();
        }

        computed_pixels
    }

    // Does adaptive supersampling of image. Returns supersampled vec of pixels and vec of pixels representing heatmap of samples taken per pixel
    pub fn render_supersample_frame_threaded(mut self, tolerance: f64) -> (Vec<Vec3>, Vec<Vec3>) {
        if self.acc_obj_num > 0 {
            self.bvh_root = BvhNode::from(&mut self.objects, 0, self.acc_obj_num);
        }

        // Cache x and y since now since self ownership is transferred to the arc
        let x = self.render_resolution.0;
        let y = self.render_resolution.1;

        // Keep track of new rays and number of gets of cached color
        let mut rays_shot: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
        let mut cache_calls: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));

        let mut handles = vec![]; // Keep track of threads
        let self_ptr = Arc::new(self); // Turn self into a pointer
        let sample_map = Arc::new(Mutex::new(HashMap::new())); // Store sample taken
        let (tx, rx) = mpsc::channel(); // Create channel for collecting pixels

        // Create a thread for each row of pixels to compute.
        for i in (0..y).rev() {
            // Clone sender and pointers for each thread
            let tx_clone = tx.clone();
            let self_clone = self_ptr.clone();
            let map_clone = sample_map.clone();
            let mut ray_shot_clone = rays_shot.clone();
            let cache_calls_clone = cache_calls.clone();

            // Create a thread at each y position to render row of pixels
            handles.push(thread::spawn(move || {
                let mut pix_row = vec![]; // Row of pixels in final image
                let mut heatmap_row = vec![]; // Row of pixels for the ray/sample heatmap
                for j in 0..x {
                    // Supersample recursively with depth of 2
                    let (new_rays, new_cache_calls, color) = self_clone.supersample_recurse((j as f64, i as f64), 2.0, 0, 2, tolerance, (x, y), map_clone.clone());
                    pix_row.push(color);

                    // Record rays shot and cached samples taken
                    *ray_shot_clone.lock().unwrap().deref_mut() += new_rays;
                    *cache_calls_clone.lock().unwrap().deref_mut() += new_cache_calls;

                    // Get pixel for heatmap
                    let total_samples = (new_rays + new_cache_calls) as f64;
                    heatmap_row.push(vec3![total_samples, total_samples, total_samples]);
                }
                // Send the index where this row should start, and of course the row of pixels for the final image and heatmap.
                tx_clone.send((x * (y - i - 1), pix_row, heatmap_row)).unwrap();
            }));
        }

        // Collect all rows
        let mut computed_pixels = vec![vec3![0.0, 0.0, 0.0]; ((x) * (y)) as usize];
        let mut heatmap_pixels = vec![vec3![0.0, 0.0, 0.0]; ((x) * (y)) as usize];
        for _ in 0..y {
            let (index, row, heatmap_row) = rx.recv().unwrap();
            computed_pixels.splice(index as usize..(index + x) as usize, row);
            heatmap_pixels.splice(index as usize..(index + x) as usize, heatmap_row);
        }

        // Make sure all threads are joined.
        for h in handles {
            h.join().unwrap();
        }

        println!("Shot a total of {} unique rays.\nCalled ray cache {} times.", rays_shot.lock().unwrap(), cache_calls.lock().unwrap());
        (computed_pixels, heatmap_pixels)
    }


    fn supersample_recurse(
        &self, top_left: (f64, f64), corner_distance: f64, depth: u32, max_depth: u32, tolerance: f64, resolution: (u32, u32), sample_map: Arc<Mutex<HashMap<String, Vec3>>>
    ) -> (u64, u64, Vec3) {
        let x = resolution.0 as f64;
        let y = resolution.1 as f64;

        // Keep track of new rays and number of searches for cached color
        let mut rays_shot: u64 = 0;
        let mut cache_calls: u64 = 0;

        // Given top left corner of this pixel/subpixel at j, i  get the remaining corners
        let mut corner_colors = vec![];
        for corner_x in 0..=1 {
            for corner_y in 0..=1 {
                let corner_px = (top_left.0 + (corner_x as f64 * corner_distance)) / x;
                let corner_py = (top_left.1 - (corner_y as f64 * corner_distance)) / y;

                // If the map already has a sample for this position, don't compute it again.
                let key = format!("{}, {}", corner_px, corner_py);
                if !sample_map.lock().unwrap().contains_key(&key) {
                    let color = self.get_color_at_pixel(corner_px, corner_py);
                    rays_shot += 1;
                    corner_colors.push(color);
                    sample_map.lock().unwrap().insert(key, color);
                } else {
                    corner_colors.push(*sample_map.lock().unwrap().get(&key).unwrap());
                    cache_calls += 1;
                }
            }
        }

        // Check tolerances
        let above_tol1 = corner_colors[0].percent_diff(corner_colors[1]) > tolerance;
        let above_tol2 = corner_colors[0].percent_diff(corner_colors[2]) > tolerance;
        let above_tol3 = corner_colors[3].percent_diff(corner_colors[1]) > tolerance;
        let above_tol4 = corner_colors[3].percent_diff(corner_colors[2]) > tolerance;

        // Recursively do AA if any tolerance is violated and the max depth hasn't been met.
        if (depth <= max_depth) && (above_tol1 || above_tol2 || above_tol3 || above_tol4) {
            // For each subpixel, change the top left corner starting from, how far the other corners are, and the depth.
            let new_corner_distance = corner_distance / 2.0;
            let new_depth = depth + 1;

            // Recursive calls. Too lazy to do a loop or something for these...
            let (new_rays, new_cache_calls, color) = self.supersample_recurse(top_left, new_corner_distance, new_depth, max_depth, tolerance, resolution, sample_map.clone());
            rays_shot += new_rays;
            cache_calls += new_cache_calls;
            corner_colors[0] = color;

            let (new_rays, new_cache_calls, color) = self.supersample_recurse((top_left.0 + 1.0, top_left.1), new_corner_distance, new_depth, max_depth, tolerance, resolution, sample_map.clone());
            rays_shot += new_rays;
            cache_calls += new_cache_calls;
            corner_colors[1] = color;

            let (new_rays, new_cache_calls, color) = self.supersample_recurse( (top_left.0, top_left.1 - 1.0), new_corner_distance, new_depth, max_depth, tolerance, resolution, sample_map.clone());
            rays_shot += new_rays;
            cache_calls += new_cache_calls;
            corner_colors[2] = color;

            let (new_rays, new_cache_calls, color) = self.supersample_recurse( (top_left.0 + 1.0, top_left.1 - 1.0), new_corner_distance, new_depth, max_depth, tolerance, resolution, sample_map.clone());
            rays_shot += new_rays;
            cache_calls += new_cache_calls;
            corner_colors[3] = color;
        }

        (rays_shot, cache_calls, (corner_colors[0] + corner_colors[1] + corner_colors[2] + corner_colors[3]) / 4.0)
    }
}
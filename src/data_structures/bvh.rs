use crate::objects::BoundingVolume;
use crate::{Hittable, HittableList, Ray, Vec3, vec3};
use crate::traits::HitData;

pub struct BvhNode {
    pub objects: HittableList,
    pub volume: BoundingVolume,
    pub children: Vec<BvhNode>
}

impl BvhNode {
    pub fn new() -> Self {
        BvhNode {
            objects: HittableList::new(),
            volume: BoundingVolume::new(vec3![0.0, 0.0, 0.0], vec3![0.0, 0.0, 0.0]),
            children: vec![]
        }
    }


    // Entrypoint for creating a BVH root node
    pub fn from(object_list: &mut HittableList, axis: u8, threshold: u64) -> Self {
        let mut all_objects = HittableList::new();
        // Go through all SceneObjects and decompose them into their constituent parts.
        // Really only relevant for TriangleMesh since decompose will return a list of all the triangles
        // and for a sphere it just returns a list containing the single sphere.
        for obj in object_list.iter() {
            all_objects.extend(obj.decompose());
        }

        // Build and return the BVH tree with all triangles + other implicitly rendered objects
        BvhNode::build_node_recursive(&mut all_objects, axis, threshold)
    }

    fn build_node_recursive(object_list: &mut HittableList, axis: u8, threshold: u64) -> Self {
        let mut new_node = BvhNode {
            objects: HittableList::new(),
            volume: object_list.get_bounding_vol(),
            children: vec![]
        };
        let list_len = object_list.len();

        // Stop if the threshold requirement is met
        if list_len as u64 <= threshold {
            new_node.objects = object_list.clone();
            return new_node;
        } else {
            // Sort all objects along some axis so it can be split later
            object_list.sort_by_axis(axis % 3);

            // Split BVH into a 4 degree tree.
            // Need to split objects into 4, and then spread remainder across the existing partitions.
            let tree_degree = 2.0; // If changing this, make sure to change obj_per_list
            let split = (list_len as f64 / tree_degree).floor() as usize;
            let mut obj_per_list = [split; 2];
            let mut obj_remainder = list_len % split;
            let remainder_per_list = (obj_remainder as f64 / tree_degree).ceil() as usize;

            let mut start = 0;  // Index where in object_list to start extracting objects
            let mut end = 0; // Index where in object_list to stop extracting objects
            for i in 0..tree_degree as usize {
                let mut amount_to_add = remainder_per_list;
                if obj_remainder > 0 {
                    // If adding the last bit of remainder makes it use more than really remains,
                    // add the difference so we don't add more objects to a list than there are
                    if (obj_remainder as i64 - amount_to_add as i64) < 0 {
                        amount_to_add += obj_remainder - amount_to_add;
                    }
                    obj_remainder -= amount_to_add;
                } else {
                    amount_to_add = 0;
                }

                // Adjust start and end for building slices of original object list
                obj_per_list[i] += amount_to_add;
                end += obj_per_list[i];
                if i > 0 {
                    start += obj_per_list[i - 1];
                }

                new_node.children.push(BvhNode::build_node_recursive(
                        &mut object_list.get_slice_copy(start, end),
                        axis + 1,
                        threshold
                    ));
            }
        }
        new_node
    }

    // Returns hit data for an object that the ray intersects with in the bounding volume hierarchy
    // Recurse through tree to find an object that the ray hits
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        // Check if the ray hits this node's volume
        let did_hit = self.volume.hit(ray, t_min, t_max);

        // Initialize the closest hit. Which of course doesn't hit and t is at the max distance.
        let mut closest_hit = HitData::new();

        // If this node's volume was hit and it has children, check the child nodes for hits
        if did_hit && self.children.len() > 0 {
            // Find the direction the ray points in the most
            let mut axis: usize = 0;
            let mut highest_dir: Option<f64> = None;
            for i in 0..3 {
                if let Some(highest) = highest_dir {
                    if highest < ray.direction[i].abs() {
                        highest_dir = Some(ray.direction[i].abs());
                        axis = i;
                    }
                } else {
                    highest_dir = Some(ray.direction[i].abs());
                    axis = i;
                }
            }

            // Hacky way of sorting child BVH nodes, not great. Making self mutable is too much work for now, so this is good enough.
            let mut vols = vec![];
            for child in self.children.iter() {
                vols.push(child.volume.clone());
            }
            let mut vol_clone = vols.clone();
            BoundingVolume::sort_by_axis(&mut vol_clone, axis as u8);

            // Reverse the order of checking children if volume sorting changed the order
            let reverse_order = vols[0].min == vol_clone[0].min;

            // Go through each child and get the hit data. If the ray hit something while going down the
            // tree, check to see if it is the closest.
            if reverse_order {
                for child in self.children.iter().rev() {
                    let hit = child.hit(ray, t_min, t_max);
                    if hit.did_hit && hit.t < closest_hit.t {
                        closest_hit = hit;
                    }
                }
            } else {
                for child in self.children.iter() {
                    let hit = child.hit(ray, t_min, t_max);
                    if hit.did_hit && hit.t < closest_hit.t {
                        closest_hit = hit;
                    }
                }
            };
        }


        // If this a leaf node (no child nodes) then test the objects in this node for intersection.
        if did_hit && self.children.len() == 0 {
            closest_hit = self.objects.hit(ray, t_min, t_max);
        }

        // Return the hit we found (if any)
        closest_hit
    }
}


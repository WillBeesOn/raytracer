use crate::data_structures::Matrix4;
use crate::{Hittable, HittableList, Material, Ray, SceneObject, Vec3, vec3, Vector};
use crate::objects::BoundingVolume;
use crate::traits::HitData;

const PARALLEL_TOLERANCE: f64 = 1e-8;

// Struct representing a single triangle to be used in a triangle mesh.
pub struct Triangle {
    pub vertex_normals: Matrix4,
    surface_normal: Vec3,
    material: Material,
    vertices: Matrix4,
    smooth: bool
}

impl Triangle {
    pub fn new(surface_normal: Vec3, vertex_normals: Matrix4, vertices: Matrix4, material: Material, smooth: bool) -> Self {
        Triangle { surface_normal, vertex_normals, vertices, material, smooth }
    }

    // TODO applying transformation to normals isn't quite right. Need to use transpose of inverse of transformation matrix.
    pub fn transform(&mut self, transformation: &Matrix4, transform_normals: bool) {
        self.vertices = &self.vertices * transformation;
        if transform_normals {
            let for_normals = transformation.inverse().transpose_square();
            self.vertex_normals = &self.vertex_normals * &for_normals;
            self.surface_normal *= &for_normals;
        }
    }
}

impl SceneObject for Triangle {
    fn get_position(&self) -> Vec3 {
        let v1 = self.vertices[0];
        let v2 = self.vertices[1];
        let v3 = self.vertices[2];
        vec3![
            (v1.x + v2.x + v3.x) / 3.0,
            (v1.y + v2.y + v3.y) / 3.0,
            (v1.z + v2.z + v3.z) / 3.0
        ]
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn translate(&mut self, translation: Vec3) {
        let to_vec4 = translation.to_vec4(0.0);
        self.vertices[0] += to_vec4;
        self.vertices[1] += to_vec4;
        self.vertices[2] += to_vec4;
    }

    fn scale(&mut self, scale: Vec3) {
        let to_vec4 = scale.to_vec4(1.0);
        self.vertices[0] *= to_vec4;
        self.vertices[1] *= to_vec4;
        self.vertices[2] *= to_vec4;
    }

    // Don't bother with this, it'll be handled in transform function
    fn rotate(&mut self, rotation: Vec3) { }

    fn decompose(&self) -> HittableList {
        let mut list = HittableList::new();
        list.push(Box::new(self.clone()));
        list
    }
}

impl Clone for Triangle {
    fn clone(&self) -> Self {
        Triangle {
            vertex_normals: self.vertex_normals.clone(),
            surface_normal: self.surface_normal,
            material: self.material,
            vertices: self.vertices.clone(),
            smooth: self.smooth
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        let mut found_hit = HitData::new();

        // Return if surface normal is in same direction of ray. It's a backfacing triangle.
        if self.surface_normal.dot(ray.direction) >= PARALLEL_TOLERANCE {
            return found_hit;
        }

        let vert1 = self.vertices[0].to_vec3();
        let vert2 = self.vertices[1].to_vec3();
        let vert3 = self.vertices[2].to_vec3();
        let edge1 = vert2 - vert1;
        let edge2 = vert3 - vert1;

        // Matrix A from notes is just edge1, edge2, ray.direction transposed.
        // Determinant can actually be calculated using crosses and dots.
        let col3_x_col2_a = ray.direction.cross(edge2);
        let det_a = edge1.dot(col3_x_col2_a);

        // If det is negative, tri is backfacing. Or if det is close to 0, it is parallel.
        // If any of these cases is true, we do not want to render it. Move on to next triangle
        if det_a < PARALLEL_TOLERANCE || det_a.abs() < PARALLEL_TOLERANCE {
            return found_hit;
        }

        // Setting up the numerator determinant to calculate t
        let col3_x_col2_t = (vert1 - ray.origin).cross(edge2);
        let det_t = edge1.dot(col3_x_col2_t);
        let t = det_t / det_a;

        // No hit if t is outside render distance
        if t < t_min || t > t_max {
            return found_hit;
        }

        // Setting up the numerator determinant to calculate beta
        let vert1_origin = ray.origin - vert1;
        let det_b = vert1_origin.dot(col3_x_col2_a);
        let b = det_b / det_a;

        // Setting up the numerator determinant to calculate gamma
        let col3_x_col2_g = ray.direction.cross(vert1_origin);
        let det_g = edge1.dot(col3_x_col2_g);
        let g = det_g / det_a;

        // Check if beta and gamma are inside tri
        if b > 0.0 && g > 0.0 && b + g <= 1.0 {
            // Only set t for the found hit to the computed if the hit point is indeed in the triangle
            found_hit.t = t;
            found_hit.hit_point = ray.get_point_at(t);
            found_hit.did_hit = true;
            found_hit.ray = ray;
            found_hit.mat = self.material;

            // Choose which normal to use: triangle surface normal or interpolated normal at the hit point
            if self.smooth {
                // Get last barycentric coordinate
                let a = 1.0 - b - g;

                // Interpolate hit point normal with vertex normals and barycentric coordinates
                found_hit.normal = (self.vertex_normals[2] * g + self.vertex_normals[0] * a + self.vertex_normals[1] * b).to_vec3();
            } else {
                found_hit.normal = self.surface_normal;
            }
        }
        return found_hit;
    }

    fn get_bounding_vol(&self) -> BoundingVolume {
        let v0 = self.vertices[0];
        let v1 = self.vertices[1];
        let v2 = self.vertices[2];
        BoundingVolume {
            min: vec3![v0.x.min(v1.x).min(v2.x), v0.y.min(v1.y).min(v2.y), v0.z.min(v1.z).min(v2.z)],
            max: vec3![v0.x.max(v1.x).max(v2.x), v0.y.max(v1.y).max(v2.y), v0.z.max(v1.z).max(v2.z)]
        }
    }
}

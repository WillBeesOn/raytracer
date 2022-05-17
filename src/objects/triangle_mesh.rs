use crate::data_structures::{Matrix4, Ray, Vec3, Vec4, Vector};
use crate::{HittableList, matrix4, SceneObject, vec3};
use crate::traits::Hittable;
use crate::materials::{Material, Flat};
use crate::objects::{BoundingVolume, Triangle};
use crate::traits::HitData;
use crate::utils::{deg_to_rad, scale_m, rotate_m, translate_m};

pub struct TriangleMesh {
    pub smooth: bool,
    triangles: Vec<Triangle>,
    position: Vec3,
    scale: Vec3,
    rotation: Vec3,
    material: Material
}

impl TriangleMesh {
    pub fn new(vertices: Matrix4, faces: Vec<Vec3>, smooth: bool) -> Self {
        let material = Material::Flat(Flat::new(vec3![0.5, 0.5, 0.5]));
        let mut triangles = vec![];
        let mut vertex_normals = vec![vec3![0.0, 0.0, 0.0]; vertices.len()];  // Keep track of vert norms
        let mut vertex_indices = vec![]; // Keep track of vertex indices for each triangle to later assign the correct vertex normals
        let mut vn_div_by = vec![0.0; vertices.len()];  // Keep track of number of surface norms making a vert norm to average it later

        // Each face contains the indices of the associated vertices.
        for face in faces.iter() {
            // Get the indices of the vertices in the face
            let v1_i = face.x as usize - 1;
            let v2_i = face.y as usize - 1;
            let v3_i = face.z as usize - 1;

            // Construct a matrix out of the vertices for easier transformation
            let tri_verts = matrix4![
                vertices[v1_i].to_vec(),
                vertices[v2_i].to_vec(),
                vertices[v3_i].to_vec()
            ];

            // Get the surface normal of the triangle.
            let surface_normal = ((tri_verts[1] - tri_verts[0]).to_vec3()).cross((tri_verts[2] - tri_verts[0]).to_vec3());

            // Accumulate the surface normals for each vertex.
            // For each triangle a vertex is a part of, the vertex normal is a sum of the surface normals.
            vertex_normals[v1_i] += surface_normal;
            vertex_normals[v2_i] += surface_normal;
            vertex_normals[v3_i] += surface_normal;
            vn_div_by[v1_i] += 1.0;
            vn_div_by[v2_i] += 1.0;
            vn_div_by[v3_i] += 1.0;

            vertex_indices.push(vec![v1_i, v2_i, v3_i]);
            triangles.push(Triangle::new(
                surface_normal.unit(),
                Matrix4::new(),
                tri_verts,
                material,
                smooth
            ));
        }

        // Make each vertex normal truly normal
        for (i, vn) in vertex_normals.iter_mut().enumerate() {
            *vn = (*vn / vn_div_by[i]).unit();
        }

        // Assign vertex normals to triangles
        for (i, tri) in triangles.iter_mut().enumerate() {
            tri.vertex_normals = Matrix4::from(vec![
                vertex_normals[vertex_indices[i][0]].to_vec4(1.0),
                vertex_normals[vertex_indices[i][1]].to_vec4(1.0),
                vertex_normals[vertex_indices[i][2]].to_vec4(1.0)
            ]);
        }

        TriangleMesh {
            triangles,
            smooth,
            material,
            scale: vec3![1.0, 1.0, 1.0],
            position: vec3![0.0, 0.0, 0.0],
            rotation: vec3![0.0, 0.0, 0.0]
        }
    }

    fn transform(&mut self, transformation_m: &Matrix4, transform_normals: bool) {
        for tri in self.triangles.iter_mut() {
            tri.transform(transformation_m, transform_normals)
        }
    }
}

impl SceneObject for TriangleMesh {
    fn get_position(&self) -> Vec3 {
        self.position
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
        for tri in self.triangles.iter_mut() {
            tri.set_material(material);
        }
    }

    fn translate(&mut self, translation: Vec3) {
        self.position += translation;
        for tri in self.triangles.iter_mut() {
            tri.translate(translation);
        }
    }

    fn scale(&mut self, scale: Vec3) {

        // First scale triangles to original scale
        let revert_scale = vec3![1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z];
        for tri in self.triangles.iter_mut() {
            tri.scale(revert_scale);
        }

        // Then scale to scale parameter
        self.scale = scale;
        for tri in self.triangles.iter_mut() {
            tri.scale(scale);
        }
    }

    // TODO does making the transformations happen on the triangle individually affect how the rotation is handled?
    //  Having some issues with rotation also moving the object. Maybe since the old implementation was based on the mesh position,
    //  the new one needs to be based on the individual triangle position
    fn rotate(&mut self, rotation: Vec3) {
        self.rotation += rotation;
        let rotate_by = vec3![deg_to_rad(rotation.x), deg_to_rad(rotation.y), deg_to_rad(rotation.z)];

        // First translate to origin then apply rotation then translate back to original position.
        let rotation_m = (
            &(&translate_m(self.position) * &rotate_m(rotate_by)) *
                &translate_m(-1.0 * self.position)
        ).transpose_square();
        self.transform(&rotation_m, true);
    }

    // Return a hittable list containing all of the triangles of this mesh.
    fn decompose(&self) -> HittableList {
        let mut list = HittableList::new();
        for tri in self.triangles.iter() {
            list.push(Box::new(tri.clone()));
        }
        list
    }
}

impl Clone for TriangleMesh {
    fn clone(&self) -> Self {
        TriangleMesh {
            triangles: self.triangles.to_vec(),
            position: self.position,
            scale: self.scale,
            rotation: self.rotation,
            material: self.material,
            smooth: self.smooth
        }
    }
}

impl Hittable for TriangleMesh {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitData {
        let mut closest_hit = HitData::new();

        // Go through all triangles and find the closest one.
        for tri in self.triangles.iter() {
            let tri_hit = tri.hit(ray, t_min, closest_hit.t);
            if tri_hit.did_hit && tri_hit.t < closest_hit.t {
                closest_hit = tri_hit;
            }
        }
        closest_hit.mat = self.material;
        closest_hit
    }

    // Get bounding volume for this mesh, which is obtained from the bounding volumes of all the triangles in this mesh.
    fn get_bounding_vol(&self) -> BoundingVolume {
        let mut vol_max = vec3![f64::MIN, f64::MIN, f64::MIN]; // Largest vec found amongst items in list
        let mut vol_min = vec3![f64::MAX, f64::MAX, f64::MAX]; // Smallest vec found amongst items in list
        for tri in self.triangles.iter() {
            let v = tri.get_bounding_vol();
            vol_max = vec3![vol_max.x.max(v.max.x), vol_max.y.max(v.max.y), vol_max.z.max(v.max.z)];
            vol_min = vec3![vol_min.x.min(v.min.x), vol_min.y.min(v.min.y), vol_min.z.min(v.min.z)];
        }
        BoundingVolume::new(vol_min, vol_max)
    }
}
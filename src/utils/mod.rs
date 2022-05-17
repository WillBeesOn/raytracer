use std::f64::consts::PI;
use std::fs;
use image::{ImageBuffer, Rgb, RgbImage};
use crate::{matrix4, vec3, vec4};
use crate::data_structures::{Matrix4, Vec4, Vec3};
use crate::objects::TriangleMesh;

pub fn load_smf_mesh(filename: &str, smooth: bool) -> TriangleMesh {
    let text = fs::read_to_string(filename).expect("Error loading mesh.");
    let split = text.split('\n');

    let mut vertices = Matrix4::new();
    let mut faces: Vec<Vec3> = vec![];
    for s in split {
        let line_vec: Vec<String> = s.split(' ').map(str::to_string).collect();
        if line_vec.len() > 3 && line_vec[0].chars().nth(0) != Some('#') {
            if line_vec[0] == "v" {
                vertices.push(vec4![
                line_vec[1].parse().unwrap(),
                line_vec[2].parse().unwrap(),
                line_vec[3].parse().unwrap(),
                1.0
            ]);
            } else if line_vec[0] == "f" {
                faces.push(vec3![
                line_vec[1].parse().unwrap(),
                line_vec[2].parse().unwrap(),
                line_vec[3].parse().unwrap()
            ])
            }
        }
    }
    TriangleMesh::new(vertices, faces, smooth)
}

pub fn save_png(pixels: &Vec<Vec3>, resolution: (u32, u32), filename: &str) {
    let mut buffer: RgbImage = ImageBuffer::new(resolution.0, resolution.1);
    for (x, y, buf_pix) in buffer.enumerate_pixels_mut() {
        let computed_pix = pixels[(resolution.0 * y + x) as usize];
        *buf_pix = Rgb([computed_pix.x as u8, computed_pix.y as u8, computed_pix.z as u8]);
    }
    buffer.save(filename).unwrap();
}

pub fn deg_to_rad(d: f64) -> f64 {
    (d * PI) / 180.0
}

pub fn supersample(pixels: &Vec<Vec3>, resolution: (u32, u32), samples: u32) -> Vec<Vec3> {
    // Return empty pixels if samples isn't a square number or if resolution can fit
    let square = (samples as f64).sqrt();
    if square % 1.0 != 0.0 || resolution.0 as f64 % square != 0.0 || resolution.1 as f64 % square != 0.0 {
        return vec![];
    }

    let u_square = square as u32;
    let mut new_image = vec![];

    // Loop through the originally created image
    for i in 0..resolution.1 / u_square {  // Loop through Y (horizontal)
        for j in 0..resolution.0 / u_square {  // Loop through X (vertical)
            let offset = u_square * i * resolution.0 + j * u_square;
            let mut sampled_pixel = vec3![0.0, 0.0, 0.0];

            // Loop through the sample square
            for s1 in 0..u_square {  // Loop through vertical of sample square
                for s2 in 0..u_square {  // Loop through horizontal of sample square
                    let sample_index = offset + s1 * resolution.0 + s2;
                    sampled_pixel += pixels[sample_index as usize];
                }
            }

            new_image.push(sampled_pixel / samples as f64);
        }
    }
    return new_image;
}

// Get translation matrix, translating to a point
pub fn translate_m(v: Vec3) -> Matrix4 {
    matrix4![
        [1.0, 0.0, 0.0, v.x],
        [0.0, 1.0, 0.0, v.y],
        [0.0, 0.0, 1.0, v.z],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

// Get scale matrix
pub fn scale_m(v: Vec3) -> Matrix4 {
    matrix4![
        [v.x, 0.0, 0.0, 0.0],
        [0.0, v.y, 0.0, 0.0],
        [0.0, 0.0, v.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

// Get translation matrix, rotating along each axis
pub fn rotate_m(v: Vec3) -> Matrix4 {
    &(&matrix4![
        [1.0, 0.0, 0.0, 0.0],
        [0.0, v.x.cos(), -v.x.sin(), 0.0],
        [0.0, v.x.sin(), v.x.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ] * &matrix4![
        [v.y.cos(), 0.0, v.y.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-v.y.sin(), 0.0, v.y.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]) * &matrix4![
        [v.z.cos(), -v.z.sin(), 0.0, 0.0],
        [v.z.sin(), v.z.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
}

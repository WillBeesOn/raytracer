use std::time::Instant;

use crate::objects::{PointLight, Sphere, SceneObject, WorldLight, Light, Plane};
use crate::objects::{Camera, Scene};
use crate::data_structures::{Matrix4, Ray, Vec3, Vec4, Vector};
use crate::materials::{Flat, Material, Phong};
use crate::traits::{Hittable, HittableList};
use crate::utils::{load_smf_mesh, save_png, supersample};

mod objects;
mod data_structures;
mod traits;
mod materials;
mod utils;

// TODO
//  Create a scene that contains
//   - at least two spheres
//   - three non-box triangle meshes
//   - at least three lights, with one being white.
//   - Also add a "floor" that your shadows can be seen on.
//  Be sure that your scene is composed in a way that self-shadowing of one of the polygonal models is obvious.
fn main() {
    let dimension = 1024;
    let resolution = (dimension, dimension);
    let mut scene = Scene::new(
        resolution,
        100.0,
        vec3![0.0, 0.0, 0.0],
        60.0,
        50,
        true
    );

    // Scene 1 ===================
    //
    // let mut teapot = load_smf_mesh("models/teapot.smf", true);
    // teapot.translate(vec3![0.0, -1.2, -6.0]);
    // teapot.rotate(vec3![0.0, 10.0, 0.0]);
    // teapot.set_material(Material::Phong(Phong::new(vec3![102.0, 0.0, 102.0], 0.32, 0.9, 15.0 , 0.45)));
    //
    // let mut frog = load_smf_mesh("models/frog.smf", true);
    // frog.translate(vec3![-0.7, -1.58, -1.9]);
    // frog.scale(vec3![0.4, 0.45, 0.41]);
    // frog.rotate(vec3![10.0, 135.0, 10.0]);
    // frog.set_material(Material::Phong(Phong::new(vec3![35.0, 65.0, 245.0], 0.25, 1.0, 3.0 , 0.15)));
    //
    // let mut bunny = load_smf_mesh("models/bound-bunny_1k.smf", true);
    // bunny.translate( vec3![1.1, -0.59, -3.7]);
    // bunny.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny.set_material(Material::Phong(Phong::new(vec3![102.0, 0.0, 204.0], 0.1, 0.85, 15.0 , 0.15)));
    //
    // let mut sphere1 = Sphere::new(
    //     0.25,
    //     Material::Phong(Phong::new(vec3![247.0, 247.0, 36.0], 0.2, 0.75, 22.0 , 0.15))
    // );
    // sphere1.translate(vec3![0.0, 0.0, -2.7]);
    //
    // let mut sphere2 = Sphere::new(
    //     0.2,
    //     Material::Phong(Phong::new(vec3![0.0, 255.0, 128.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // sphere2.translate(vec3![-0.4, -0.4, -3.4]);
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.85));
    // light1.set_position(vec3![0.0, 5.0, -5.0]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.25));
    // light2.set_position(vec3![-5.0, 4.0, -8.0]);
    // scene.push_light(light2);
    //
    // let mut light3 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.95));
    // light3.set_position(vec3![5.0, 2.5, 2.0]);
    // scene.push_light(light3);
    //
    // let mut xz_plane = Plane::new(
    //     1.0, 1.0,
    //     vec3![0.0, 1.0, 0.0],
    //     Material::Flat(Flat::new(vec3![120.0, 120.0, 120.0]))
    // );
    // xz_plane.translate(vec3![0.0, -1.0, -5.0]);
    //
    // scene.add_objects(vec![
    //     Box::new(xz_plane),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(teapot),
    //     Box::new(frog),
    //     Box::new(bunny)
    // ]);


    // Scene 2 ===================


    // let mut cow = load_smf_mesh("models/cow.smf", true);
    // cow.translate(vec3![0.0, -0.68, -2.8]);
    // cow.set_material(Material::Phong(Phong::new(vec3![102.0, 0.0, 102.0], 0.25, 1.0, 10.0 , 0.25)));
    //
    // let mut bunny = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny.scale(vec3![0.55, 0.56, 0.44]);
    // bunny.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny.translate( vec3![0.43, -0.59, -1.8]);
    // bunny.set_material(Material::Phong(Phong::new(vec3![153.0, 76.0, 0.0], 0.1, 0.85, 15.0 , 0.15)));
    //
    // let mut campfire = load_smf_mesh("models/campfire.smf", true);
    // campfire.scale(vec3![0.003, 0.0031, 0.0029]);
    // campfire.rotate(vec3![-80.0, 0.0, 0.0]);
    // campfire.translate(vec3![-0.76, -0.9, -2.5]);
    // campfire.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 66.0], 0.35, 0.4, 20.0 , 0.15)));
    //
    //
    // let mut sphere1 = Sphere::new(
    //     0.25,
    //     Material::Phong(Phong::new(vec3![102.0, 102.0, 255.0], 0.2, 0.75, 30.0 , 0.15))
    // );
    // sphere1.translate(vec3![0.0, 0.7, -2.7]);
    //
    // let mut sphere2 = Sphere::new(
    //     0.2,
    //     Material::Phong(Phong::new(vec3![76.0, 153.0, 0.0], 0.2  , 0.35, 35.0, 0.15))
    // );
    // sphere2.translate(vec3![0.7, -0.2, -2.4]);
    //
    // let mut xz_plane = Plane::new(
    //     1.0, 1.0,
    //     vec3![0.0, 1.0, 0.0],
    //     Material::Phong(Phong::new(vec3![120.0, 120.0, 120.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // xz_plane.translate(vec3![0.0, -1.0, -5.0]);
    //
    // let mut left_yz_plane = Plane::new(
    //     10.0, 10.0,
    //     vec3![1.0, 0.0, 0.0],
    //     Material::Phong(Phong::new(vec3![77.0, 162.0, 219.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // left_yz_plane.translate(vec3![-1.3, 0.0, -5.0]);
    //
    // let mut right_yz_plane = Plane::new(
    //     10.0, 10.0,
    //     vec3![-1.0, 0.0, 0.0],
    //     Material::Phong(Phong::new(vec3![77.0, 162.0, 219.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // right_yz_plane.translate(vec3![1.3, 0.0, -5.0]);
    //
    // let mut back_cube = load_smf_mesh("models/box.smf", false);
    // back_cube.translate(vec3![0.0, 0.0, -6.0]);
    // back_cube.set_material(Material::Phong(Phong::new(vec3![77.0, 162.0, 219.0], 0.35, 0.4, 20.0 , 0.15)));
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 102.0], 0.06));
    // light1.set_position(vec3![0.9, -0.6, -1.0]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 51.0, 51.0], 0.15));
    // light2.set_position(vec3![-0.9, 1.0, 2.0]);
    // scene.push_light(light2);
    //
    // let mut light3 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.55));
    // light3.set_position(vec3![0.0, -0.7, 3.0]);
    // scene.push_light(light3);
    //
    // scene.add_objects(vec![
    //     Box::new(xz_plane),
    //     Box::new(left_yz_plane),
    //     Box::new(right_yz_plane),
    //     Box::new(back_cube),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(cow),
    //     Box::new(campfire),
    //     Box::new(bunny)
    // ]);

    // Scene 3=========================
    //
    // let mut dragon = load_smf_mesh("models/dragon-50000.smf", true);
    // dragon.translate(vec3![0.5, -0.9, -2.1]);
    // dragon.set_material(Material::Phong(Phong::new(vec3![102.0, 178.0, 255.0], 0.15, 1.0, 15.0 , 0.25)));
    //
    // let mut teapot = load_smf_mesh("models/teapot.smf", true);
    // teapot.scale(vec3![0.15, 0.16, 0.11]);
    // teapot.rotate(vec3![0.0, 0.0, 0.0]);
    // teapot.translate( vec3![-0.49, -0.88, -2.1]);
    // teapot.set_material(Material::Phong(Phong::new(vec3![153.0, 76.0, 0.0], 0.1, 0.85, 15.0 , 0.15)));
    //
    // let mut campfire = load_smf_mesh("models/campfire.smf", true);
    // campfire.scale(vec3![0.003, 0.0031, 0.0029]);
    // campfire.rotate(vec3![-80.0, 0.0, 0.0]);
    // campfire.translate(vec3![0.0, -0.8, -3.5]);
    // campfire.set_material(Material::Phong(Phong::new(vec3![255.0, 153.0, 51.0], 0.35, 0.4, 20.0 , 0.15)));
    //
    // let mut sphere1 = Sphere::new(
    //     0.25,
    //     Material::Phong(Phong::new(vec3![15.0, 122.0, 37.0], 0.2, 0.85, 30.0 , 0.15))
    // );
    // sphere1.translate(vec3![-0.7, 0.4, -4.2]);
    //
    // let mut sphere2 = Sphere::new(
    //     0.2,
    //     Material::Phong(Phong::new(vec3![76.0, 153.0, 0.0], 0.2  , 0.55, 50.0, 0.15))
    // );
    // sphere2.translate(vec3![0.25, 0.2, -2.9]);
    //
    // let mut xz_plane = Plane::new(
    //     1.0, 1.0,
    //     vec3![0.0, 1.0, 0.0],
    //     Material::Phong(Phong::new(vec3![120.0, 120.0, 120.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // xz_plane.translate(vec3![0.0, -0.88, -5.0]);
    //
    // let mut left_yz_plane = Plane::new(
    //     10.0, 10.0,
    //     vec3![1.0, 0.0, 0.0],
    //     Material::Phong(Phong::new(vec3![77.0, 162.0, 219.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // left_yz_plane.translate(vec3![-1.3, 0.0, -5.0]);
    //
    // let mut right_yz_plane = Plane::new(
    //     10.0, 10.0,
    //     vec3![-1.0, 0.0, 0.0],
    //     Material::Phong(Phong::new(vec3![77.0, 162.0, 219.0], 0.5  , 0.35, 35.0, 0.15))
    // );
    // right_yz_plane.translate(vec3![1.3, 0.0, -5.0]);
    //
    // let mut back_cube = load_smf_mesh("models/box.smf", false);
    // back_cube.translate(vec3![0.0, 0.0, -6.0]);
    // back_cube.set_material(Material::Phong(Phong::new(vec3![77.0, 162.0, 219.0], 0.35, 0.4, 20.0 , 0.15)));
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 102.0], 0.02));
    // light1.set_position(vec3![0.0, 0.2, -3.5]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 51.0, 51.0], 0.35));
    // light2.set_position(vec3![0.4, -0.5, 1.0]);
    // scene.push_light(light2);
    //
    // let mut light3 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.15));
    // light3.set_position(vec3![0.0, 5.0, -3.0]);
    // scene.push_light(light3);
    //
    // scene.add_objects(vec![
    //     Box::new(xz_plane),
    //     Box::new(left_yz_plane),
    //     Box::new(right_yz_plane),
    //     Box::new(back_cube),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(dragon),
    //     Box::new(campfire),
    //     Box::new(teapot)
    // ]);

    let now = Instant::now();
    let (computed, heatmap) = scene.render_supersample_frame_threaded(0.05);
    //let computed = scene.render_frame_threaded();
    println!("Took {:.2?} to render.", now.elapsed());
    save_png(&computed, resolution, "out.png");
    //save_png(&heatmap, resolution, "out_heatmap.png");

    // // Print out super sampled version of image
    // let samples = 4;
    // let square = (samples as f32).sqrt() as u32;
    // let sampled_resolution = (resolution.0 / square, resolution.1 / square);
    // let antialiased = supersample(&computed, resolution, samples);
    // save_png(&antialiased, sampled_resolution, "out_supersample.png");
}

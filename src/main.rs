use std::time::Instant;

use crate::objects::{SceneObject, WorldLight, Light, Sphere, PointLight, Plane, AmbientLight};
use crate::objects::{Camera, Scene};
use crate::data_structures::{Ray, Vec3, Vector};
use crate::materials::{Flat, Hall, Material, Phong};
use crate::traits::{Hittable, HittableList};
use crate::utils::{load_smf_mesh, save_png};

mod objects;
mod data_structures;
mod traits;
mod materials;
mod utils;

// TODO
//  Modify your shading equation to be more in line with the Hall model.
//  two spheres and three non-box triangle meshes, with at least one of each being reflective
//  at least two lights, with one being white.
//  Also add a "floor" that your shadows can be seen on
//  Be sure to position your objects in way that something is reflected in your reflective objects (at least one sphere and one mesh)
//  demonstrate that you can produce multiple reflections between objects.
//  Use adaptive supersampling with two levels of subdivision.

fn main() {
    let dimension = 1024;
    let resolution = (dimension, dimension);
    let mut scene = Scene::new(
        resolution,
        100.0,
        vec3![100.0, 100.0, 100.0],
        60.0,
        50,
        4,
        true
    );

    // Scene 1 ===================

    // let mut teapot = load_smf_mesh("models/teapot.smf", true);
    // teapot.scale(vec3![0.5, 0.5, 0.5]);
    // teapot.rotate(vec3![0.0, 140.0, 0.0]);
    // teapot.translate(vec3![0.0, 0.4, -4.0]);
    // teapot.set_material(Material::Hall(Hall::new(
    //     vec3![255.0, 128.0, 0.0], vec3![255.0, 128.0, 0.0],
    //     0.3, 0.7, 0.0, 200.0, 0.1)
    // ));
    // let mut frog = load_smf_mesh("models/frog.smf", true);
    // frog.scale(vec3![0.6, 0.6, 0.6]);
    // frog.rotate(vec3![10.0, 185.0, 10.0]);
    // frog.translate(vec3![1.0, 0.5, -3.0]);
    // frog.set_material(Material::Hall(Hall::new(
    //     vec3![35.0, 65.0, 235.0], vec3![35.0, 65.0, 235.0],
    //     0.1, 0.9, 0.0, 150.0, 0.1)
    // ));
    //
    // let mut bunny = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny.translate( vec3![-1.1, 1.0, -3.0]);
    // bunny.set_material(Material::Hall(Hall::new(
    //     vec3![102.0, 255.0, 178.0], vec3![102.0, 255.0, 178.0],
    //     0.2, 0.8, 0.0, 200.0, 0.1)
    // ));
    //
    // let mut sphere1 = Sphere::new(
    //     0.95,
    //     Material::Hall(Hall::new(
    //         vec3![178.0, 102.0, 255.0], vec3![178.0, 102.0, 255.0],
    //         0.25, 0.75, 0.0, 200.0, 0.1)
    //     )
    // );
    // sphere1.translate(vec3![0.0, -0.5, -6.0]);
    //
    // let mut sphere2 = Sphere::new(
    //     0.75,
    //     Material::Hall(Hall::new(
    //         vec3![102.0, 0.0, 0.0], vec3![192.0, 192.0, 192.0],
    //         0.4, 0.6, 0.0, 250.0, 0.1)
    //     )
    // );
    // sphere2.translate(vec3![-1.1, -0.35, -4.0]);
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.45)); // White
    // light1.set_position(vec3![0.0, 5.0, -4.0]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 153.0, 255.0], 0.05)); // Light pink
    // light2.set_position(vec3![1.0, 0.0, -1.0]);
    // scene.push_light(light2);
    //
    // scene.push_ambient_light(AmbientLight {
    //     intensity: vec3![1.0, 1.0, 1.0]
    // });
    //
    // let mut back_cube = load_smf_mesh("models/box.smf", false);
    // back_cube.scale(vec3![50.0, 50.0, 1.0]);
    // back_cube.translate(vec3![0.0, 0.0, -20.0]);
    // back_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut front_cube = load_smf_mesh("models/box.smf", false);
    // front_cube.scale(vec3![50.0, 50.0, 1.0]);
    // front_cube.translate(vec3![0.0, 0.0, 0.5]);
    // front_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut left_cube = load_smf_mesh("models/box.smf", false);
    // left_cube.scale(vec3![1.0, 50.0, 70.0]);
    // left_cube.rotate(vec3![0.0, 20.0, 0.0]);
    // left_cube.translate(vec3![-5.0, 0.0, -6.0]);
    // left_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut right_cube = load_smf_mesh("models/box.smf", false);
    // right_cube.scale(vec3![1.0, 50.0, 70.0]);
    // right_cube.rotate(vec3![0.0, -20.0, 0.0]);
    // right_cube.translate(vec3![5.0, 0.0, -6.0]);
    // right_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut bottom_cube = load_smf_mesh("models/box.smf", false);
    // bottom_cube.scale(vec3![50.0, 0.4, 70.0]);
    // bottom_cube.rotate(vec3![-10.0, 0.0, 0.0]);
    // bottom_cube.translate(vec3![0.0, -3.0, 0.0]);
    // bottom_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut top_cube = load_smf_mesh("models/box.smf", false);
    // top_cube.scale(vec3![50.0, 0.4, 50.0]);
    // top_cube.translate(vec3![0.0, 10.0, 0.0]);
    // top_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // scene.add_objects(vec![
    //     Box::new(bottom_cube),
    //     Box::new(top_cube),
    //     Box::new(left_cube),
    //     Box::new(right_cube),
    //     Box::new(back_cube),
    //     Box::new(front_cube),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(teapot),
    //     Box::new(frog),
    //     Box::new(bunny)
    // ]);


    // Scene 2 ===================
    let mut cow = load_smf_mesh("models/cow.smf", true);
    cow.scale(vec3![6.35, 6.35, 6.35]);
    cow.rotate(vec3![0.0, 140.0, 0.0]);
    cow.translate(vec3![0.0, -1.3, -8.0]);
    cow.set_material(Material::Hall(Hall::new(
        vec3![255.0, 255.0, 75.0], vec3![255.0, 255.0, 75.0],
        0.2, 0.8, 0.0, 200.0, 0.1)
    ));
    let mut frog = load_smf_mesh("models/frog.smf", true);
    frog.scale(vec3![1.4, 1.4, 1.4]);
    frog.rotate(vec3![10.0, 135.0, 10.0]);
    frog.translate(vec3![-1.2, 2.7, -9.0]);
    frog.set_material(Material::Hall(Hall::new(
        vec3![127.0, 0.0, 255.0], vec3![127.0, 0.0, 255.0],
        0.1, 0.9, 0.0, 100.0, 0.1)
    ));

    let mut bunny = load_smf_mesh("models/bound-bunny_5k.smf", true);
    bunny.rotate(vec3![0.0, 0.0, 0.0]);
    bunny.scale(vec3![1.4, 1.4, 1.4]);
    bunny.translate( vec3![0.0, -2.3, -7.1]);
    bunny.set_material(Material::Hall(Hall::new(
        vec3![202.0, 22.0, 22.0], vec3![202.0, 22.0, 22.0],
        0.2, 0.8, 0.0, 200.0, 0.1)
    ));

    let mut sphere1 = Sphere::new(
        1.95,
        Material::Hall(Hall::new(
            vec3![204.0, 255.0, 255.0], vec3![204.0, 255.0, 255.0],
            0.15, 0.85, 0.0, 350.0, 0.1)
        )
    );
    sphere1.translate(vec3![1.9, 2.4, -12.0]);

    let mut sphere2 = Sphere::new(
        0.65,
        Material::Hall(Hall::new(
            vec3![19.0, 64.0, 213.0], vec3![19.0, 64.0, 213.0],
            0.4, 0.6, 0.0, 150.0, 0.1)
        )
    );
    sphere2.translate(vec3![-2.0, -0.75, -6.4]);

    let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.4)); // White
    light1.set_position(vec3![0.0, 4.0, -7.0]);
    scene.push_light(light1);

    let mut light2 = Light::PointLight(PointLight::new(vec3![49.0, 192.0, 78.0], 0.95)); // Green
    light2.set_position(vec3![-1.5, -0.8, 0.0]);
    scene.push_light(light2);

    scene.push_ambient_light(AmbientLight {
        intensity: vec3![1.0, 1.0, 1.0]
    });

    let mut back_cube = load_smf_mesh("models/box.smf", false);
    back_cube.scale(vec3![50.0, 50.0, 1.0]);
    back_cube.translate(vec3![0.0, 0.0, -20.0]);
    back_cube.set_material(Material::Hall(Hall::new(
        vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
        0.4, 0.6, 0.0, 250.0, 0.1)
    ));

    let mut front_cube = load_smf_mesh("models/box.smf", false);
    front_cube.scale(vec3![50.0, 50.0, 1.0]);
    front_cube.translate(vec3![0.0, 0.0, 0.5]);
    front_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    let mut left_cube = load_smf_mesh("models/box.smf", false);
    left_cube.scale(vec3![1.0, 50.0, 70.0]);
    left_cube.rotate(vec3![0.0, 20.0, 0.0]);
    left_cube.translate(vec3![-5.0, 0.0, -6.0]);
    left_cube.set_material(Material::Hall(Hall::new(
        vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
        0.4, 0.6, 0.0, 250.0, 0.1)
    ));

    let mut right_cube = load_smf_mesh("models/box.smf", false);
    right_cube.scale(vec3![1.0, 50.0, 70.0]);
    right_cube.rotate(vec3![0.0, -20.0, 0.0]);
    right_cube.translate(vec3![5.0, 0.0, -6.0]);
    right_cube.set_material(Material::Hall(Hall::new(
        vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
        0.4, 0.6, 0.0, 250.0, 0.1)
    ));

    let mut bottom_cube = load_smf_mesh("models/box.smf", false);
    bottom_cube.scale(vec3![50.0, 0.4, 70.0]);
    bottom_cube.rotate(vec3![-10.0, 0.0, 0.0]);
    bottom_cube.translate(vec3![0.0, -3.0, 0.0]);
    bottom_cube.set_material(Material::Hall(Hall::new(
        vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
        0.7, 0.3, 0.0, 250.0, 0.1)
    ));

    let mut top_cube = load_smf_mesh("models/box.smf", false);
    top_cube.scale(vec3![70.0, 0.4, 70.0]);
    top_cube.translate(vec3![0.0, 10.0, 0.0]);
    top_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    scene.add_objects(vec![
        Box::new(bottom_cube),
        Box::new(top_cube),
        Box::new(left_cube),
        Box::new(right_cube),
        Box::new(back_cube),
        Box::new(front_cube),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(cow),
        Box::new(frog),
        Box::new(bunny)
    ]);

    // Scene 3=========================
    // let mut cow = load_smf_mesh("models/cow.smf", true);
    // cow.scale(vec3![6.35, 6.35, 6.35]);
    // cow.rotate(vec3![0.0, 140.0, 0.0]);
    // cow.translate(vec3![0.0, -1.3, -8.0]);
    // cow.set_material(Material::Hall(Hall::new(
    //     vec3![255.0, 255.0, 75.0], vec3![255.0, 255.0, 75.0],
    //     0.2, 0.8, 1.0, 200.0, 0.1)
    // ));
    // let mut frog = load_smf_mesh("models/frog.smf", true);
    // frog.scale(vec3![1.4, 1.4, 1.4]);
    // frog.rotate(vec3![10.0, 135.0, 10.0]);
    // frog.translate(vec3![-1.2, 2.7, -9.0]);
    // frog.set_material(Material::Hall(Hall::new(
    //     vec3![127.0, 0.0, 255.0], vec3![127.0, 0.0, 255.0],
    //     0.1, 0.9, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut bunny = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny.scale(vec3![4.2, 4.8, 4.8]);
    // bunny.translate( vec3![-0.5, -2.1, -15.0]);
    // bunny.set_material(Material::Hall(Hall::new(
    //     vec3![200.0, 200.0, 200.0], vec3![100.0, 100.0, 100.0],
    //     0.2, 0.8, 0.0, 200.0, 0.1)
    // ));
    //
    // let mut bunny2 = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny2.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny2.scale(vec3![1.9, 1.9, 1.9]);
    // bunny2.translate( vec3![-2.0, -2.5, -9.5]);
    // bunny2.set_material(Material::Hall(Hall::new(
    //     vec3![76.0, 0.0, 153.0], vec3![76.0, 0.0, 153.0],
    //     0.3, 0.7, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut bunny3 = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny3.rotate(vec3![90.0, 180.0, 0.0]);
    // bunny3.scale(vec3![1.9, 1.9, 1.9]);
    // bunny3.translate( vec3![-2.9, 2.9, -7.5]);
    // bunny3.set_material(Material::Hall(Hall::new(
    //     vec3![76.0, 0.0, 153.0], vec3![76.0, 0.0, 153.0],
    //     0.3, 0.7, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut bunny4 = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny4.rotate(vec3![45.0, 45.0, 0.0]);
    // bunny4.scale(vec3![1.9, 1.9, 1.9]);
    // bunny4.translate( vec3![2.8, 2.5, -10.5]);
    // bunny4.set_material(Material::Hall(Hall::new(
    //     vec3![76.0, 0.0, 153.0], vec3![76.0, 0.0, 153.0],
    //     0.3, 0.7, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut bunny5 = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny5.rotate(vec3![0.0, -90.0, 0.0]);
    // bunny5.scale(vec3![1.9, 1.9, 1.9]);
    // bunny5.translate( vec3![3.7, -2.2, -8.0]);
    // bunny5.set_material(Material::Hall(Hall::new(
    //     vec3![76.0, 0.0, 153.0], vec3![76.0, 0.0, 153.0],
    //     0.7, 0.3, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut bunny6 = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny6.rotate(vec3![0.0, -90.0, 0.0]);
    // bunny6.scale(vec3![2.9, 2.9, 2.9]);
    // bunny6.translate( vec3![-4.7, -2.6, -13.0]);
    // bunny6.set_material(Material::Hall(Hall::new(
    //     vec3![76.0, 0.0, 153.0], vec3![76.0, 0.0, 153.0],
    //     0.3, 0.7, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut bunny7 = load_smf_mesh("models/bound-bunny_5k.smf", true);
    // bunny7.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny7.scale(vec3![2.0, 2.0, 2.0]);
    // bunny7.translate( vec3![0.9, 2.0, -10.0]);
    // bunny7.set_material(Material::Hall(Hall::new(
    //     vec3![255.0, 255.0, 255.0], vec3![255.0, 255.0, 255.0],
    //     0.3, 0.7, 0.0, 100.0, 0.1)
    // ));
    //
    // let mut sphere1 = Sphere::new(
    //     2.1,
    //     Material::Hall(Hall::new(
    //         vec3![204.0, 102.0, 0.0], vec3![204.0, 102.0, 0.0],
    //         0.15, 0.85, 0.0, 350.0, 0.1)
    //     )
    // );
    // sphere1.translate(vec3![3.0, -1.3, -12.0]);
    //
    // let mut sphere2 = Sphere::new(
    //     2.1,
    //     Material::Hall(Hall::new(
    //         vec3![204.0, 255.0, 255.0], vec3![204.0, 255.0, 255.0],
    //         0.15, 0.85, 0.0, 350.0, 0.1)
    //     )
    // );
    // sphere2.translate(vec3![-3.0, 2.8, -12.0]);
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.4)); // White
    // light1.set_position(vec3![0.0, 4.0, -14.0]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 0.0], 0.95)); // Yellow
    // light2.set_position(vec3![-1.5, -0.8, 0.0]);
    // scene.push_light(light2);
    //
    // scene.push_ambient_light(AmbientLight {
    //     intensity: vec3![1.0, 1.0, 1.0]
    // });
    //
    // let mut back_cube = load_smf_mesh("models/box.smf", false);
    // back_cube.scale(vec3![50.0, 50.0, 1.0]);
    // back_cube.translate(vec3![0.0, 0.0, -20.0]);
    // back_cube.set_material(Material::Hall(Hall::new(
    //     vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
    //     0.4, 0.6, 0.0, 250.0, 0.1)
    // ));
    //
    // let mut front_cube = load_smf_mesh("models/box.smf", false);
    // front_cube.scale(vec3![50.0, 50.0, 1.0]);
    // front_cube.translate(vec3![0.0, 0.0, 0.5]);
    // front_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut left_cube = load_smf_mesh("models/box.smf", false);
    // left_cube.scale(vec3![1.0, 50.0, 70.0]);
    // left_cube.rotate(vec3![0.0, 20.0, 0.0]);
    // left_cube.translate(vec3![-5.0, 0.0, -6.0]);
    // left_cube.set_material(Material::Hall(Hall::new(
    //     vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
    //     0.4, 0.6, 0.0, 250.0, 0.1)
    // ));
    //
    // let mut right_cube = load_smf_mesh("models/box.smf", false);
    // right_cube.scale(vec3![1.0, 50.0, 70.0]);
    // right_cube.rotate(vec3![0.0, -20.0, 0.0]);
    // right_cube.translate(vec3![5.0, 0.0, -6.0]);
    // right_cube.set_material(Material::Hall(Hall::new(
    //     vec3![0.0, 200.0, 200.0], vec3![0.0, 200.0, 200.0],
    //     0.4, 0.6, 0.0, 250.0, 0.1)
    // ));
    //
    // let mut bottom_cube = load_smf_mesh("models/box.smf", false);
    // bottom_cube.scale(vec3![50.0, 0.4, 70.0]);
    // bottom_cube.rotate(vec3![-10.0, 0.0, 0.0]);
    // bottom_cube.translate(vec3![0.0, -3.0, 0.0]);
    // bottom_cube.set_material(Material::Hall(Hall::new(
    //     vec3![200.0, 200.0, 200.0], vec3![200.0, 200.0, 200.0],
    //     0.7, 0.3, 0.0, 250.0, 0.1)
    // ));
    //
    // let mut top_cube = load_smf_mesh("models/box.smf", false);
    // top_cube.scale(vec3![70.0, 0.4, 70.0]);
    // top_cube.translate(vec3![0.0, 10.0, 0.0]);
    // top_cube.set_material(Material::Phong(Phong::new(vec3![0.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // scene.add_objects(vec![
    //     Box::new(bottom_cube),
    //     Box::new(top_cube),
    //     Box::new(left_cube),
    //     Box::new(right_cube),
    //     Box::new(back_cube),
    //     Box::new(front_cube),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(bunny),
    //     Box::new(bunny2),
    //     Box::new(bunny3),
    //     Box::new(bunny4),
    //     Box::new(bunny5),
    //     Box::new(bunny6),
    //     Box::new(bunny7),
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

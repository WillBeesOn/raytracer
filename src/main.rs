use std::time::Instant;

use crate::objects::{SceneObject, WorldLight, Light, Sphere, PointLight, AmbientLight};
use crate::objects::{Camera, Scene};
use crate::data_structures::{Ray, Vec3, Vector};
use crate::materials::{Hall, Material, Phong};
use crate::traits::{Hittable, HittableList};
use crate::utils::{load_smf_mesh, save_png};

mod objects;
mod data_structures;
mod traits;
mod materials;
mod utils;

fn main() {
    let dimension = 1024;
    let resolution = (dimension, dimension);
    let mut scene = Scene::new(
        resolution,
        100.0,
        vec3![255.0, 229.0, 204.0],
        60.0,
        50,
        5,
        1.0,
        true
    );

    // Scene 1 ===================
    //
    // let mut teapot = load_smf_mesh("models/teapot.smf", true);
    // teapot.scale(vec3![0.35, 0.35, 0.35]);
    // teapot.rotate(vec3![0.0, 130.0, 0.0]);
    // teapot.translate(vec3![-1.2, -2.45, -8.0]);
    // teapot.set_material(Material::Hall(Hall::new(
    //     vec3![255.0, 128.0, 0.0], vec3![255.0, 128.0, 0.0], vec3![255.0, 128.0, 0.0],
    //     0.3, 0.7,
    //     0.0, 0.0, 1.0,
    //     200.0, 0.1)
    // ));
    // let mut frog = load_smf_mesh("models/frog.smf", true);
    // frog.scale(vec3![3.9, 3.9, 3.9]);
    // frog.rotate(vec3![10.0, 110.0, 10.0]);
    // frog.translate(vec3![0.0, 1.7, -14.0]);
    // frog.set_material(Material::Hall(Hall::new(
    //     vec3![35.0, 65.0, 235.0], vec3![35.0, 65.0, 235.0],vec3![35.0, 65.0, 235.0],
    //     0.4, 0.6,
    //     0.4, 0.0, 1.0,
    //     50.0, 0.1)
    // ));
    //
    // let mut bunny = load_smf_mesh("models/bound-bunny_1k.smf", true);
    // bunny.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny.translate( vec3![-1.0, -0.75, -3.5]);
    // bunny.set_material(Material::Hall(Hall::new(
    //     vec3![204.0, 153.0, 255.0], vec3![204.0, 153.0, 255.0],vec3![255.0, 255.0, 255.0],
    //     0.4, 0.6,
    //     0.0, 0.0, 1.055,
    //     10.0, 0.1)
    // ));
    //
    // let mut sphere1 = Sphere::new(
    //     0.95,
    //     Material::Hall(Hall::new(
    //         vec3![159.0, 252.0, 178.0],vec3![159.0, 252.0, 178.0],vec3![255.0, 255.0, 255.0],
    //         0.45, 0.55,
    //         0.4, 0.0, 1.52,
    //         200.0, 0.0)
    //     )
    // );
    // sphere1.translate(vec3![1.0, -1.8, -6.5]);
    //
    // let mut sphere2 = Sphere::new(
    //     0.75,
    //     Material::Hall(Hall::new(
    //         vec3![102.0, 0.0, 0.0], vec3![192.0, 192.0, 192.0],vec3![192.0, 192.0, 192.0],
    //         0.4, 0.6,
    //         0.4, 0.0, 1.0,
    //         250.0, 0.1)
    //     )
    // );
    // sphere2.translate(vec3![2.2, -2.4, -9.0]);
    //
    // let mut sphere3 = Sphere::new(
    //     0.75,
    //     Material::Hall(Hall::new(
    //         vec3![102.0, 0.0, 0.0], vec3![192.0, 192.0, 192.0],vec3![192.0, 192.0, 192.0],
    //         0.4, 0.6,
    //         0.4, 0.0, 1.0,
    //         250.0, 0.1)
    //     )
    // );
    // sphere3.translate(vec3![-0.2, -1.9, -9.0]);
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.45)); // White
    // light1.set_position(vec3![0.0, 2.0, 4.0]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.5)); // White
    // light2.set_position(vec3![0.0, 5.0, -10.0]);
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
    // front_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut left_cube = load_smf_mesh("models/box.smf", false);
    // left_cube.scale(vec3![1.0, 50.0, 70.0]);
    // left_cube.rotate(vec3![0.0, 20.0, 0.0]);
    // left_cube.translate(vec3![-5.0, 0.0, -6.0]);
    // left_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut right_cube = load_smf_mesh("models/box.smf", false);
    // right_cube.scale(vec3![1.0, 50.0, 70.0]);
    // right_cube.rotate(vec3![0.0, -20.0, 0.0]);
    // right_cube.translate(vec3![5.0, 0.0, -6.0]);
    // right_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut bottom_cube = load_smf_mesh("models/box.smf", false);
    // bottom_cube.scale(vec3![50.0, 0.4, 70.0]);
    // bottom_cube.rotate(vec3![-10.0, 0.0, 0.0]);
    // bottom_cube.translate(vec3![0.0, -3.0, 0.0]);
    // bottom_cube.set_material(Material::Phong(Phong::new(vec3![230.0, 230.0, 230.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut top_cube = load_smf_mesh("models/box.smf", false);
    // top_cube.scale(vec3![50.0, 0.4, 50.0]);
    // top_cube.translate(vec3![0.0, 10.0, 0.0]);
    // top_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // scene.add_objects(vec![
    //     Box::new(bottom_cube),
    //     //Box::new(top_cube),
    //     //Box::new(left_cube),
    //     //Box::new(right_cube),
    //     //Box::new(back_cube),
    //     //Box::new(front_cube),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(sphere3),
    //     Box::new(teapot),
    //     Box::new(frog),
    //     Box::new(bunny)
    // ]);


    // Scene 2 ===================
    // let mut teapot = load_smf_mesh("models/teapot.smf", true);
    // teapot.scale(vec3![0.85, 0.85, 0.85]);
    // teapot.rotate(vec3![0.0, 130.0, 0.0]);
    // teapot.translate(vec3![2.8, -0.95, -14.0]);
    // teapot.set_material(Material::Hall(Hall::new(
    //     vec3![255.0, 128.0, 0.0], vec3![255.0, 128.0, 0.0], vec3![255.0, 128.0, 0.0],
    //     0.3, 0.7,
    //     0.0, 0.0, 1.0,
    //     200.0, 0.1)
    // ));
    let mut cow = load_smf_mesh("models/cow.smf", true);
    cow.scale(vec3![4.5, 4.5, 4.5]);
    cow.rotate(vec3![-10.0, 0.0, 0.0]);
    cow.translate(vec3![0.1, -1.7, -6.3]);
    cow.set_material(Material::Hall(Hall::new(
        vec3![174.0, 226.0, 255.0], vec3![174.0, 226.0, 255.0],vec3![255.0, 255.0, 255.0],
        0.35, 0.65,
        0.01, 0.0, 1.25,
        50.0, 0.1)
    ));
    //
    // let mut bunny = load_smf_mesh("models/bound-bunny_1k.smf", true);
    // bunny.scale(vec3![4.7, 4.7, 4.7]);
    // bunny.rotate(vec3![0.0, 0.0, 0.0]);
    // bunny.translate( vec3![-4.45, -2.8, -18.0]);
    // bunny.set_material(Material::Hall(Hall::new(
    //     vec3![255.0, 255.0, 51.0], vec3![255.0, 255.0, 51.0],vec3![255.0, 255.0, 255.0],
    //     0.25, 0.75,
    //     0.9, 0.0, 1.0,
    //     10.0, 0.1)
    // ));
    //
    // let mut sphere1 = Sphere::new(
    //     0.95,
    //     Material::Hall(Hall::new(
    //         vec3![159.0, 252.0, 178.0],vec3![159.0, 252.0, 178.0],vec3![255.0, 255.0, 255.0],
    //         0.25, 0.75,
    //         0.1, 0.0, 1.0,
    //         200.0, 0.0)
    //     )
    // );
    // sphere1.translate(vec3![1.75, -1.9, -7.2]);
    //
    // let mut sphere2 = Sphere::new(
    //     2.55,
    //     Material::Hall(Hall::new(
    //         vec3![255.0, 153.0, 255.0], vec3![255.0, 153.0, 255.0],vec3![255.0, 255.0, 255.0],
    //         0.55, 0.45,
    //         0.05, 0.0, 1.03,
    //         250.0, 0.1)
    //     )
    // );
    // sphere2.translate(vec3![0.0, -0.5, -8.8]);
    //
    // let mut sphere3 = Sphere::new(
    //     0.75,
    //     Material::Hall(Hall::new(
    //         vec3![102.0, 0.0, 0.0], vec3![192.0, 192.0, 192.0],vec3![192.0, 192.0, 192.0],
    //         0.4, 0.6,
    //         0.4, 0.0, 1.0,
    //         250.0, 0.1)
    //     )
    // );
    // sphere3.translate(vec3![-0.2, -1.9, -9.0]);
    //
    // let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.45)); // White
    // light1.set_position(vec3![0.0, 2.0, 3.0]);
    // scene.push_light(light1);
    //
    // let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.5)); // White
    // light2.set_position(vec3![0.0, 5.0, -7.0]);
    // scene.push_light(light2);
    //
    // scene.push_ambient_light(AmbientLight {
    //     intensity: vec3![1.0, 1.0, 1.0]
    // });
    //
    // let mut back_cube = load_smf_mesh("models/box.smf", false);
    // back_cube.scale(vec3![50.0, 50.0, 1.0]);
    // back_cube.translate(vec3![0.0, 0.0, -20.0]);
    // back_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut front_cube = load_smf_mesh("models/box.smf", false);
    // front_cube.scale(vec3![50.0, 50.0, 1.0]);
    // front_cube.translate(vec3![0.0, 0.0, 0.5]);
    // front_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut left_cube = load_smf_mesh("models/box.smf", false);
    // left_cube.scale(vec3![1.0, 50.0, 70.0]);
    // left_cube.rotate(vec3![0.0, 20.0, 0.0]);
    // left_cube.translate(vec3![-5.0, 0.0, -6.0]);
    // left_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut right_cube = load_smf_mesh("models/box.smf", false);
    // right_cube.scale(vec3![1.0, 50.0, 70.0]);
    // right_cube.rotate(vec3![0.0, -20.0, 0.0]);
    // right_cube.translate(vec3![5.0, 0.0, -6.0]);
    // right_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut bottom_cube = load_smf_mesh("models/box.smf", false);
    // bottom_cube.scale(vec3![50.0, 0.4, 70.0]);
    // bottom_cube.rotate(vec3![-10.0, 0.0, 0.0]);
    // bottom_cube.translate(vec3![0.0, -3.0, 0.0]);
    // bottom_cube.set_material(Material::Phong(Phong::new(vec3![230.0, 230.0, 230.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
    //
    // let mut top_cube = load_smf_mesh("models/box.smf", false);
    // top_cube.scale(vec3![50.0, 0.4, 50.0]);
    // top_cube.translate(vec3![0.0, 10.0, 0.0]);
    // top_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));
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
    //     //Box::new(sphere3),
    //     Box::new(teapot),
    //     Box::new(cow),
    //     Box::new(bunny)
    // ]);

    // Scene 3=========================
    let mut teapot = load_smf_mesh("models/teapot.smf", true);
    teapot.scale(vec3![1.0, 1.0, 1.0]);
    teapot.rotate(vec3![0.0, -20.0, 0.0]);
    teapot.translate(vec3![0.0, -1.5, -7.0]);
    teapot.set_material(Material::Hall(Hall::new(
        vec3![255.0, 218.0, 141.0], vec3![255.0, 218.0, 141.0], vec3![255.0, 255.0, 255.0],
        0.05, 0.35,
        0.0, 0.0, 1.05,
        450.0, 0.1)
    ));
    let mut cow = load_smf_mesh("models/cow.smf", true);
    cow.scale(vec3![4.5, 4.5, 4.5]);
    cow.rotate(vec3![-10.0, 0.0, 0.0]);
    cow.translate(vec3![0.1, -1.7, -6.3]);
    cow.set_material(Material::Hall(Hall::new(
        vec3![174.0, 226.0, 255.0], vec3![174.0, 226.0, 255.0],vec3![255.0, 255.0, 255.0],
        0.35, 0.65,
        0.01, 0.0, 1.25,
        50.0, 0.1)
    ));

    let mut bunny = load_smf_mesh("models/bound-bunny_1k.smf", true);
    bunny.scale(vec3![1.7, 1.7, 1.7]);
    bunny.rotate(vec3![0.0, 0.0, 0.0]);
    bunny.translate( vec3![-0.8, 1.8, -6.5]);
    bunny.set_material(Material::Hall(Hall::new(
        vec3![0.0, 128.0, 255.0], vec3![0.0, 128.0, 255.0],vec3![255.0, 255.0, 255.0],
        0.25, 0.75,
        0.0, 0.0, 1.0,
        80.0, 0.1)
    ));

    let mut bunny2 = load_smf_mesh("models/bound-bunny_1k.smf", true);
    bunny2.scale(vec3![1.7, 1.7, 1.7]);
    bunny2.rotate(vec3![0.0, 180.0, 0.0]);
    bunny2.translate( vec3![0.8, 1.8, -6.5]);
    bunny2.set_material(Material::Hall(Hall::new(
        vec3![29.0, 132.0, 53.0], vec3![29.0, 132.0, 53.0],vec3![255.0, 255.0, 255.0],
        0.25, 0.75,
        0.0, 0.0, 1.0,
        80.0, 0.1)
    ));

    let mut sphere1 = Sphere::new(
        0.75,
        Material::Hall(Hall::new(
            vec3![159.0, 252.0, 178.0],vec3![159.0, 252.0, 178.0],vec3![255.0, 255.0, 255.0],
            0.25, 0.75,
            0.1, 0.0, 1.0,
            200.0, 0.0)
        )
    );
    sphere1.translate(vec3![-2.0, 0.0, -9.0]);

    let mut sphere2 = Sphere::new(
        0.75,
        Material::Hall(Hall::new(
            vec3![255.0, 153.0, 255.0], vec3![255.0, 153.0, 255.0],vec3![255.0, 255.0, 255.0],
            0.55, 0.45,
            0.05, 0.0, 1.03,
            250.0, 0.1)
        )
    );
    sphere2.translate(vec3![2.0, 1.0, -9.0]);

    let mut sphere3 = Sphere::new(
        0.75,
        Material::Hall(Hall::new(
            vec3![255.0, 102.0, 102.0], vec3![255.0, 102.0, 102.0],vec3![255.0, 255.0, 255.0],
            0.05, 0.1,
            0.0, 0.0, 1.05,
            250.0, 0.1)
        )
    );
    sphere3.translate(vec3![0.8, -0.8, -4.0]);

    let mut light1 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.65)); // White
    light1.set_position(vec3![-1.0, 3.0, 0.0]);
    scene.push_light(light1);

    let mut light2 = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.65)); // White
    light2.set_position(vec3![1.0, -1.0, 0.0]);
    scene.push_light(light2);

    scene.push_ambient_light(AmbientLight {
        intensity: vec3![1.0, 1.0, 1.0]
    });

    let mut back_cube = load_smf_mesh("models/box.smf", false);
    back_cube.scale(vec3![50.0, 50.0, 1.0]);
    back_cube.translate(vec3![0.0, 0.0, -20.0]);
    back_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    let mut front_cube = load_smf_mesh("models/box.smf", false);
    front_cube.scale(vec3![50.0, 50.0, 1.0]);
    front_cube.translate(vec3![0.0, 0.0, 0.5]);
    front_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    let mut left_cube = load_smf_mesh("models/box.smf", false);
    left_cube.scale(vec3![1.0, 50.0, 70.0]);
    left_cube.rotate(vec3![0.0, 20.0, 0.0]);
    left_cube.translate(vec3![-5.0, 0.0, -6.0]);
    left_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    let mut right_cube = load_smf_mesh("models/box.smf", false);
    right_cube.scale(vec3![1.0, 50.0, 70.0]);
    right_cube.rotate(vec3![0.0, -20.0, 0.0]);
    right_cube.translate(vec3![5.0, 0.0, -6.0]);
    right_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    let mut bottom_cube = load_smf_mesh("models/box.smf", false);
    bottom_cube.scale(vec3![50.0, 0.4, 70.0]);
    bottom_cube.rotate(vec3![-10.0, 0.0, 0.0]);
    bottom_cube.translate(vec3![0.0, -3.0, 0.0]);
    bottom_cube.set_material(Material::Phong(Phong::new(vec3![230.0, 230.0, 230.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    let mut top_cube = load_smf_mesh("models/box.smf", false);
    top_cube.scale(vec3![50.0, 0.4, 50.0]);
    top_cube.translate(vec3![0.0, 10.0, 0.0]);
    top_cube.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 200.0], 0.35, 0.4, 20.0 , 0.0, 0.15)));

    scene.add_objects(vec![
        Box::new(bottom_cube),
        Box::new(top_cube),
        Box::new(left_cube),
        Box::new(right_cube),
        Box::new(back_cube),
        Box::new(front_cube),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
        Box::new(teapot),
        Box::new(bunny),
        Box::new(bunny2)
    ]);

    let now = Instant::now();
    let (computed, _heatmap) = scene.render_supersample_frame_threaded(0.05);
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

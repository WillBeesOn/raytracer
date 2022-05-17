use std::time::Instant;

use crate::objects::{PointLight, Sphere, SceneObject, WorldLight, Light};
use crate::objects::{Camera, Scene};
use crate::data_structures::{Matrix4, Ray, Vec3, Vec4, Vector};
use crate::materials::{Material, Phong};
use crate::traits::{Hittable, HittableList};
use crate::utils::{load_smf_mesh, save_png, supersample};

mod objects;
mod data_structures;
mod traits;
mod materials;
mod utils;

fn main() {
    let dimension = 50;
    let resolution = (dimension, dimension);
    let mut scene = Scene::new(
        resolution,
        100.0,
        vec3![100.0, 100.0, 100.0],
        60.0,
        25
    );

    // ======================

    // let mut penguin = load_smf_mesh("penguin.smf", true);
    // penguin.translate(vec3![0.15, 0.35, -2.5]);
    // penguin.set_material(Material::Phong(Phong::new(vec3![252.0, 115.0, 115.0], 0.5, 0.1, 25.0 , 0.15)));
    //
    // let mut teapot = load_smf_mesh("teapot.smf", true);
    // teapot.translate(vec3![0.2, -9.5, -7.5]);
    // teapot.rotate(vec3![-30.0, 25.0, 0.0]);
    // teapot.scale(vec3![0.15, 0.1, 0.3]);
    // teapot.set_material(Material::Phong(Phong::new(vec3![41.0, 238.0, 47.0], 0.35, 0.9, 7.0 , 0.15)));
    //
    // let mut bunny = load_smf_mesh("bound-bunny_1k.smf", true);
    // bunny.translate( vec3![1.0, 0.8, -3.0]);
    // bunny.rotate(vec3![0.0, 0.0, -45.0]);
    // bunny.set_material(Material::Phong(Phong::new(vec3![195.0, 94.0, 26.0], 0.1, 0.85, 5.0 , 0.15)));
    //
    // let mut cow = load_smf_mesh("cow.smf", true);
    // cow.translate( vec3![-0.7, -0.45, -2.0]);
    // cow.rotate(vec3![0.0, 0.0, 45.0]);
    // cow.set_material(Material::Phong(Phong::new(vec3![155.0, 157.0, 236.0], 0.45, 0.1, 20.0 , 0.15)));
    //
    // let mut sphere1 = Sphere::new(
    //     0.5,
    //     Material::Phong(Phong::new(vec3![247.0, 247.0, 36.0], 0.2, 0.75, 22.0 , 0.15))
    // );
    // sphere1.translate(vec3![-1.1, 0.9, -3.0]);
    //
    // let mut sphere2 = Sphere::new(
    //     0.5,
    //     Material::Phong(Phong::new(vec3![200.0, 63.0, 242.0], 0.4  , 0.55, 25.0, 0.15))
    // );
    // sphere2.translate(vec3![-0.49, 0.38, -4.0]);
    //
    // let mut sphere3 = Sphere::new(
    //     0.5,
    //     Material::Phong(Phong::new(vec3![75.0, 75.0, 75.0], 0.25 , 1.0 , 25.0, 0.15))
    // );
    // sphere3.translate(vec3![0.13, -0.42, -6.0]);
    //
    // let mut sphere4 = Sphere::new(
    //     0.5,
    //     Material::Phong(Phong::new(vec3![169.0, 210.0, 154.0], 0.5  , 0.75, 30.0, 0.15))
    // );
    // sphere4.translate(vec3![1.25, -1.35, -8.0]);
    //
    // scene.add_objects(vec![
    //     Box::new(penguin),
    //     Box::new(teapot),
    //     Box::new(bunny),
    //     Box::new(cow),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(sphere3),
    //     Box::new(sphere4),
    // ]);
    //
    // let mut white_light = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.75));
    // white_light.set_position(vec3![3.5, 2.4, 3.0]);
    // scene.push_light(white_light);


    // ======================

    // let mut frog = load_smf_mesh("frog.smf", true);
    // frog.translate(vec3![1.5, -1.5, -6.5]);
    // frog.rotate(vec3![0.0, 35.0, 0.0]);
    // frog.set_material(Material::Phong(Phong::new(vec3![252.0, 40.0, 115.0], 0.5, 0.1, 25.0 , 0.45)));
    //
    // let mut frog2 = load_smf_mesh("frog.smf", true);
    // frog2.translate(vec3![-1.8, -1.5, -6.5]);
    // frog2.rotate(vec3![0.0, 135.0, 0.0]);
    // frog2.set_material(Material::Phong(Phong::new(vec3![41.0, 238.0, 47.0], 0.35, 0.9, 7.0 , 0.15)));
    //
    // let mut bunny = load_smf_mesh("bound-bunny_5k.smf", true);
    // bunny.translate( vec3![-0.12, -0.08 , -7.0]);
    // bunny.rotate(vec3![0.0, 70.0, 0.0]);
    // bunny.scale(vec3![2.7, 2.7, 1.0]);
    // bunny.set_material(Material::Phong(Phong::new(vec3![160.0, 160.0, 160.0], 0.1, 0.85, 5.0 , 0.15)));
    //
    // let mut fish = load_smf_mesh("fish.smf", true);
    // fish.translate( vec3![-3.7, 4.8, -8.0]);
    // fish.scale( vec3![0.75, 0.75, 1.0]);
    // fish.rotate(vec3![25.0, 55.0, -45.0]);
    // fish.set_material(Material::Phong(Phong::new(vec3![70.0, 231.0, 255.0], 0.35, 0.9, 20.0 , 0.15)));
    //
    // let mut sphere1 = Sphere::new(
    //     0.45,
    //     Material::Phong(Phong::new(vec3![247.0, 247.0, 36.0], 0.2, 0.75, 22.0 , 0.15))
    // );
    // sphere1.translate(vec3![-0.08, -2.2, -5.5]);
    //
    // let mut sphere2 = Sphere::new(
    //     1.5,
    //     Material::Phong(Phong::new(vec3![200.0, 63.0, 242.0], 0.4  , 0.55, 25.0, 0.15))
    // );
    // sphere2.translate(vec3![-2.9, -0.2, -8.5]);
    //
    // let mut sphere3 = Sphere::new(
    //     1.55,
    //     Material::Phong(Phong::new(vec3![255.0, 155.0, 0.0], 0.25 , 1.0 , 25.0, 0.15))
    // );
    // sphere3.translate(vec3![2.4, -0.4, -7.0]);
    //
    // let mut sphere4 = Sphere::new(
    //     1.4,
    //     Material::Phong(Phong::new(vec3![185.0, 21.0, 21.0], 0.5  , 0.75, 30.0, 0.15))
    // );
    // sphere4.translate(vec3![1.05, 2.45, -7.8]);
    //
    // scene.add_objects(vec![
    //     Box::new(frog),
    //     Box::new(frog2),
    //     Box::new(bunny),
    //     Box::new(fish),
    //     Box::new(sphere1),
    //     Box::new(sphere2),
    //     Box::new(sphere3),
    //     Box::new(sphere4),
    // ]);
    //
    // let mut white_light = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.15));
    // white_light.set_position(vec3![-0.08, -2.2, -3.5]);
    // scene.push_light(white_light);

    // ======================

    let mut teddy = load_smf_mesh("models/teddy.smf", true);
    teddy.translate(vec3![-4.0, -1.3, -3.0]);
    teddy.scale(vec3![0.05, 0.05, 0.05]);
    teddy.rotate(vec3![0.0, 35.0, 0.0]);
    teddy.set_material(Material::Phong(Phong::new(vec3![102.0, 0.0, 102.0], 0.5, 0.75, 25.0 , 0.45)));

    let mut frog = load_smf_mesh("models/frog.smf", true);
    frog.translate(vec3![-1.8, -0.55, -6.5]);
    frog.rotate(vec3![10.0, 135.0, 30.0]);
    frog.set_material(Material::Phong(Phong::new(vec3![35.0, 65.0, 245.0], 0.25, 1.0, 3.0 , 0.15)));

    let mut bunny = load_smf_mesh("models/bound-bunny_5k.smf", true);
    bunny.translate( vec3![-0.5, 0.6, -2.0]);
    bunny.rotate(vec3![0.0, -25.0, 20.0]);
    bunny.set_material(Material::Phong(Phong::new(vec3![41.0, 238.0, 47.0], 0.1, 0.85, 15.0 , 0.15)));

    let mut penguin = load_smf_mesh("models/penguin.smf", true);
    penguin.rotate(vec3![0.0, 0.0, 90.0]);
    penguin.translate(vec3![-0.86, -0.92, -2.5]);
    penguin.set_material(Material::Phong(Phong::new(vec3![200.0, 200.0, 66.0], 0.35, 0.4, 20.0 , 0.15)));

    let mut sphere1 = Sphere::new(
        0.5,
        Material::Phong(Phong::new(vec3![247.0, 247.0, 36.0], 0.2, 0.75, 22.0 , 0.15))
    );
    sphere1.translate(vec3![-0.4, -0.2, -4.5]);

    let mut sphere2 = Sphere::new(
        0.2,
        Material::Phong(Phong::new(vec3![200.0, 63.0, 242.0], 0.4  , 0.55, 25.0, 0.15))
    );
    sphere2.translate(vec3![0.1, 0.8, -2.0]);

    let mut sphere3 = Sphere::new(
        0.1,
        Material::Phong(Phong::new(vec3![255.0, 155.0, 0.0], 0.25 , 1.0 , 25.0, 0.15))
    );
    sphere3.translate(vec3![0.4, -0.4, -1.0]);

    let mut sphere4 = Sphere::new(
        2.0,
        Material::Phong(Phong::new(vec3![185.0, 21.0, 21.0], 0.5  , 0.75, 30.0, 0.15))
    );
    sphere4.translate(vec3![6.55, 5.45, -14.8]);

    scene.add_objects(vec![
        Box::new(teddy),
        Box::new(frog),
        Box::new(bunny),
        Box::new(penguin),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3),
        Box::new(sphere4),
    ]);

    let mut white_light = Light::PointLight(PointLight::new(vec3![255.0, 255.0, 255.0], 0.95));
    white_light.set_position(vec3![-8.0, 2.4, 4.0]);
    scene.push_light(white_light);

    let now = Instant::now();
    let (computed, heatmap) = scene.render_supersample_frame_threaded(0.05);
    //let computed = scene.render_frame_threaded();
    println!("Took {:.2?} to render.", now.elapsed());
    save_png(&computed, resolution, "out.png");
    save_png(&heatmap, resolution, "out_heatmap.png");

    // // Print out super sampled version of image
    // let samples = 4;
    // let square = (samples as f32).sqrt() as u32;
    // let sampled_resolution = (resolution.0 / square, resolution.1 / square);
    // let antialiased = supersample(&computed, resolution, samples);
    // save_png(&antialiased, sampled_resolution, "out_supersample.png");
}

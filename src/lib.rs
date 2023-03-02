pub mod camera;
pub mod consts;
pub mod geometry;
pub mod material;
pub mod math;
pub mod ray;
pub mod renderer;
pub mod scene;

use consts::*;
use indicatif::HumanDuration;
use math::vec3::*;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};

use crate::camera::Camera;
use crate::geometry::hit::HittableList;
use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::math::color::Color;
use crate::renderer::{render_no_bar, render_single_core};
use crate::scene::Scene;

const FILE_DEFAULT_PATH: &str = "out.png";

pub fn run_random_scene() {
    if cfg!(debug_assertions) {
        println!("You are rendering this scene in debug, which is very slow. Make sure you don't want to run it in release mode.")
    }

    let start = Instant::now();
    let pixels = render_single_core(&Scene::random(), IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("Raytracing finished in {}", HumanDuration(start.elapsed()));

    write_image(&pixels, Path::new(FILE_DEFAULT_PATH));
}

pub fn run_same_as_bench() {
    let mut world = HittableList::new();

    let material = Material::new_dielectric(1.5);
    world.add(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material));

    let camera = Camera::default();
    let scene = Scene::new(world, camera);

    let start = Instant::now();

    let pixels = render_no_bar(&scene, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    write_image(&pixels, Path::new(FILE_DEFAULT_PATH));
}

fn write_image(pixels: &Vec<u8>, path: &Path) {
    println!("Writing image...");

    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&pixels).unwrap();

    let path_str = path.to_str().unwrap();
    println!("Image written to {path_str}");
}

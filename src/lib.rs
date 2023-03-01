pub mod camera;
pub mod consts;
pub mod geometry;
pub mod material;
pub mod math;
pub mod ray;
pub mod renderer;

use consts::*;
use indicatif::HumanDuration;
use math::vec3::*;
use rand::Rng;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};

use crate::camera::Camera;
use crate::geometry::hit::HittableList;
use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::renderer::{render_single_core, render_single_core_no_bar};

pub fn run() {
    // Setup image encoder
    let path = Path::new("out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    // Setup object world
    let mut world = random_scene();

    let camera = Camera::default();

    let start = Instant::now();

    let pixels = render_single_core(&mut world, &camera, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Point3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::new_lambertian(albedo);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Material::new_metal(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    let sphere_material = Material::new_dielectric(1.5);
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material = Material::new_dielectric(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material));

    world
}

pub fn run_same_as_bench() {
    // Setup image encoder
    let path = Path::new("out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut world = HittableList::new();

    let material = Material::new_dielectric(1.5);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material));

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
    );

    let start = Instant::now();

    let pixels = render_single_core_no_bar(&world, &camera, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}

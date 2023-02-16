pub mod camera;
pub mod consts;
pub mod geometry;
pub mod material;
pub mod math;
pub mod ray;
pub mod renderer;

use consts::*;
use geometry::hit::Hittable;
use indicatif::HumanDuration;
use math::vec3::*;
use rand::Rng;
use ray::Ray;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};

use crate::camera::Camera;
use crate::geometry::hit::HittableList;
use crate::geometry::sphere::Sphere;
use crate::material::Lambertian;
use crate::material::Dielectric;
use crate::material::Metal;
use crate::renderer::render_single_core;

fn ray_color(ray: &Ray, world: &mut HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color::zero();
    }

    if let Some(record) = world.hit(ray, 0.001, f32::INFINITY) {
        return if let Some(result) = record.material().scatter(ray, &record) {
            result.attenuation * ray_color(&result.scattered, world, depth - 1)
        } else {
            Color::zero()
        };
    }

    let unit_direction = ray.direction().to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Setup image encoder
    let path = Path::new("out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    // Setup object world
    let mut world = random_scene();

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

    let pixels = render_single_core(&mut world, &camera, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        Box::new(material_ground),
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
                    let sphere_material = Lambertian::new(albedo);
                    world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Metal::new(albedo, fuzz);
                    world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                } else {
                    let sphere_material = Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, Box::new(sphere_material)));
                }
            }
        }
    }

    let material = Dielectric::new(1.5);
    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Box::new(material),
    ));
    let material = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Box::new(material),
    ));
    let material = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Box::new(material),
    ));

    world
}

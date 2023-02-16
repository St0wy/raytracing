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
use ray::Ray;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};

use crate::camera::Camera;
use crate::geometry::hit::HittableList;
use crate::geometry::sphere::Sphere;
use crate::material::{Lambertian, Metal};
use crate::renderer::render;

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
    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8));
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2));

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(material_center),
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    ));

    let camera = Camera::new();

    let start = Instant::now();

    let pixels = render(&mut world, &camera, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}

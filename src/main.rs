pub mod camera;
pub mod consts;
pub mod geometry;
pub mod math;
pub mod ray;
pub mod renderer;

use consts::*;
use geometry::hit::Hittable;
use indicatif::HumanDuration;
use math::vec3::*;
use ray::Ray;
use std::f32::INFINITY;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};

use crate::camera::Camera;
use crate::geometry::hit::HittableList;
use crate::geometry::sphere::Sphere;
use crate::renderer::render;

fn ray_color(ray: &Ray, world: &mut HittableList) -> Color {
    if let Some(record) = world.hit(ray, 0.0, INFINITY) {
        return 0.5 * (*record.normal() + Color::new(1.0, 1.0, 1.0));
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
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let camera = Camera::new();

    let start = Instant::now();

    let pixels = render(&mut world, &camera, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}
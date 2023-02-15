pub mod ray;
pub mod renderer;
pub mod vec3;

use indicatif::HumanDuration;
use ray::Ray;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};
use vec3::{Color, Point3, Vec3};

use crate::renderer::render;

fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).to_unit();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction = ray.direction().to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin() - *center;
    let a = ray.direction().squared_magnitude();
    let half_b = oc.dot(ray.direction());
    let c = oc.squared_magnitude() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}

fn main() {
    let aspect_ratio = 16.0f32 / 9.0;
    let image_width = 400usize;
    // let image_width = 1920usize;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let path = Path::new("out.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, image_width as u32, image_height as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let start = Instant::now();

    let pixels = render(image_width, image_height);

    println!("Finished in {}", HumanDuration(start.elapsed()));

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&pixels).unwrap();
    println!("Image written to {:?}", path);
}

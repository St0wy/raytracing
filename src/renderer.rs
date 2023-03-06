use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

use crate::consts::{MAX_DEPTH, SAMPLES_PER_PIXEL};
use crate::geometry::hit::HittableList;
use crate::math::color::Color;
use crate::ray::Ray;
use crate::scene::Scene;

/// Gets the color of the provided ray.
///
/// # Arguments
///
/// * `ray`: Ray to get the color of.
/// * `hittable_list`: List of hittable objects to check the ray on.
/// * `depth`: Number of iterations left.
///
/// returns: Vec3
fn ray_color(ray: &Ray, hittable_list: &HittableList, depth: u32) -> Color {
    if depth == 0 {
        return Color::black();
    }

    if let Some(record) = hittable_list.hit_no_limit(ray) {
        return if let Some(result) = record.material().scatter(ray, &record) {
            result.attenuation * ray_color(&result.scattered, hittable_list, depth - 1)
        } else {
            Color::black()
        };
    }

    let unit_direction = ray.direction().to_unit();
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
}

/// Writes the color to the pixels vector.
///
/// # Arguments
///
/// * `pixels`: Pixels vector to write the color into.
/// * `color`: Color to write in the pixels vector.
///
/// returns: ()
fn write_color(pixels: &mut Vec<u8>, color: Color) {
    let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
    for k in 0..3 {
        let scaled_color = (color[k as usize] * scale).sqrt();
        pixels.push((256.0 * scaled_color.clamp(0.0, 0.999)) as u8);
    }
}

pub fn render_single_core(scene: &Scene, image_width: usize, image_height: usize) -> Vec<u8> {
    let bar = &Box::new(ProgressBar::new(((image_width * image_height) / 64) as u64));
    bar.set_prefix("🎨  Rendering");
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%")
            .unwrap(),
    );

    let mut rng = rand::thread_rng();

    let mut pixels = Vec::new();
    pixels.reserve(image_width * image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::black();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                let ray = scene.camera().get_ray(u, v);

                pixel_color += ray_color(&ray, scene.hittable_list(), MAX_DEPTH);
            }

            // Increment the loading bar (not all the time to save perf)
            if i % 64 == 0 {
                bar.inc(1);
            }

            write_color(&mut pixels, pixel_color);
        }
    }

    bar.finish();

    return pixels;
}

pub fn render_no_bar(scene: &Scene, image_width: usize, image_height: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    let mut pixels = Vec::new();
    pixels.reserve(image_width * image_height);
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color::black();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                let ray = scene.camera().get_ray(u, v);

                pixel_color += ray_color(&ray, scene.hittable_list(), MAX_DEPTH);
            }

            write_color(&mut pixels, pixel_color);
        }
    }

    return pixels;
}

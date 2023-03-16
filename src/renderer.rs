use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
use tracy::zone;

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
///
/// returns: Vec3
fn ray_color(mut ray: Ray, background_color: &Color, hittable_list: &HittableList) -> Color {
    zone!();
    let mut color = Color::white();
    let mut emitted = Color::black();

    for _ in 0..MAX_DEPTH {
        let record = hittable_list.hit_no_limit(&ray);

        if record.is_none() {
            return *background_color;
        }
        let record = record.unwrap();
        let emit = record
            .material()
            .emit(record.u(), record.v(), record.point());
        emitted += color * emit;

        let scatter = record.material().scatter(&ray, &record);
        if scatter.is_none() {
            return emitted;
        }

        let scatter = scatter.unwrap();
        color = color * scatter.attenuation;
        ray = scatter.scattered;

        if color.dot(&color) < 0.0001 {
            return emitted;
        }
    }

    emitted
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
    let bar = &Box::new(ProgressBar::new(
        ((image_width * image_height) / 256) as u64,
    ));
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

                pixel_color += ray_color(ray, scene.background_color(), scene.hittable_list());
            }

            // Increment the loading bar (not all the time to save perf)
            if i % 256 == 0 {
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

                pixel_color += ray_color(ray, scene.background_color(), scene.hittable_list());
            }

            write_color(&mut pixels, pixel_color);
        }
    }

    return pixels;
}

pub fn render_no_bar_multithreaded(
    scene: &Scene,
    image_width: usize,
    image_height: usize,
) -> Vec<u8> {
    zone!();
    let pixels: Vec<u8> = (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..image_width)
                .into_par_iter()
                .flat_map(|i| {
                    zone!();
                    let mut pixel_color = Color::black();
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let mut rng = rand::thread_rng();
                        let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                        let v = (j as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                        let ray = scene.camera().get_ray(u, v);

                        pixel_color +=
                            ray_color(ray, scene.background_color(), scene.hittable_list());
                    }

                    const SCALE: f32 = 1.0 / SAMPLES_PER_PIXEL as f32;
                    (0..3)
                        .into_iter()
                        .map(|k| {
                            (256.0 * (pixel_color[k as usize] * SCALE).sqrt().clamp(0.0, 0.999))
                                as u8
                        })
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>();

    return pixels;
}

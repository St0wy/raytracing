use rand::Rng;
use rayon::prelude::*;

use crate::consts::{MAX_DEPTH, SAMPLES_PER_PIXEL};
use crate::geometry::hittable_world::HittableWorld;
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
fn ray_color(mut ray: Ray, background_color: &Color, hittable_list: &HittableWorld) -> Color {
    let mut color = Color::white();
    let mut emitted = Color::black();

    for _ in 0..MAX_DEPTH {
        let record = hittable_list.hit_no_limit(&ray);

        if record.is_none() {
            return *background_color * color;
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
        color *= scatter.attenuation;
        ray = scatter.scattered;

        if color.dot(&color) < 0.0001 {
            return emitted;
        }
    }

    emitted
}

pub fn render(scene: &Scene, image_width: usize, image_height: usize) -> Vec<u8> {
    (0..image_height)
        .into_par_iter()
        .rev()
        .flat_map(|j| {
            (0..image_width)
                .flat_map(|i| {
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
                        .map(|k| {
                            (256.0 * (pixel_color[k as usize] * SCALE).sqrt().clamp(0.0, 0.999))
                                as u8
                        })
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<u8>>()
}

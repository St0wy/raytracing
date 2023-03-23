use rand::Rng;

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
fn ray_color(
    ray: &Ray,
    background_color: &Color,
    hittable_world: &HittableWorld,
    depth: u32,
) -> Color {
    if depth == 0 {
        return Color::black();
    }

    let record = hittable_world.hit_no_limit(ray);

    if record.is_none() {
        return *background_color;
    }
    let record = record.unwrap();

    let emitted = record
        .material()
        .emit(record.u(), record.v(), record.point());

    let scatter = record.material().scatter(ray, &record);
    if scatter.is_none() {
        return emitted;
    }
    let scatter_result = scatter.unwrap();

    let ray_color = ray_color(
        &scatter_result.scattered,
        background_color,
        hittable_world,
        depth - 1,
    );

    emitted + scatter_result.attenuation * ray_color
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

pub fn render(scene: &Scene, image_width: usize, image_height: usize) -> Vec<u8> {
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

                pixel_color += ray_color(
                    &ray,
                    scene.background_color(),
                    scene.hittable_list(),
                    MAX_DEPTH,
                );
            }

            write_color(&mut pixels, pixel_color);
        }
    }

    pixels
}

use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

use crate::consts::MAX_DEPTH;
use crate::{
    camera::Camera, consts::SAMPLES_PER_PIXEL, geometry::hit::HittableList, math::vec3::*,
    ray_color,
};

fn write_color(pixels: &mut Vec<u8>, color: Vec3) {
    let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
    for k in 0..3 {
        let scaled_color = (color[k as usize] * scale).sqrt();
        pixels.push((256.0 * scaled_color.clamp(0.0, 0.999)) as u8);
    }
}

pub fn render_single_core(
    world: &mut HittableList,
    camera: &Camera,
    image_width: usize,
    image_height: usize,
) -> Vec<u8> {
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
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, world, MAX_DEPTH);
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

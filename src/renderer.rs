use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;

use crate::{
    camera::Camera, consts::SAMPLES_PER_PIXEL, geometry::hit::HittableList, math::vec3::*,
    ray_color,
};

fn write_color(pixels: &mut Vec<u8>, color: Vec3) {
    let scale = 1.0 / SAMPLES_PER_PIXEL as f32;
    for k in 0..3 {
        let scaled_color = color[k as usize] * scale;
        pixels.push((256.0 * scaled_color.clamp(0.0, 0.999)) as u8);
    }
}

pub fn render(
    world: &mut HittableList,
    camera: &Camera,
    image_width: usize,
    image_height: usize,
) -> Vec<u8> {
    let bar = &Box::new(ProgressBar::new((image_width * image_height / 64) as u64));
    bar.set_prefix("🎨  Rendering");
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%")
            .unwrap(),
    );

    let mut rng = rand::thread_rng();

    let mut pixels = Vec::new();
    for j in (0..image_height).rev() {
        for i in (0..image_width).rev() {
            let mut pixel_color = Color::zero();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + rng.gen::<f32>()) / (image_width as f32 - 1.0);
                let v = (j as f32 + rng.gen::<f32>()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);

                // Increment the loading bar (not all the time to save perf)
                if i % 64 == 0 {
                    bar.inc(1);
                }

                pixel_color += ray_color(&ray, world);
            }
            write_color(&mut pixels, pixel_color);
        }
    }

    return pixels;
}

// let pixels: Vec<u8> = (0..image_height)
//     .into_iter()
//     .rev()
//     .flat_map(|j| {
//         (0..image_width).into_iter().flat_map(move |i| {
//             let u = i as f32 / (image_width as f32 - 1.0);
//             let v = j as f32 / (image_height as f32 - 1.0);
//             let ray = Ray::new(
//                 origin,
//                 lower_left_corner + u * horizontal + v * vertical - origin,
//             );

//             let color = ray_color(&ray);
//             (0..3)
//                 .into_iter()
//                 .map(move |k| (255.99 * color[k as usize]).min(255.0) as u8)
//         })
//     })
//     .collect();

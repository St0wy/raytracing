use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    ray::Ray,
    ray_color,
    vec3::{Point3, Vec3},
};

pub fn render(image_width: usize, image_height: usize) -> Vec<u8> {
    let aspect_ratio = image_width as f32 / image_height as f32;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let bar = &Box::new(ProgressBar::new((image_width * image_height / 64) as u64));
    bar.set_prefix("🎨  Rendering");
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%")
            .unwrap(),
    );

    let mut pixels = Vec::new();
    for j in (0..image_height).rev() {
        for i in (0..image_width).rev() {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            if i % 64 == 0 {
                bar.inc(1);
            }

            let color = ray_color(&ray);
            for k in 0..3 {
                pixels.push((255.99 * color[k as usize]).min(255.0) as u8);
            }
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

pub mod vec3;
use vec3::{Color, Vec3};

fn set_color(pixel: &mut image::Rgb<u8>, vec: &Color) {
    *pixel = image::Rgb([
        (vec.x * 255.999) as u8,
        (vec.y * 255.999) as u8,
        (vec.z * 255.999) as u8,
    ])
}

fn main() {
    let img_x = 256;
    let img_y = 256;

    let mut imgbuf = image::ImageBuffer::new(img_x, img_y);

    for x in (0..img_x).rev() {
        println!("Scanlines remaining : {x}");
        for y in (0..img_y).rev() {
            let pixel_color = Color::new(
                x as f32 / (img_x - 1) as f32,
                1.0 - y as f32 / (img_y - 1) as f32,
                0.25,
            );

            let pixel = imgbuf.get_pixel_mut(x, y);
            set_color(pixel, &pixel_color);
        }
    }

    imgbuf.save("myimg.png").unwrap();
}

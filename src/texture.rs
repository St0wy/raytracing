use crate::math::color::Color;
use crate::math::perlin::Perlin;
use crate::math::vec3::Vec3;
use std::fs::File;
use tracy::zone;

const BYTES_PER_PIXEL: usize = 3;

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(Color),
    Checker {
        odd: Box<Texture>,
        even: Box<Texture>,
    },
    Noise {
        noise: Perlin,
        scale: f32,
    },
    Image {
        data: Vec<u8>,
        width: usize,
        height: usize,
        bytes_per_scanline: usize,
    },
}

impl Texture {
    pub fn new_solid_color(color: Color) -> Self {
        Texture::SolidColor(color)
    }

    pub fn new_checker(odd: Texture, even: Texture) -> Self {
        Texture::Checker {
            odd: Box::new(odd),
            even: Box::new(even),
        }
    }

    pub fn new_noise(noise: Perlin, scale: f32) -> Self {
        Texture::Noise { noise, scale }
    }

    pub fn new_image(filename: String) -> Option<Self> {
        zone!();
        let file = File::open(filename);
        if let Err(err) = file {
            eprintln!("Could not open texture image : {err}");
            return None;
        }
        let file = file.unwrap();

        let decoder = png::Decoder::new(file);
        let reader = decoder.read_info();
        if let Err(err) = reader {
            eprintln!("Could not decode the file : {err}");
            return None;
        }
        let mut reader = reader.unwrap();

        let mut buffer = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buffer);
        if let Err(err) = info {
            eprintln!("Could not read the next frame in the image : {err}");
            return None;
        }
        let info = info.unwrap();
        let bytes = buffer[..info.buffer_size()].to_vec();

        Some(Texture::Image {
            data: bytes,
            width: info.width as usize,
            height: info.height as usize,
            bytes_per_scanline: BYTES_PER_PIXEL * info.width as usize,
        })
    }

    pub fn value(&self, u: f32, v: f32, p: &Vec3) -> Color {
        zone!();
        match self {
            Texture::SolidColor(color) => *color,
            Texture::Checker { odd, even } => {
                let sines = f32::sin(10.0 * p.x) * f32::sin(10.0 * p.y) * f32::sin(10.0 * p.z);
                if sines < 0.0 {
                    odd.value(u, v, p)
                } else {
                    even.value(u, v, p)
                }
            }
            Texture::Noise { noise, scale } => {
                Color::white()
                    * 0.5
                    * (1.0 + f32::sin(scale * p.z + 10.0 * noise.turbulence(*p, None)))
            }
            Texture::Image {
                data,
                width,
                height,
                bytes_per_scanline,
            } => get_texture_image_value(u, v, &data, *width, *height, *bytes_per_scanline),
        }
    }
}

fn get_texture_image_value(
    u: f32,
    v: f32,
    data: &Vec<u8>,
    width: usize,
    height: usize,
    bytes_per_scanline: usize,
) -> Color {
    zone!();
    if data.is_empty() {
        return Color::new(0.0, 1.0, 1.0);
    }

    let u = f32::clamp(u, 0.0, 1.0);
    let v = 1.0 - f32::clamp(v, 0.0, 1.0);

    let mut i = (u * width as f32) as usize;
    let mut j = (v * height as f32) as usize;

    if i >= width {
        i = width - 1;
    }

    if j >= height {
        j = height - 1;
    }

    let pixel_index = j * bytes_per_scanline + i * BYTES_PER_PIXEL;
    if pixel_index + 2 >= data.len() {
        return Color::new(0.0, 1.0, 1.0);
    }

    const COLOR_SCALE: f32 = 1.0 / 255.0;
    let r = data[pixel_index] as f32 * COLOR_SCALE;
    let g = data[pixel_index + 1] as f32 * COLOR_SCALE;
    let b = data[pixel_index + 2] as f32 * COLOR_SCALE;

    Color::new(r, g, b)
}

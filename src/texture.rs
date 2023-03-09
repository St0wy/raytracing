use crate::math::color::Color;
use crate::math::perlin::Perlin;
use crate::math::vec3::Vec3;

#[derive(Debug, Clone)]
pub enum Texture {
    SolidColor(Color),
    Checker {
        odd: Box<Texture>,
        even: Box<Texture>,
    },
    Noise(Perlin),
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

    pub fn new_noise(noise: Perlin) -> Self {
        Texture::Noise(noise)
    }

    pub fn value(&self, u: f32, v: f32, p: &Vec3) -> Color {
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
            Texture::Noise(noise) => Color::white() * noise.noise(p),
        }
    }
}

use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::Rng;
use std::ops::{Add, AddAssign, Index, Mul, Range};

#[derive(Default, Debug, Copy, Clone)]
pub struct Color {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Color {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn from_rgb(x: u8, y: u8, z: u8) -> Self {
        let x = x as f32 / 255.0;
        let y = y as f32 / 255.0;
        let z = z as f32 / 255.0;

        Self { x, y, z }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(range);
        Self::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        )
    }

    pub const fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Index<usize> for Color {
    type Output = f32;

    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of color bounds !"),
        }
    }
}

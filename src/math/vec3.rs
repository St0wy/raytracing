use glam::Vec3A;
use rand::distributions::{Distribution, Uniform};
use rand_xoshiro::rand_core::RngCore;
use std::ops::Range;

pub trait Vec3Ext {
    fn random_range(range: Range<f32>, rng: &mut impl RngCore) -> Self;
    fn random_in_unit_circle(rng: &mut impl RngCore) -> Self;
    fn random_in_unit_sphere(rng: &mut impl RngCore) -> Self;
    fn random_unit_normalized(rng: &mut impl RngCore) -> Self;
    fn is_near_zero(&self) -> bool;
    fn reflect(&self, normal: Self) -> Self;
    fn refract(&self, normal: Self, refraction_ratio: f32) -> Self;
}

impl Vec3Ext for Vec3A {
    fn random_range(range: Range<f32>, rng: &mut impl RngCore) -> Self {
        let between = Uniform::from(range);
        Self::new(
            between.sample(rng),
            between.sample(rng),
            between.sample(rng),
        )
    }

    fn random_in_unit_circle(rng: &mut impl RngCore) -> Self {
        let between = Uniform::from(-1.0..1.0);
        loop {
            let point = Vec3A::new(between.sample(rng), between.sample(rng), 0.0);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    fn random_in_unit_sphere(rng: &mut impl RngCore) -> Self {
        loop {
            let point = Self::random_range(-1.0..1.0, rng);
            if point.length_squared() < 1.0 {
                return point;
            }
        }
    }

    fn random_unit_normalized(rng: &mut impl RngCore) -> Self {
        Self::random_in_unit_sphere(rng).normalize()
    }

    fn is_near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    fn reflect(&self, normal: Self) -> Self {
        *self - (2.0 * self.dot(normal) * normal)
    }

    fn refract(&self, normal: Self, refraction_ratio: f32) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = refraction_ratio * (*self + cos_theta * normal);
        let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * normal;

        r_out_perp + r_out_parallel
    }
}

use crate::geometry::hit::HitRecord;
use crate::math::vec3::{Color, Vec3};
use crate::ray::Ray;

#[derive(Default)]
pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl ScatterResult {
    pub fn new(attenuation: Color, scattered: Ray) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = *record.normal() + Vec3::random_unit_normalized();
        if scatter_direction.is_near_zero() {
            scatter_direction = *record.normal();
        }

        Some(ScatterResult::new(
            self.albedo,
            Ray::new(*record.point(), scatter_direction),
        ))
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction().to_unit().reflect(record.normal());
        let scattered = Ray::new(*record.point(), reflected);
        if scattered.direction().dot(record.normal()) > 0.0 {
            Some(ScatterResult::new(
                self.albedo,
                Ray::new(*record.point(), reflected),
            ))
        } else {
            None
        }
    }
}

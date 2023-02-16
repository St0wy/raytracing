use rand::Rng;
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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction().to_unit().reflect(record.normal());
        let scattered = Ray::new(
            *record.point(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
        );
        if scattered.direction().dot(record.normal()) > 0.0 {
            Some(ScatterResult::new(self.albedo, scattered))
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn new(refraction_index: f32) -> Self {
        Self { refraction_index }
    }
}

fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray_in.direction().to_unit();
        let cos_theta = (-unit_direction).dot(record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = reflectance(cos_theta, refraction_ratio);
        let mut rng = rand::thread_rng();
        let direction = if cannot_refract || reflectance > rng.gen() {
            unit_direction.reflect(record.normal())
        } else {
            unit_direction.refract(record.normal(), refraction_ratio)
        };

        let scattered = Ray::new(*record.point(), direction);

        Some(ScatterResult::new(attenuation, scattered))
    }
}

use crate::geometry::hit::HitRecord;
use crate::math::color::Color;
use crate::math::vec3::Vec3Ext;
use crate::ray::Ray;
use crate::texture::Texture;
use glam::Vec3A;
use rand::Rng;
use tracy_full::zone;

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

#[derive(Debug, Clone)]
pub enum Material {
    Lambertian { albedo: Texture },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { refraction_index: f32 },
    DiffuseLight { emit: Texture },
}

impl Material {
    pub fn new_lambertian_color(albedo: Color) -> Self {
        Self::Lambertian {
            albedo: Texture::SolidColor(albedo),
        }
    }

    pub fn new_lambertian(albedo: Texture) -> Self {
        Self::Lambertian { albedo }
    }

    pub fn new_metal(albedo: Color, fuzz: f32) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self::Metal { albedo, fuzz }
    }

    pub fn new_dielectric(refraction_index: f32) -> Self {
        Self::Dielectric { refraction_index }
    }

    pub fn new_diffuse_light(emit: Texture) -> Self {
        Self::DiffuseLight { emit }
    }

    pub fn new_diffuse_light_color(color: Color) -> Self {
        Self::DiffuseLight {
            emit: Texture::new_solid_color(color),
        }
    }

    pub fn scatter(&self, ray_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        zone!();
        match self {
            Material::Lambertian { albedo } => scatter_lambertian(albedo, ray_in, record),
            Material::Metal { albedo, fuzz } => scatter_metal(albedo, *fuzz, ray_in, record),
            Material::Dielectric { refraction_index } => {
                scatter_dielectrics(*refraction_index, ray_in, record)
            }
            Material::DiffuseLight { emit: _ } => None,
        }
    }

    pub fn emit(&self, u: f32, v: f32, point: Vec3A) -> Color {
        zone!();
        match self {
            Self::DiffuseLight { emit } => emit.value(u, v, point),
            _ => Color::black(),
        }
    }
}

fn scatter_lambertian(albedo: &Texture, ray_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
    let mut scatter_direction = record.normal() + Vec3A::random_unit_normalized();
    if scatter_direction.is_near_zero() {
        scatter_direction = record.normal();
    }

    let mut scattered = Ray::new(record.point(), scatter_direction);
    scattered.time = ray_in.time;
    let attenuation = albedo.value(record.u(), record.v(), record.point());

    Some(ScatterResult::new(attenuation, scattered))
}

fn scatter_metal(
    albedo: &Color,
    fuzz: f32,
    ray_in: &Ray,
    record: &HitRecord,
) -> Option<ScatterResult> {
    let reflected = ray_in.direction().normalize().reflect(record.normal());

    let mut scattered = Ray::new(
        record.point(),
        reflected + fuzz * Vec3A::random_in_unit_sphere(),
    );
    scattered.time = ray_in.time;

    if scattered.direction().dot(record.normal()) > 0.0 {
        Some(ScatterResult::new(*albedo, scattered))
    } else {
        None
    }
}

fn scatter_dielectrics(
    refraction_index: f32,
    ray_in: &Ray,
    record: &HitRecord,
) -> Option<ScatterResult> {
    let attenuation = Color::white();
    let refraction_ratio = if record.front_face() {
        1.0 / refraction_index
    } else {
        refraction_index
    };

    let unit_direction = ray_in.direction().normalize();
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

    let mut scattered = Ray::new(record.point(), direction);
    scattered.time = ray_in.time;

    Some(ScatterResult::new(attenuation, scattered))
}

fn reflectance(cosine: f32, refraction_ratio: f32) -> f32 {
    let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

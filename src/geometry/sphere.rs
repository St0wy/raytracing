use crate::geometry::aabb::Aabb;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;
use std::f32::consts::PI;
use tracy_full::zone;

use super::hit::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3A,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3A, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_sphere_uv(p: &Vec3A) -> (f32, f32) {
        let theta = f32::acos(-p.y);
        let phi = f32::atan2(-p.z, p.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        zone!();
        let oc = ray.origin() - self.center;
        let direction = ray.direction();
        let a = direction.length_squared();
        let half_b = oc.dot(direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        let record = HitRecord::new(
            point,
            root,
            u,
            v,
            outward_normal,
            &ray.direction(),
            &self.material,
        );

        Some(record)
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<Aabb> {
        let radius_vec = Vec3A::new(self.radius, self.radius, self.radius);

        Some(Aabb::new(
            self.center - radius_vec,
            self.center + radius_vec,
        ))
    }
}

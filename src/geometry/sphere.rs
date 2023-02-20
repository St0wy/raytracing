use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

use super::hit::{HitRecord, Hittable};

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let direction = ray.direction();
        let a = direction.squared_magnitude();
        let half_b = oc.dot(&direction);
        let c = oc.squared_magnitude() - self.radius * self.radius;

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
        let mut record = HitRecord::new(point, root, &self.material);
        record.set_face_normal(ray, &outward_normal);

        Some(record)
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

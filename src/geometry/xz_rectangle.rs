use crate::geometry::aabb::Aabb;
use crate::geometry::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::ray::Ray;

pub struct XzRectangle {
    material: Material,
    x0: f32,
    x1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl XzRectangle {
    pub fn new(material: Material, x0: f32, x1: f32, z0: f32, z1: f32, k: f32) -> Self {
        Self {
            material,
            x0,
            x1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for XzRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().y) / ray.direction().y;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x + t * ray.direction().x;
        let z = ray.origin().z + t * ray.direction().z;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0.0, 1.0, 0.0);

        Some(HitRecord::new(
            ray.at(t),
            t,
            u,
            v,
            outward_normal,
            &ray.direction(),
            &self.material,
        ))
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<Aabb> {
        Some(Aabb::new(
            Vec3::new(self.x0, self.k - 0.0001, self.z0),
            Vec3::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

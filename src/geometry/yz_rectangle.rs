use crate::geometry::aabb::Aabb;
use crate::geometry::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::ray::Ray;

pub struct YzRectangle {
    material: Material,
    y0: f32,
    y1: f32,
    z0: f32,
    z1: f32,
    k: f32,
}

impl YzRectangle {
    pub fn new(material: Material, y0: f32, y1: f32, z0: f32, z1: f32, k: f32) -> Self {
        Self {
            material,
            y0,
            y1,
            z0,
            z1,
            k,
        }
    }
}

impl Hittable for YzRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let t = (self.k - ray.origin().x) / ray.direction().x;
        if t < t_min || t > t_max {
            return None;
        }

        let y = ray.origin().y + t * ray.direction().y;
        let z = ray.origin().z + t * ray.direction().z;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1.0, 0.0, 0.0);

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
            Vec3::new(self.k - 0.0001, self.y0, self.z0),
            Vec3::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}

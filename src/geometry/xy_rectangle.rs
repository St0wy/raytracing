use crate::geometry::aabb::Aabb;
use crate::geometry::hit::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::vec3::Vec3;
use crate::ray::Ray;
use tracy_full::zone;

pub struct XyRectangle {
    material: Material,
    x0: f32,
    x1: f32,
    y0: f32,
    y1: f32,
    k: f32,
}

impl XyRectangle {
    pub fn new(material: Material, x0: f32, x1: f32, y0: f32, y1: f32, k: f32) -> Self {
        Self {
            material,
            x0,
            x1,
            y0,
            y1,
            k,
        }
    }
}

impl Hittable for XyRectangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        zone!();
        let t = (self.k - ray.origin().z) / ray.direction().z;
        if t < t_min || t > t_max {
            return None;
        }

        let x = ray.origin().x + t * ray.direction().x;
        let y = ray.origin().y + t * ray.direction().y;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }

        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0.0, 0.0, 1.0);

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
            Vec3::new(self.x0, self.y0, self.k - 0.0001),
            Vec3::new(self.x1, self.y1, self.k + 0.0001),
        ))
    }
}

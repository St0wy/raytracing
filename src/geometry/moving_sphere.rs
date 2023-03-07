use crate::geometry::aabb::Aabb;
use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

use super::hit::{HitRecord, Hittable};

pub struct MovingSphere {
    center0: Vec3,
    center1: Vec3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Material,
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time);
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
        let outward_normal = (point - self.center(ray.time)) / self.radius;
        let record = HitRecord::new(
            point,
            root,
            outward_normal,
            &ray.direction(),
            &self.material,
        );

        Some(record)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        let radius_vec = Vec3::new(self.radius, self.radius, self.radius);
        let box0 = Aabb::new(
            self.center(time0) - radius_vec,
            self.center(time0) + radius_vec,
        );

        let box1 = Aabb::new(
            self.center(time1) - radius_vec,
            self.center(time1) + radius_vec,
        );

        Some(Aabb::surrounding_box(box0, box1))
    }
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: Material,
    ) -> Self {
        Self {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f32) -> Vec3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

use crate::geometry::aabb::Aabb;
use crate::geometry::hit::{HitRecord, Hittable};
use crate::geometry::xy_rectangle::XyRectangle;
use crate::geometry::xz_rectangle::XzRectangle;
use crate::geometry::yz_rectangle::YzRectangle;
use crate::material::Material;
use crate::ray::Ray;
use glam::Vec3A;

pub struct AabbBox {
    box_min: Vec3A,
    box_max: Vec3A,
    sides_xy: [XyRectangle; 2],
    sides_xz: [XzRectangle; 2],
    sides_yz: [YzRectangle; 2],
}

impl AabbBox {
    pub fn new(box_min: Vec3A, box_max: Vec3A, material: Material) -> Self {
        Self {
            box_min,
            box_max,
            sides_xy: [
                XyRectangle::new(
                    material.clone(),
                    box_min.x,
                    box_max.x,
                    box_min.y,
                    box_max.y,
                    box_max.z,
                ),
                XyRectangle::new(
                    material.clone(),
                    box_min.x,
                    box_max.x,
                    box_min.y,
                    box_max.y,
                    box_min.z,
                ),
            ],
            sides_xz: [
                XzRectangle::new(
                    material.clone(),
                    box_min.x,
                    box_max.x,
                    box_min.z,
                    box_max.z,
                    box_max.y,
                ),
                XzRectangle::new(
                    material.clone(),
                    box_min.x,
                    box_max.x,
                    box_min.z,
                    box_max.z,
                    box_min.y,
                ),
            ],
            sides_yz: [
                YzRectangle::new(
                    material.clone(),
                    box_min.y,
                    box_max.y,
                    box_min.z,
                    box_max.z,
                    box_max.x,
                ),
                YzRectangle::new(
                    material, box_min.y, box_max.y, box_min.z, box_max.z, box_min.x,
                ),
            ],
        }
    }
}

impl Hittable for AabbBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record_option: Option<HitRecord> = None;
        let mut closest_distance = t_max;

        for side in self.sides_xy.iter() {
            let record = side.hit(ray, t_min, closest_distance);
            if let Some(record) = record {
                closest_distance = record.t();
                record_option = Some(record);
            }
        }

        for side in self.sides_xz.iter() {
            let record = side.hit(ray, t_min, closest_distance);
            if let Some(record) = record {
                closest_distance = record.t();
                record_option = Some(record);
            }
        }

        for side in self.sides_yz.iter() {
            let record = side.hit(ray, t_min, closest_distance);
            if let Some(record) = record {
                closest_distance = record.t();
                record_option = Some(record);
            }
        }

        record_option
    }

    fn bounding_box(&self, _: f32, _: f32) -> Option<Aabb> {
        Some(Aabb::new(self.box_min, self.box_max))
    }
}

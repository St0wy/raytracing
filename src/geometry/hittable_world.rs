use crate::geometry::aabb_box::AabbBox;
use crate::geometry::hit::{HitRecord, Hittable, HittableObjectType};
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::sphere::Sphere;
use crate::geometry::xy_rectangle::XyRectangle;
use crate::geometry::xz_rectangle::XzRectangle;
use crate::geometry::yz_rectangle::YzRectangle;
use crate::ray::Ray;

#[derive(Copy, Clone, Debug)]
pub struct HittableObjectIndex {
    pub object_type: HittableObjectType,
    pub index: usize,
}

impl HittableObjectIndex {
    pub fn new(object_type: HittableObjectType, index: usize) -> Self {
        HittableObjectIndex { object_type, index }
    }
}

#[derive(Default)]
pub struct HittableWorld {
    spheres: Vec<Sphere>,
    moving_spheres: Vec<MovingSphere>,
    xy_rectangles: Vec<XyRectangle>,
    xz_rectangles: Vec<XzRectangle>,
    yz_rectangles: Vec<YzRectangle>,
    aabb_boxes: Vec<AabbBox>,
}

impl HittableWorld {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            moving_spheres: Vec::new(),
            xy_rectangles: Vec::new(),
            xz_rectangles: Vec::new(),
            yz_rectangles: Vec::new(),
            aabb_boxes: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_moving_sphere(&mut self, moving_sphere: MovingSphere) {
        self.moving_spheres.push(moving_sphere);
    }

    pub fn add_xy_rectangle(&mut self, rectangle: XyRectangle) {
        self.xy_rectangles.push(rectangle);
    }

    pub fn add_xz_rectangle(&mut self, rectangle: XzRectangle) {
        self.xz_rectangles.push(rectangle);
    }

    pub fn add_yz_rectangle(&mut self, rectangle: YzRectangle) {
        self.yz_rectangles.push(rectangle);
    }

    pub fn add_aabb_box(&mut self, aabb_box: AabbBox) {
        self.aabb_boxes.push(aabb_box);
    }

    pub fn len(&self) -> usize {
        self.spheres.len()
            + self.moving_spheres.len()
            + self.xy_rectangles.len()
            + self.xz_rectangles.len()
            + self.yz_rectangles.len()
            + self.aabb_boxes.len()
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
        self.moving_spheres.clear();
        self.xy_rectangles.clear();
        self.xz_rectangles.clear();
        self.yz_rectangles.clear();
        self.aabb_boxes.clear();
    }

    pub fn hit_no_limit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit(ray, 0.001, f32::INFINITY)
    }
}

impl Hittable for HittableWorld {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.spheres.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t();
                temp_record = Some(record);
            }
        }

        for object in self.moving_spheres.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t();
                temp_record = Some(record);
            }
        }

        for object in self.xy_rectangles.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t();
                temp_record = Some(record);
            }
        }

        for object in self.xz_rectangles.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t();
                temp_record = Some(record);
            }
        }

        for object in self.yz_rectangles.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t();
                temp_record = Some(record);
            }
        }

        for object in self.aabb_boxes.iter() {
            if let Some(record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = record.t();
                temp_record = Some(record);
            }
        }

        temp_record
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::hittable_world::HittableWorld;
    use crate::geometry::sphere::Sphere;
    use crate::material::Material;
    use crate::math::vec3::Vec3;
    use crate::ray::Ray;

    #[test]
    fn hittable_world_hit_with_one_object() {
        let mut hittable_list = HittableWorld::new();
        let sphere = Sphere::new(
            Vec3::new(0.0, 0.0, 10.0),
            3.0,
            Material::new_dielectric(0.0),
        );
        hittable_list.add_sphere(sphere);

        let ray = Ray::new(Vec3::zero(), Vec3::forward());
        let result = hittable_list.hit_no_limit(&ray);

        assert!(result.is_some());
    }
}

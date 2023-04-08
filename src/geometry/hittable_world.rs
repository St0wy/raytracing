use crate::geometry::hit::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableWorld {
    hittables: Vec<Box<dyn Hittable>>,
}

impl HittableWorld {
    pub fn new() -> Self {
        Self {
            hittables: Vec::new(),
        }
    }

    pub fn add_hittable(&mut self, hittable: impl Hittable + 'static) {
        self.hittables.push(Box::new(hittable));
    }

    pub fn len(&self) -> usize {
        self.hittables.len()
    }

    pub fn clear(&mut self) {
        self.hittables.clear();
    }

    pub fn hit_no_limit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit(ray, 0.001, f32::INFINITY)
    }

    pub fn is_empty(&self) -> bool {
        self.hittables.is_empty()
    }
}

impl Hittable for HittableWorld {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut temp_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.hittables.iter() {
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
        hittable_list.add_hittable(sphere);

        let ray = Ray::new(Vec3::zero(), Vec3::forward());
        let result = hittable_list.hit_no_limit(&ray);

        assert!(result.is_some());
    }
}

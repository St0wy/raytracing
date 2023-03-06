use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

#[derive(Debug)]
pub struct HitRecord<'a> {
    point: Vec3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3,
        t: f32,
        outward_normal: Vec3,
        ray_direction: &Vec3,
        material: &'a Material,
    ) -> Self {
        let front_face = ray_direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn point(&self) -> &Vec3 {
        &self.point
    }

    pub fn material(&self) -> &Material {
        self.material
    }

    pub fn front_face(&self) -> bool {
        self.front_face
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    spheres: Vec<Sphere>,
    moving_spheres: Vec<MovingSphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            moving_spheres: Vec::new(),
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_moving_sphere(&mut self, moving_sphere: MovingSphere) {
        self.moving_spheres.push(moving_sphere);
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
    }

    pub fn hit_no_limit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit(ray, 0.001, f32::INFINITY)
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record_option: Option<HitRecord> = None;
        let mut closest_distance = t_max;

        for object in self.spheres.iter() {
            let record = object.hit(ray, t_min, closest_distance);
            if record.is_some() {
                let record = record.unwrap();
                closest_distance = record.t;
                record_option = Some(record);
            }
        }

        for object in self.moving_spheres.iter() {
            let record = object.hit(ray, t_min, closest_distance);
            if record.is_some() {
                let record = record.unwrap();
                closest_distance = record.t;
                record_option = Some(record);
            }
        }

        record_option
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::hit::HittableList;
    use crate::geometry::sphere::Sphere;
    use crate::material::Material;
    use crate::math::vec3::Vec3;
    use crate::ray::Ray;

    #[test]
    fn hittable_list_hit_with_one_object() {
        let mut hittable_list = HittableList::new();
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

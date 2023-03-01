use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

#[derive(Debug)]
pub struct HitRecord<'a> {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3,
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

    pub fn point(&self) -> &Point3 {
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
    objects: Vec<Sphere>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Sphere) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit_limit(ray, 0.001, f32::INFINITY)
    }

    pub fn hit_limit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut record_option: Option<HitRecord> = None;
        let mut closest_distance = t_max;

        for object in self.objects.iter() {
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
    use crate::math::vec3::{Point3, Vec3};
    use crate::ray::Ray;

    #[test]
    fn hittable_list_hit_with_one_object() {
        let mut hittable_list = HittableList::new();
        let sphere = Sphere::new(
            Point3::new(0.0, 0.0, 10.0),
            3.0,
            Material::new_dielectric(0.0),
        );
        hittable_list.add(sphere);

        let ray = Ray::new(Point3::zero(), Vec3::forward());
        let result = hittable_list.hit(&ray);

        assert!(result.is_some());
    }
}

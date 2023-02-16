use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

pub struct HitRecord<'a> {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: &'a Box<dyn Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Point3, t: f32, material: &'a Box<dyn Material>) -> Self {
        Self {
            point,
            normal: Vec3::zero(),
            t,
            front_face: false,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
    }

    pub fn normal(&self) -> &Vec3 {
        &self.normal
    }

    pub fn point(&self) -> &Point3 {
        &self.point
    }

    pub fn material(&self) -> &Box<dyn Material> {
        self.material
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

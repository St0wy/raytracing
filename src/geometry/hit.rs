use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

pub struct HitRecord<'a> {
    point: Point3,
    normal: Vec3,
    t: f32,
    front_face: bool,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Point3, t: f32, material: &'a Material) -> Self {
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

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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

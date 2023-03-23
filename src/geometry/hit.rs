use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};

#[derive(Debug)]
pub struct HitRecord<'a> {
    point: Vec3,
    normal: Vec3,
    t: f32,
    u: f32,
    v: f32,
    front_face: bool,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Vec3,
        t: f32,
        u: f32,
        v: f32,
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
            u,
            v,
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

    pub fn u(&self) -> f32 {
        self.u
    }

    pub fn v(&self) -> f32 {
        self.v
    }

    pub fn t(&self) -> f32 {
        self.t
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

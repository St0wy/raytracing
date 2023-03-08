use crate::geometry::aabb::Aabb;
use crate::geometry::bvh::BvhNode;
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::{math::vec3::*, ray::Ray};
use rand::Rng;
use std::cmp::Ordering;

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
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb>;
}

#[derive(Copy, Clone, Debug)]
pub enum HittableObjectType {
    Sphere,
    MovingSphere,
    BvhNode,
}

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

pub struct HittableList {
    spheres: Vec<Sphere>,
    moving_spheres: Vec<MovingSphere>,
    bvh_nodes: Vec<BvhNode>,
    first_node_index: usize,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            spheres: Vec::new(),
            moving_spheres: Vec::new(),
            bvh_nodes: Vec::new(),
            first_node_index: 0,
        }
    }

    pub fn add_sphere(&mut self, sphere: Sphere) {
        self.spheres.push(sphere);
    }

    pub fn add_moving_sphere(&mut self, moving_sphere: MovingSphere) {
        self.moving_spheres.push(moving_sphere);
    }

    pub fn len(&self) -> usize {
        self.spheres.len() + self.moving_spheres.len()
    }

    pub fn clear(&mut self) {
        self.spheres.clear();
    }

    pub fn hit_no_limit(&self, ray: &Ray) -> Option<HitRecord> {
        self.hit(ray, 0.001, f32::INFINITY)
    }

    pub fn hit_at(
        &self,
        hittable_object_index: &HittableObjectIndex,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
    ) -> Option<HitRecord> {
        match hittable_object_index.object_type {
            HittableObjectType::BvhNode => self.hit_node(
                &self.bvh_nodes[hittable_object_index.index],
                ray,
                t_min,
                t_max,
            ),
            HittableObjectType::Sphere => {
                self.spheres[hittable_object_index.index].hit(ray, t_min, t_max)
            }
            HittableObjectType::MovingSphere => {
                self.moving_spheres[hittable_object_index.index].hit(ray, t_min, t_max)
            }
        }
    }

    pub fn get_aabb(
        &self,
        hittable_object_index: HittableObjectIndex,
        time0: f32,
        time1: f32,
    ) -> Option<Aabb> {
        match hittable_object_index.object_type {
            HittableObjectType::Sphere => {
                self.spheres[hittable_object_index.index].bounding_box(time0, time1)
            }
            HittableObjectType::MovingSphere => {
                self.moving_spheres[hittable_object_index.index].bounding_box(time0, time1)
            }
            HittableObjectType::BvhNode => {
                Some(self.bvh_nodes[hittable_object_index.index].aabb().clone())
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.spheres.is_empty() && self.moving_spheres.is_empty()
    }

    fn hit_node(&self, node: &BvhNode, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !node.aabb().hit(ray, t_min, t_max) {
            return None;
        }

        let record_left_option = self.hit_at(node.left(), ray, t_min, t_max);
        let mut left_distance = t_max;
        let mut record = None;
        if let Some(record_left) = record_left_option {
            left_distance = record_left.t;
            record = Some(record_left);
        }

        let record_right = self.hit_at(node.right(), ray, t_min, left_distance);
        if let Some(record_right) = record_right {
            if left_distance < record_right.t {
                record
            } else {
                Some(record_right)
            }
        } else {
            record
        }
    }

    fn box_compare(
        &self,
        time0: f32,
        time1: f32,
        axis: usize,
    ) -> impl FnMut(&HittableObjectIndex, &HittableObjectIndex) -> Ordering + '_ {
        move |a, b| {
            let a_bbox = self.get_aabb(*a, time0, time1);
            let b_bbox = self.get_aabb(*b, time0, time1);
            if a_bbox.is_none() || b_bbox.is_none() {
                panic!("no bounding box in bvh node")
            }

            if a_bbox.unwrap().min()[axis] - b_bbox.unwrap().min()[axis] < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
    }

    fn create_node(
        &mut self,
        hittables: &mut [HittableObjectIndex],
        time0: f32,
        time1: f32,
    ) -> HittableObjectIndex {
        let axis = rand::thread_rng().gen_range(0..3) as usize;

        let len = hittables.len();
        let (left, right) = match len {
            0 => panic!("0 Hittables provided to node creation"),
            1 => (hittables[0].clone(), hittables[0].clone()),
            2 => (hittables[0].clone(), hittables[1].clone()),
            _ => {
                hittables.sort_unstable_by(self.box_compare(time0, time1, axis));
                let mid = len / 2;
                (
                    self.create_node(&mut hittables[0..mid], time0, time1),
                    self.create_node(&mut hittables[mid..len], time0, time1),
                )
            }
        };

        let left_box = self.get_aabb(left, time0, time1);
        let right_box = self.get_aabb(right, time0, time1);

        if left_box.is_none() || right_box.is_none() {
            panic!("No bounding box in Bvh Node");
        }

        let aabb = Aabb::surrounding_box(left_box.unwrap(), right_box.unwrap());

        let node = BvhNode::new(left, right, aabb);
        self.bvh_nodes.push(node);

        HittableObjectIndex::new(HittableObjectType::BvhNode, self.bvh_nodes.len() - 1)
    }

    pub fn init_bvh_nodes(&mut self, time0: f32, time1: f32) {
        let mut hittable = Vec::new();

        for i in 0..self.spheres.len() {
            hittable.push(HittableObjectIndex::new(HittableObjectType::Sphere, i))
        }

        for i in 0..self.moving_spheres.len() {
            hittable.push(HittableObjectIndex::new(
                HittableObjectType::MovingSphere,
                i,
            ))
        }

        let node = self.create_node(&mut hittable[..], time0, time1);
        self.first_node_index = node.index;
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let first = self.bvh_nodes.get(self.first_node_index);

        if first.is_none() {
            panic!("There should be nodes in the hittable list.");
        }

        self.hit_node(&first.unwrap(), ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let mut temp_box: Aabb;
        let mut output_box = Aabb::empty();
        let mut first_box = true;

        // TODO: Refactor this to not duplicate loop (maybe ask on the rust discord ?)
        for object in self.spheres.iter() {
            temp_box = object.bounding_box(time0, time1)?;
            output_box = if first_box {
                temp_box
            } else {
                Aabb::surrounding_box(output_box, temp_box)
            };

            first_box = false;
        }

        for object in self.moving_spheres.iter() {
            temp_box = object.bounding_box(time0, time1)?;
            output_box = if first_box {
                temp_box
            } else {
                Aabb::surrounding_box(output_box, temp_box)
            };

            first_box = false;
        }

        None
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

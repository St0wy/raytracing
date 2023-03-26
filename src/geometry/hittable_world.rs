use crate::geometry::aabb::Aabb;
use crate::geometry::aabb_box::AabbBox;
use crate::geometry::bvh::BvhNode;
use crate::geometry::hit::{HitRecord, Hittable, HittableObjectType};
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::sphere::Sphere;
use crate::geometry::xy_rectangle::XyRectangle;
use crate::geometry::xz_rectangle::XzRectangle;
use crate::geometry::yz_rectangle::YzRectangle;
use crate::ray::Ray;
use rand::Rng;
use std::cmp::Ordering;

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
    bvh_nodes: Vec<BvhNode>,
    first_node_index: usize,
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
            HittableObjectType::XyRectangle => {
                self.xy_rectangles[hittable_object_index.index].hit(ray, t_min, t_max)
            }
            HittableObjectType::XzRectangle => {
                self.xz_rectangles[hittable_object_index.index].hit(ray, t_min, t_max)
            }
            HittableObjectType::YzRectangle => {
                self.yz_rectangles[hittable_object_index.index].hit(ray, t_min, t_max)
            }
            HittableObjectType::AabbBox => {
                self.aabb_boxes[hittable_object_index.index].hit(ray, t_min, t_max)
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
            HittableObjectType::XyRectangle => {
                self.xy_rectangles[hittable_object_index.index].bounding_box(time0, time1)
            }
            HittableObjectType::XzRectangle => {
                self.xz_rectangles[hittable_object_index.index].bounding_box(time0, time1)
            }
            HittableObjectType::YzRectangle => {
                self.yz_rectangles[hittable_object_index.index].bounding_box(time0, time1)
            }
            HittableObjectType::BvhNode => {
                Some(self.bvh_nodes[hittable_object_index.index].aabb().clone())
            }
            HittableObjectType::AabbBox => {
                self.aabb_boxes[hittable_object_index.index].bounding_box(time0, time1)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.spheres.is_empty()
            && self.moving_spheres.is_empty()
            && self.xy_rectangles.is_empty()
            && self.xz_rectangles.is_empty()
            && self.yz_rectangles.is_empty()
            && self.aabb_boxes.is_empty()
    }

    fn hit_node(&self, node: &BvhNode, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if !node.aabb().hit(ray, t_min, t_max) {
            return None;
        }

        let record_left_option = self.hit_at(node.left(), ray, t_min, t_max);
        let mut left_distance = t_max;
        let mut record = None;
        if let Some(record_left) = record_left_option {
            left_distance = record_left.t();
            record = Some(record_left);
        }

        let record_right = self.hit_at(node.right(), ray, t_min, left_distance);
        if let Some(record_right) = record_right {
            if left_distance < record_right.t() {
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
            1 => (hittables[0], hittables[0]),
            2 => (hittables[0], hittables[1]),
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

    pub fn init_bvh_nodes(&mut self) {
        let mut hittables = Vec::new();

        for i in 0..self.spheres.len() {
            hittables.push(HittableObjectIndex::new(HittableObjectType::Sphere, i))
        }

        for i in 0..self.moving_spheres.len() {
            hittables.push(HittableObjectIndex::new(
                HittableObjectType::MovingSphere,
                i,
            ))
        }

        for i in 0..self.xy_rectangles.len() {
            hittables.push(HittableObjectIndex::new(HittableObjectType::XyRectangle, i));
        }

        for i in 0..self.xz_rectangles.len() {
            hittables.push(HittableObjectIndex::new(HittableObjectType::XzRectangle, i));
        }

        for i in 0..self.yz_rectangles.len() {
            hittables.push(HittableObjectIndex::new(HittableObjectType::YzRectangle, i));
        }

        for i in 0..self.aabb_boxes.len() {
            hittables.push(HittableObjectIndex::new(HittableObjectType::AabbBox, i));
        }

        let node = self.create_node(&mut hittables[..], 0.0, 1.0);
        self.first_node_index = node.index;
    }
}

fn get_objects_bounding_box<T: Hittable>(items: &Vec<T>, time0: f32, time1: f32) -> Option<Aabb> {
    if items.is_empty() {
        return None;
    }

    let mut temp_box: Aabb;
    let mut output_box = Aabb::empty();
    let mut first_box = true;

    for object in items.iter() {
        temp_box = object.bounding_box(time0, time1)?;

        output_box = if first_box {
            temp_box
        } else {
            Aabb::surrounding_box(output_box, temp_box)
        };

        first_box = false;
    }

    Some(output_box)
}

impl Hittable for HittableWorld {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let first = self.bvh_nodes.get(self.first_node_index);

        if first.is_none() {
            panic!("There should be nodes in the hittable list.");
        }

        self.hit_node(first.unwrap(), ray, t_min, t_max)
    }

    fn bounding_box(&self, time0: f32, time1: f32) -> Option<Aabb> {
        if self.is_empty() {
            return None;
        }

        let spheres_box = get_objects_bounding_box(&self.spheres, time0, time1);
        let moving_spheres_box = get_objects_bounding_box(&self.moving_spheres, time0, time1);
        let xy_rectangles_box = get_objects_bounding_box(&self.xy_rectangles, time0, time1);
        let xz_rectangles_box = get_objects_bounding_box(&self.xz_rectangles, time0, time1);
        let yz_rectangles_box = get_objects_bounding_box(&self.yz_rectangles, time0, time1);
        let aabb_box_box = get_objects_bounding_box(&self.aabb_boxes, time0, time1);

        let a = Aabb::opt_surrounding_box(spheres_box, moving_spheres_box);
        let b = Aabb::opt_surrounding_box(a, xy_rectangles_box);
        let c = Aabb::opt_surrounding_box(b, xz_rectangles_box);
        let d = Aabb::opt_surrounding_box(c, yz_rectangles_box);

        Aabb::opt_surrounding_box(d, aabb_box_box)
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::hittable_world::HittableWorld;
    use crate::geometry::sphere::Sphere;
    use crate::material::Material;
    use crate::ray::Ray;
    use glam::Vec3A;

    #[test]
    fn hittable_world_hit_with_one_object() {
        let mut hittable_list = HittableWorld::new();
        let sphere = Sphere::new(
            Vec3A::new(0.0, 0.0, 10.0),
            3.0,
            Material::new_dielectric(0.0),
        );
        hittable_list.add_sphere(sphere);
        hittable_list.init_bvh_nodes();

        let ray = Ray::new(Vec3A::ZERO, Vec3A::Y);
        let result = hittable_list.hit_no_limit(&ray);

        assert!(result.is_some());
    }
}

use crate::geometry::aabb::Aabb;
use crate::geometry::hittable_list::HittableObjectIndex;

#[derive(Debug)]
pub struct BvhNode {
    left: HittableObjectIndex,
    right: HittableObjectIndex,
    aabb: Aabb,
}

impl BvhNode {
    pub fn left(&self) -> &HittableObjectIndex {
        &self.left
    }
    pub fn right(&self) -> &HittableObjectIndex {
        &self.right
    }
    pub fn aabb(&self) -> &Aabb {
        &self.aabb
    }
    pub fn new(left: HittableObjectIndex, right: HittableObjectIndex, aabb: Aabb) -> Self {
        Self { left, right, aabb }
    }
}

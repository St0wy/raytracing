use crate::ray::Ray;
use glam::Vec3A;
use tracy_full::zone;

#[derive(Clone, Debug)]
pub struct Aabb {
    minimum: Vec3A,
    maximum: Vec3A,
}

impl Aabb {
    pub const fn new(minimum: Vec3A, maximum: Vec3A) -> Self {
        Self { minimum, maximum }
    }

    pub const fn empty() -> Self {
        Self::new(Vec3A::ZERO, Vec3A::ZERO)
    }

    pub const fn min(&self) -> Vec3A {
        self.minimum
    }

    pub const fn max(&self) -> Vec3A {
        self.maximum
    }

    pub fn hit(&self, ray: &Ray, mut t_min: f32, mut t_max: f32) -> bool {
        zone!();
        for i in 0..3 {
            let inverse_direction = 1.0 / ray.direction()[i];
            let mut t0 = (self.min()[i] - ray.origin()[i]) * inverse_direction;
            let mut t1 = (self.max()[i] - ray.origin()[i]) * inverse_direction;

            if inverse_direction < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t0.max(t_min);
            t_max = t1.min(t_max);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }

    pub fn opt_surrounding_box(box0: Option<Self>, box1: Option<Self>) -> Option<Self> {
        if box0.is_none() {
            return box1;
        }

        if box1.is_none() {
            return box0;
        }

        Some(Self::surrounding_box(box0?, box1?))
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let small = Vec3A::new(
            box0.min().x.min(box1.min().x),
            box0.min().y.min(box1.min().y),
            box0.min().z.min(box1.min().z),
        );

        let big = Vec3A::new(
            box0.max().x.max(box1.max().x),
            box0.max().y.max(box1.max().y),
            box0.max().z.max(box1.max().z),
        );

        Aabb::new(small, big)
    }
}

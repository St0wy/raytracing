use crate::math::vec3::*;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Creates a new ray with an origin and a direction.
    ///
    /// # Arguments
    ///
    /// * `origin`: Origin of the ray.
    /// * `direction`: Direction of the ray, should be normalized.
    ///
    /// returns: Ray
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::*;
    /// use raytracing::ray::Ray;
    /// let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    /// assert_eq!(ray.origin(), Point3::new(0.0, 0.0, 0.0));
    /// assert_eq!(ray.direction(), Vec3::new(1.0, 0.0, 0.0));
    /// ```
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Gets the point along the ray at the specified distance.
    ///
    /// # Arguments
    ///
    /// * `t`: Distance to the origin of the ray.
    ///
    /// returns: Vec3
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::*;
    /// use raytracing::ray::Ray;
    /// let ray = Ray::new(Point3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
    /// assert_eq!(ray.at(3.0), Vec3::new(4.0, 0.0, 0.0));
    /// ```
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.direction
    }
}

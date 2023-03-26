use glam::Vec3A;

#[derive(Debug, Copy, Clone, Default)]
pub struct Ray {
    origin: Vec3A,
    direction: Vec3A,
    pub time: f32,
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
    /// use glam::Vec3A;
    /// use raytracing::ray::Ray;
    /// let ray = Ray::new(Vec3A::new(0.0, 0.0, 0.0), Vec3A::new(1.0, 0.0, 0.0));
    /// assert_eq!(ray.origin(), Vec3A::new(0.0, 0.0, 0.0));
    /// assert_eq!(ray.direction(), Vec3A::new(1.0, 0.0, 0.0));
    /// ```
    pub fn new(origin: Vec3A, direction: Vec3A) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn origin(&self) -> Vec3A {
        self.origin
    }

    pub fn direction(&self) -> Vec3A {
        self.direction
    }

    /// Gets the point along the ray at the specified distance.
    ///
    /// # Arguments
    ///
    /// * `t`: Distance to the origin of the ray.
    ///
    /// returns: Vec3A
    ///
    /// # Examples
    ///
    /// ```
    /// use glam::Vec3A;
    /// use raytracing::ray::Ray;
    /// let ray = Ray::new(Vec3A::new(1.0, 0.0, 0.0), Vec3A::new(1.0, 0.0, 0.0));
    /// assert_eq!(ray.at(3.0), Vec3A::new(4.0, 0.0, 0.0));
    /// ```
    pub fn at(&self, t: f32) -> Vec3A {
        self.origin + t * self.direction
    }
}

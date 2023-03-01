use rand::distributions::Distribution;
use rand::distributions::Uniform;
use rand::Rng;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, RangeInclusive,
    Sub,
};

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    /// Creates a new [Vec3]
    ///
    /// returns: Vec3
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let vec = Vec3::new(1.0, 2.0, 3.0);
    /// assert_eq!(vec.x, 1.0);
    /// assert_eq!(vec.y, 2.0);
    /// assert_eq!(vec.z, 3.0);
    /// ```
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Computes the squared magnitude of the vector (aka. the magnitude of the vector to the power of two)
    ///
    /// It can then be passed to `.sqrt()` to compute the real magnitude.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let vec = Vec3::new(1.0, 1.0, 1.0);
    /// assert_eq!(vec.squared_magnitude(), 3.0);
    /// ```
    pub fn squared_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Computes the magnitude of the vector (aka. the length of the vector).
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let vec = Vec3::new(1.0, 1.0, 1.0);
    /// assert_eq!(vec.magnitude(), f32::sqrt(3.0));
    /// ```
    pub fn magnitude(&self) -> f32 {
        self.squared_magnitude().sqrt()
    }

    /// Computes the dot product of this vector and the other one. (aka. scalar product)
    ///
    /// # Arguments
    ///
    /// * `other`: The vector to compute the dot product with.
    ///
    /// returns: f32
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let vec1 = Vec3::new(1.0, 2.0, 3.0);
    /// let vec2 = Vec3::new(4.0, 5.0, 6.2);
    /// assert_eq!(vec1.dot(&vec2), 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.2)
    /// ```
    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product (aka. vector product).
    ///
    /// # Arguments
    ///
    /// * `other`: The other vector to compute the cross product with.
    ///
    /// returns: Vec3 A vector perpendicular to the other two.
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let vec1 = Vec3::new(1.0, 0.0, 0.0);
    /// let vec2 = Vec3::new(0.0, 1.0, 0.0);
    /// let cross = vec1.cross(vec2);
    /// assert_eq!(cross.x, 0.0);
    /// assert_eq!(cross.y, 0.0);
    /// assert_eq!(cross.z, 1.0);
    /// ```
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Returns the vector as a unit vector (with a magnitude of 1).
    ///
    /// # Example
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let vec = Vec3::new(2.0, 3.0, 4.0);
    /// let unit = vec.to_unit();
    /// assert_eq!(unit.x, 0.37139067);
    /// assert_eq!(unit.y, 0.557086);
    /// assert_eq!(unit.z, 0.74278134);
    /// assert_eq!(unit.magnitude(), 1.0);
    /// ```
    pub fn to_unit(self) -> Self {
        self / self.magnitude()
    }

    /// Gets a random vector where x, y and z are in the range 0..1.
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let random = Vec3::random();
    /// assert!(random.x >= 0.0 && random.x < 1.0);
    /// assert!(random.y >= 0.0 && random.y < 1.0);
    /// assert!(random.z >= 0.0 && random.z < 1.0);
    /// ```
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen(), rng.gen(), rng.gen())
    }

    /// Gets a random vector where x, y and z are in the specified range.
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let random = Vec3::random_range(2.0..5.0);
    /// assert!(random.x >= 2.0 && random.x < 5.0);
    /// assert!(random.y >= 2.0 && random.y < 5.0);
    /// assert!(random.z >= 2.0 && random.z < 5.0);
    /// ```
    pub fn random_range(range: Range<f32>) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(range);
        Self::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        )
    }

    /// Gets a random vector where x, y and z are in the specified inclusive range.
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let random = Vec3::random_range_inclusive(2.0..=5.0);
    /// assert!(random.x >= 2.0 && random.x <= 5.0);
    /// assert!(random.y >= 2.0 && random.y <= 5.0);
    /// assert!(random.z >= 2.0 && random.z <= 5.0);
    /// ```
    pub fn random_range_inclusive(range: RangeInclusive<f32>) -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(range);
        Self::new(
            between.sample(&mut rng),
            between.sample(&mut rng),
            between.sample(&mut rng),
        )
    }

    /// Gets a random vector that is inside a unit sphere (where magnitude is < 1.0).
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let random = Vec3::random_in_unit_sphere();
    /// assert!(random.x >= -1.0 && random.x < 1.0);
    /// assert!(random.y >= -1.0 && random.y < 1.0);
    /// assert!(random.z >= -1.0 && random.z < 1.0);
    /// assert!(random.magnitude() < 1.0);
    /// ```
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let point = Self::random_range(-1.0..1.0);
            if point.squared_magnitude() < 1.0 {
                return point;
            }
        }
    }

    /// Gets a random vector that is along the edge of a unit sphere (where magnitude is 1.0).
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let random = Vec3::random_unit_normalized();
    /// let diff = f32::abs(random.magnitude() - 1.0);
    /// assert!(diff < 0.00001);
    /// ```
    pub fn random_unit_normalized() -> Self {
        Self::random_in_unit_sphere().to_unit()
    }

    /// Gets a random vector where x and y are inside a unit circle and z is 0.
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let random = Vec3::random_in_unit_circle();
    /// assert!(random.x >= -1.0 && random.x < 1.0);
    /// assert!(random.y >= -1.0 && random.y < 1.0);
    /// assert!(random.z >= -1.0 && random.z < 1.0);
    /// assert!(random.magnitude() < 1.0);
    /// ```
    pub fn random_in_unit_circle() -> Self {
        let mut rng = rand::thread_rng();
        let between = Uniform::from(-1.0..1.0);
        loop {
            let point = Vec3::new(between.sample(&mut rng), between.sample(&mut rng), 0.0);
            if point.squared_magnitude() < 1.0 {
                return point;
            }
        }
    }

    /// Returns true if all of the elements in the vector are near 0.
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let zero = Vec3::zero();
    /// assert!(zero.is_near_zero());
    /// ```
    pub fn is_near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }

    /// Reflects this vector on the provided normal.
    ///
    /// # Arguments
    ///
    /// * `normal`: Normal to reflect on.
    ///
    /// returns: Vec3
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let normal = Vec3::new(0.14074421, 0.9846618, 0.10285115);
    /// let vec = Vec3::new(-0.94833744, -0.0015498066, -0.31725982);
    /// let reflection = vec.reflect(&normal);
    /// assert_eq!(reflection.x, -0.90115166);
    /// assert_eq!(reflection.y, 0.32856706);
    /// assert_eq!(reflection.z, -0.28277802);
    /// ```
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - (2.0 * self.dot(normal) * *normal)
    }

    /// Refracts this vector on the provided normal.
    ///
    /// # Arguments
    ///
    /// * `normal`: Normal to refracts on.
    /// * `refraction_ratio`: This is η / η′. η (pronounced "eta") is the refraction index.
    ///
    /// returns: Vec3
    ///
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let normal = Vec3::new(0.97541153, 0.017504334, 0.21969497);
    /// let vec = Vec3::new( -0.7097238, -0.67490864, -0.20196645);
    /// let refracted = vec.refract(&normal, 1.5);
    /// assert_eq!(refracted.x, -0.06909665);
    /// assert_eq!(refracted.y, -0.9944983);
    /// assert_eq!(refracted.z, -0.078732595);
    /// ```
    pub fn refract(&self, normal: &Vec3, refraction_ratio: f32) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = refraction_ratio * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perp.squared_magnitude()).abs().sqrt() * *normal;

        r_out_perp + r_out_parallel
    }

    /// Creates a vector where x, y and z are 0.
    ///
    /// # Example
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let zero = Vec3::zero();
    /// assert_eq!(zero.x, 0.0);
    /// assert_eq!(zero.y, 0.0);
    /// assert_eq!(zero.z, 0.0);
    /// ```
    pub const fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub const fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub const fn forward() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = Vec3::new(1.0, 1.0, 1.0);
    /// let b = Vec3::new(1.0, 1.0, 1.0);
    /// let c = a + b;
    /// assert_eq!(c.x, 2.0);
    /// assert_eq!(c.y, 2.0);
    /// assert_eq!(c.z, 2.0);
    /// ```
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = Vec3::new(2.0, 2.0, 2.0);
    /// let b = Vec3::new(1.0, 1.0, 1.0);
    /// let c = a - b;
    /// assert_eq!(c.x, 1.0);
    /// assert_eq!(c.y, 1.0);
    /// assert_eq!(c.z, 1.0);
    /// ```
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = Vec3::new(2.0, 2.0, 2.0);
    /// let b = Vec3::new(1.5, 1.5, 1.5);
    /// let c = a * b;
    /// assert_eq!(c.x, 3.0);
    /// assert_eq!(c.y, 3.0);
    /// assert_eq!(c.z, 3.0);
    /// ```
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = 2.0;
    /// let b = Vec3::new(1.5, 1.5, 1.5);
    /// let c = a * b;
    /// assert_eq!(c.x, 3.0);
    /// assert_eq!(c.y, 3.0);
    /// assert_eq!(c.z, 3.0);
    /// ```
    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs * self
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let b = 2.0;
    /// let a = Vec3::new(1.5, 1.5, 1.5);
    /// let c = a * b;
    /// assert_eq!(c.x, 3.0);
    /// assert_eq!(c.y, 3.0);
    /// assert_eq!(c.z, 3.0);
    /// ```
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = Vec3::new(3.0, 3.0, 3.0);
    /// let b = 2.0;
    /// let c = a / b;
    /// assert_eq!(c.x, 1.5);
    /// assert_eq!(c.y, 1.5);
    /// assert_eq!(c.z, 1.5);
    /// ```
    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Neg for Vec3 {
    type Output = Self;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = Vec3::new(3.0, 3.0, 3.0);
    /// let neg = -a;
    /// assert_eq!(neg.x, -3.0);
    /// assert_eq!(neg.y, -3.0);
    /// assert_eq!(neg.z, -3.0);
    /// ```
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let mut a = Vec3::new(3.0, 3.0, 3.0);
    /// let b = Vec3::new(1.0, 1.0, 1.0);
    /// a += b;
    /// assert_eq!(a.x, 4.0);
    /// assert_eq!(a.y, 4.0);
    /// assert_eq!(a.z, 4.0);
    /// ```
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl MulAssign<f32> for Vec3 {
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let mut a = Vec3::new(3.0, 3.0, 3.0);
    /// let b = 2.0;
    /// a *= b;
    /// assert_eq!(a.x, 6.0);
    /// assert_eq!(a.y, 6.0);
    /// assert_eq!(a.z, 6.0);
    /// ```
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl DivAssign<f32> for Vec3 {
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let mut a = Vec3::new(3.0, 3.0, 3.0);
    /// let b = 2.0;
    /// a /= b;
    /// assert_eq!(a.x, 1.5);
    /// assert_eq!(a.y, 1.5);
    /// assert_eq!(a.z, 1.5);
    /// ```
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let a = Vec3::new(2.0, 3.0, 4.0);
    /// assert_eq!(a[0], 2.0);
    /// assert_eq!(a[1], 3.0);
    /// assert_eq!(a[2], 4.0);
    /// ```
    fn index(&self, i: usize) -> &f32 {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Out of vector bounds !"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    /// # Examples
    ///
    /// ```
    /// use raytracing::math::vec3::Vec3;
    /// let mut a = Vec3::new(2.0, 3.0, 4.0);
    /// a[0] += 3.0;
    /// assert_eq!(a[0], 5.0);
    /// assert_eq!(a[1], 3.0);
    /// assert_eq!(a[2], 4.0);
    /// ```
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Out of vector bounds !"),
        }
    }
}

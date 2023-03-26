use crate::consts::ASPECT_RATIO;
use crate::math::vec3::Vec3Ext;
use crate::ray::Ray;
use glam::Vec3A;
use rand::Rng;
use tracy_full::zone;

pub struct Camera {
    origin: Vec3A,
    lower_left_corner: Vec3A,
    horizontal: Vec3A,
    vertical: Vec3A,
    u: Vec3A,
    v: Vec3A,
    lens_radius: f32,
    time0: f32,
    time1: f32,
}

impl Camera {
    pub fn new(
        look_from: Vec3A,
        look_at: Vec3A,
        vup: Vec3A,
        vertical_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w,
            u,
            v,
            lens_radius: aperture / 2.0,
            time0: 0.0,
            time1: 0.0,
        }
    }

    pub fn new_look(look_from: Vec3A, look_at: Vec3A) -> Self {
        let mut cam = Self::new(look_from, look_at, Vec3A::Y, 20.0, ASPECT_RATIO, 0.1, 10.0);
        cam.set_time(0.0, 1.0);

        cam
    }

    pub fn new_look_fov(look_from: Vec3A, look_at: Vec3A, fov: f32) -> Self {
        let mut cam = Self::new(look_from, look_at, Vec3A::Y, fov, ASPECT_RATIO, 0.1, 10.0);
        cam.set_time(0.0, 1.0);

        cam
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        zone!();
        let rd = self.lens_radius * Vec3A::random_in_unit_circle();
        let offset = self.u * rd.x + self.v * rd.y;

        let mut ray = Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        );
        let mut rng = rand::thread_rng();
        ray.time = rng.gen_range(self.time0..self.time1);

        ray
    }

    pub fn set_time(&mut self, time0: f32, time1: f32) {
        self.time0 = time0;
        self.time1 = time1;
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut cam = Self::new(
            Vec3A::new(13.0, 2.0, 3.0),
            Vec3A::ZERO,
            Vec3A::Y,
            20.0,
            ASPECT_RATIO,
            0.1,
            10.0,
        );
        cam.set_time(0.0, 1.0);

        cam
    }
}

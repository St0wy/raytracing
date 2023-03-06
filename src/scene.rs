use crate::camera::Camera;
use crate::geometry::hit::HittableList;
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::sphere::Sphere;
use crate::material::Material;
use crate::math::color::Color;
use crate::math::vec3::Vec3;
use rand::Rng;

pub struct Scene {
    hittable_list: HittableList,
    camera: Camera,
}

impl Scene {
    pub fn new(hittable_list: HittableList, camera: Camera) -> Self {
        Self {
            hittable_list,
            camera,
        }
    }

    pub fn random() -> Self {
        Self {
            hittable_list: random_hittable_list(),
            camera: Camera::default(),
        }
    }

    pub fn hittable_list(&self) -> &HittableList {
        &self.hittable_list
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn set_hittable_list(&mut self, hittable_list: HittableList) {
        self.hittable_list = hittable_list;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
}

fn random_hittable_list() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Material::new_lambertian(Color::new(0.5, 0.5, 0.5));
    world.add_sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    ));

    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    let sphere_material = Material::new_lambertian(albedo);
                    let center2 = center + Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add_moving_sphere(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ));
                    // world.add(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // Metal 🤘
                    let albedo = Color::random_range(0.5..1.0);
                    let fuzz = rng.gen_range(0.0..0.5);
                    let sphere_material = Material::new_metal(albedo, fuzz);
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // Glass
                    let sphere_material = Material::new_dielectric(1.5);
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material = Material::new_dielectric(1.5);
    world.add_sphere(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add_sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material));

    world
}

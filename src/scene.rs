use crate::camera::Camera;
use crate::consts::ASPECT_RATIO;
use crate::geometry::aabb_box::AabbBox;
use crate::geometry::hittable_world::HittableWorld;
use crate::geometry::moving_sphere::MovingSphere;
use crate::geometry::sphere::Sphere;
use crate::geometry::xy_rectangle::XyRectangle;
use crate::geometry::xz_rectangle::XzRectangle;
use crate::geometry::yz_rectangle::YzRectangle;
use crate::material::Material;
use crate::math::color::Color;
use crate::math::perlin::Perlin;
use crate::texture::Texture;
use glam::Vec3A;
use rand::{Rng, SeedableRng};
use rand_xoshiro::rand_core::RngCore;
use tracy_full::zone;

pub struct Scene {
    hittable_list: HittableWorld,
    camera: Camera,
    background_color: Color,
}

impl Scene {
    pub fn new(hittable_list: HittableWorld, camera: Camera, background_color: Color) -> Self {
        Self {
            hittable_list,
            camera,
            background_color,
        }
    }

    pub fn bench_three_spheres() -> Self {
        let mut world = HittableWorld::new();

        let material = Material::new_dielectric(1.5);
        world.add_sphere(Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, material));
        let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
        world.add_sphere(Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, material));
        let material = Material::new_lambertian_color(Color::new(0.4, 0.2, 0.1));
        world.add_sphere(Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, material));
        world.init_bvh_nodes();

        Self::new(world, Camera::default(), Color::new(0.70, 0.80, 1.00))
    }

    pub fn random(rng: &mut impl RngCore) -> Self {
        Self {
            hittable_list: random_hittable_list(rng),
            camera: Camera::default(),
            background_color: Color::new(0.70, 0.80, 1.00),
        }
    }

    pub fn big_scene() -> Self {
        Self {
            hittable_list: fixed_big_scene(),
            camera: Camera::default(),
            background_color: Color::new(0.70, 0.80, 1.00),
        }
    }

    pub fn two_spheres() -> Self {
        let mut hittable_list = HittableWorld::new();
        let checker = Texture::new_checker(
            Texture::new_solid_color(Color::new(0.2, 0.3, 0.1)),
            Texture::new_solid_color(Color::new(0.9, 0.9, 0.9)),
        );

        hittable_list.add_sphere(Sphere::new(
            Vec3A::new(0.0, -10.0, 0.0),
            10.0,
            Material::new_lambertian(checker.clone()),
        ));
        hittable_list.add_sphere(Sphere::new(
            Vec3A::new(0.0, 10.0, 0.0),
            10.0,
            Material::new_lambertian(checker),
        ));
        hittable_list.init_bvh_nodes();

        Self {
            hittable_list,
            camera: Camera::default(),
            background_color: Color::new(0.70, 0.80, 1.00),
        }
    }

    pub fn two_perlin_spheres(rng: &mut impl RngCore) -> Self {
        let mut hittable_list = HittableWorld::new();
        let perlin_texture = Texture::new_noise(Perlin::new(rng), 4.0);

        hittable_list.add_sphere(Sphere::new(
            Vec3A::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::new_lambertian(perlin_texture.clone()),
        ));
        hittable_list.add_sphere(Sphere::new(
            Vec3A::new(0.0, 2.0, 0.0),
            2.0,
            Material::new_lambertian(perlin_texture),
        ));
        hittable_list.init_bvh_nodes();

        Self {
            hittable_list,
            camera: Camera::default(),
            background_color: Color::new(0.70, 0.80, 1.00),
        }
    }

    pub fn perlin_and_earth(rng: &mut impl RngCore) -> Self {
        let mut hittable_list = HittableWorld::new();
        let perlin_texture = Texture::new_noise(Perlin::new(rng), 4.0);
        let earth_texture =
            Texture::new_image("earthmap.png".to_string()).expect("Failed to load earth texture");
        let earth_surface = Material::new_lambertian(earth_texture);

        hittable_list.add_sphere(Sphere::new(
            Vec3A::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::new_lambertian(perlin_texture),
        ));
        hittable_list.add_sphere(Sphere::new(Vec3A::new(0.0, 2.0, 0.0), 2.0, earth_surface));
        hittable_list.init_bvh_nodes();

        Self {
            hittable_list,
            camera: Camera::default(),
            background_color: Color::new(0.70, 0.80, 1.00),
        }
    }

    pub fn earth() -> Self {
        let mut hittable_list = HittableWorld::new();
        let earth_texture =
            Texture::new_image("earthmap.png".to_string()).expect("Failed to load image texture");
        let earth_surface = Material::new_lambertian(earth_texture);
        let globe = Sphere::new(Vec3A::new(0.0, 0.0, 0.0), 2.0, earth_surface);
        hittable_list.add_sphere(globe);
        hittable_list.init_bvh_nodes();

        Self {
            hittable_list,
            camera: Camera::default(),
            background_color: Color::new(0.70, 0.80, 1.00),
        }
    }

    pub fn simple_light(rng: &mut impl RngCore) -> Self {
        let mut hittable_list = HittableWorld::new();

        let perlin_texture = Texture::new_noise(Perlin::new(rng), 4.0);
        let ground = Sphere::new(
            Vec3A::new(0.0, -1000.0, 0.0),
            1000.0,
            Material::new_lambertian(perlin_texture.clone()),
        );
        hittable_list.add_sphere(ground);

        let sphere = Sphere::new(
            Vec3A::new(0.0, 2.0, 0.0),
            2.0,
            Material::new_lambertian(perlin_texture),
        );
        hittable_list.add_sphere(sphere);

        let diffuse_light = Material::new_diffuse_light_color(Color::new(4.0, 4.0, 4.0));
        hittable_list.add_xy_rectangle(XyRectangle::new(diffuse_light, 3.0, 5.0, 1.0, 3.0, -2.0));
        hittable_list.init_bvh_nodes();

        let mut camera = Camera::new(
            Vec3A::new(26.0, 3.0, 6.0),
            Vec3A::new(0.0, 0.0, 0.0),
            Vec3A::Y,
            20.0,
            ASPECT_RATIO,
            0.0,
            10.0,
        );
        camera.set_time(0.0, 1.0);

        Self {
            hittable_list,
            camera,
            background_color: Color::black(),
        }
    }

    pub fn cornell_box() -> Self {
        zone!();
        let mut hittable_list = HittableWorld::new();
        let red = Material::new_lambertian_color(Color::new(0.65, 0.05, 0.05));
        let white = Material::new_lambertian_color(Color::new(0.73, 0.73, 0.73));
        let green = Material::new_lambertian_color(Color::new(0.12, 0.45, 0.15));
        let force = 15.0;
        let light = Material::new_diffuse_light_color(Color::new(force, force, force));

        hittable_list.add_yz_rectangle(YzRectangle::new(green, 0.0, 555.0, 0.0, 555.0, 555.0));
        hittable_list.add_yz_rectangle(YzRectangle::new(red, 0.0, 555.0, 0.0, 555.0, 0.0));
        let size = 30.0;
        hittable_list.add_xz_rectangle(XzRectangle::new(
            light,
            213.0 - size,
            343.0 + size,
            227.0 - size,
            332.0 + size,
            554.0,
        ));
        hittable_list.add_xz_rectangle(XzRectangle::new(
            white.clone(),
            0.0,
            555.0,
            0.0,
            555.0,
            0.0,
        ));
        hittable_list.add_xz_rectangle(XzRectangle::new(
            white.clone(),
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
        ));
        hittable_list.add_xy_rectangle(XyRectangle::new(
            white.clone(),
            0.0,
            555.0,
            0.0,
            555.0,
            555.0,
        ));
        hittable_list.add_aabb_box(AabbBox::new(
            Vec3A::new(130.0, 0.0, 65.0),
            Vec3A::new(295.0, 165.0, 230.0),
            white.clone(),
        ));
        hittable_list.add_aabb_box(AabbBox::new(
            Vec3A::new(265.0, 0.0, 295.0),
            Vec3A::new(430.0, 330.0, 460.0),
            white,
        ));

        let mut camera = Camera::new(
            Vec3A::new(278.0, 278.0, -800.0),
            Vec3A::new(278.0, 278.0, 0.0),
            Vec3A::Y,
            40.0,
            ASPECT_RATIO,
            0.0,
            10.0,
        );
        camera.set_time(0.0, 1.0);
        hittable_list.init_bvh_nodes();

        Self::new(hittable_list, camera, Color::black())
    }

    pub fn hittable_list(&self) -> &HittableWorld {
        &self.hittable_list
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn background_color(&self) -> &Color {
        &self.background_color
    }

    pub fn set_hittable_list(&mut self, hittable_list: HittableWorld) {
        self.hittable_list = hittable_list;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }
}

fn fixed_big_scene() -> HittableWorld {
    let mut world = HittableWorld::new();

    let checker = Texture::new_checker(
        Texture::new_solid_color(Color::new(0.2, 0.3, 0.1)),
        Texture::new_solid_color(Color::new(0.9, 0.9, 0.9)),
    );
    let material_ground = Material::new_lambertian(checker);
    world.add_sphere(Sphere::new(
        Vec3A::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    ));

    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3A::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse moving
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    let sphere_material = Material::new_lambertian_color(albedo);
                    let center2 = center + Vec3A::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add_moving_sphere(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.8 {
                    // Diffuse not moving
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    let sphere_material = Material::new_lambertian_color(albedo);
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // Metal 🤘
                    let albedo = Color::random_range(0.5..1.0, &mut rng);
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
    world.add_sphere(Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian_color(Color::new(0.4, 0.2, 0.1));
    world.add_sphere(Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, material));
    world.init_bvh_nodes();

    world
}

fn random_hittable_list(rng: &mut impl RngCore) -> HittableWorld {
    let mut world = HittableWorld::new();

    let material_ground = Material::new_lambertian_color(Color::new(0.5, 0.5, 0.5));
    world.add_sphere(Sphere::new(
        Vec3A::new(0.0, -1000.0, -1.0),
        1000.0,
        material_ground,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f32 = rng.gen();
            let center = Vec3A::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3A::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // Diffuse moving
                    let albedo = Color::random(rng) * Color::random(rng);
                    let sphere_material = Material::new_lambertian_color(albedo);
                    let center2 = center + Vec3A::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.add_moving_sphere(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ));
                } else if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random(rng) * Color::random(rng);
                    let sphere_material = Material::new_lambertian_color(albedo);
                    world.add_sphere(Sphere::new(center, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // Metal 🤘
                    let albedo = Color::random_range(0.5..1.0, rng);
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
    world.add_sphere(Sphere::new(Vec3A::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian_color(Color::new(0.4, 0.2, 0.1));
    world.add_sphere(Sphere::new(Vec3A::new(-4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Sphere::new(Vec3A::new(4.0, 1.0, 0.0), 1.0, material));
    world.init_bvh_nodes();

    world
}

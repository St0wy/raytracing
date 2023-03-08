use criterion::{criterion_group, criterion_main, Criterion};
use raytracing::camera::Camera;
use raytracing::consts::*;
use raytracing::geometry::hit::HittableList;
use raytracing::geometry::sphere::Sphere;
use raytracing::material::Material;
use raytracing::math::color::Color;
use raytracing::math::vec3::*;
use raytracing::renderer::{render_no_bar_multithreaded};
use raytracing::scene::Scene;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut world = HittableList::new();

    let material = Material::new_dielectric(1.5);
    world.add_sphere(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add_sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add_sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, material));
    world.init_bvh_nodes(0.0, 1.0);

    let camera = Camera::default();

    let scene = Scene::new(world, camera);

    c.bench_function("render", |b| {
        b.iter(|| {
            render_no_bar_multithreaded(&scene, IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs_f64(130.0));
    targets = criterion_benchmark
}
criterion_main!(benches);

use criterion::{criterion_group, criterion_main, Criterion};
use raytracing::camera::Camera;
use raytracing::consts::*;
use raytracing::geometry::hit::HittableList;
use raytracing::geometry::sphere::Sphere;
use raytracing::material::Material;
use raytracing::math::vec3::*;
use raytracing::renderer::render_single_core_no_bar;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut world = HittableList::new();

    let material = Material::new_dielectric(1.5);
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_lambertian(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material));
    let material = Material::new_metal(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material));

    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
    );

    c.bench_function("render", |b| {
        b.iter(|| {
            render_single_core_no_bar(&world, &camera, IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs_f64(130.0));
    targets = criterion_benchmark
}
criterion_main!(benches);

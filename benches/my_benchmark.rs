use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand_xoshiro::rand_core::SeedableRng;
use raytracing::consts::*;
use raytracing::renderer::render;
use raytracing::scene::Scene;

fn bench_three_spheres(c: &mut Criterion) {
    let scene = Scene::bench_three_sphere();

    c.bench_function("render three_spheres", |b| {
        b.iter(|| {
            render(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

fn bench_big_scene(c: &mut Criterion) {
    let scene = Scene::big_scene();

    c.bench_function("render big_scene", |b| {
        b.iter(|| {
            render(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

fn bench_cornell_box(c: &mut Criterion) {
    let scene = Scene::cornell_box();

    c.bench_function("render cornell_box", |b| {
        b.iter(|| {
            render(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

fn bench_perlin_and_earth(c: &mut Criterion) {
    let mut rng = rand_xoshiro::Xoshiro256Plus::seed_from_u64(0);
    let scene = Scene::perlin_and_earth(&mut rng);

    c.bench_function("render perlin_and_earth", |b| {
        b.iter(|| {
            render(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_three_spheres, bench_big_scene, bench_cornell_box, bench_perlin_and_earth
}
criterion_main!(benches);

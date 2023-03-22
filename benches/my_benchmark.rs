use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracing::consts::*;
use raytracing::renderer::render_no_bar_multithreaded;
use raytracing::scene::Scene;
use std::time::Duration;

fn bench_three_spheres(c: &mut Criterion) {
    let scene = Scene::bench_three_sphere();

    c.bench_function("render three_spheres", |b| {
        b.iter(|| {
            render_no_bar_multithreaded(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

fn bench_big_scene(c: &mut Criterion) {
    let scene = Scene::big_scene();

    c.bench_function("render big_scene", |b| {
        b.iter(|| {
            render_no_bar_multithreaded(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

fn bench_cornell_box(c: &mut Criterion) {
    let scene = Scene::cornell_box();

    c.bench_function("render cornell_box", |b| {
        b.iter(|| {
            render_no_bar_multithreaded(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

fn bench_perlin_and_earth(c: &mut Criterion) {
    let scene = Scene::perlin_and_earth();

    c.bench_function("render perlin_and_earth", |b| {
        b.iter(|| {
            render_no_bar_multithreaded(black_box(&scene), IMAGE_WIDTH, IMAGE_HEIGHT);
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs_f64(170.0));
    targets = bench_three_spheres, bench_big_scene, bench_cornell_box, bench_perlin_and_earth
}
criterion_main!(benches);

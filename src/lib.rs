pub mod camera;
pub mod consts;
pub mod geometry;
pub mod material;
pub mod math;
pub mod ray;
pub mod renderer;
pub mod scene;
pub mod texture;

use consts::*;
use human_time::ToHumanTimeString;
use std::io::BufWriter;
use std::path::Path;
use std::{fs::File, time::Instant};
use tracy_full::zone;

use crate::renderer::{render_no_bar, render_no_bar_multithreaded};
use crate::scene::Scene;

const FILE_DEFAULT_PATH: &str = "out.png";

pub fn run_big_scene() {
    zone!();
    if cfg!(debug_assertions) {
        println!("You are rendering this scene in debug, which is very slow. Make sure you don't want to run it in release mode.")
    }

    println!("Starting ray tracing...");
    let start = Instant::now();
    let pixels = render_no_bar_multithreaded(&Scene::cornell_box(), IMAGE_WIDTH, IMAGE_HEIGHT);
    println!(
        "Raytracing finished in {}",
        start.elapsed().to_human_time_string()
    );

    write_image(&pixels, Path::new(FILE_DEFAULT_PATH));
}

pub fn run_same_as_bench() {
    let scene = Scene::bench();

    let start = Instant::now();

    let pixels = render_no_bar(&scene, IMAGE_WIDTH, IMAGE_HEIGHT);

    println!(
        "Raytracing finished in {}",
        start.elapsed().to_human_time_string()
    );

    write_image(&pixels, Path::new(FILE_DEFAULT_PATH));
}

fn write_image(pixels: &[u8], path: &Path) {
    zone!();
    println!("Writing image...");

    let file = File::create(path).unwrap();
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels).unwrap();

    let path_str = path.to_str().unwrap();
    println!("Image written to {path_str}");
}

use gpu_raytracing::render;

fn main() {
    pollster::block_on(render());
}

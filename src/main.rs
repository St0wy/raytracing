use raytracing::run_big_scene;
use tracy_full::alloc::GlobalAllocator;

#[global_allocator]
static ALLOC: GlobalAllocator = GlobalAllocator::new();

fn main() {
    run_big_scene();
}

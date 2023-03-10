use tracy::alloc::GlobalAllocator;
use raytracing::run_big_scene;

#[global_allocator]
static ALLOC: GlobalAllocator = GlobalAllocator::new();

fn main() {
    run_big_scene();
}

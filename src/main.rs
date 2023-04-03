use raytracing::run;
use tracy_full::alloc::GlobalAllocator;

#[global_allocator]
static ALLOC: GlobalAllocator = GlobalAllocator::new();

fn main() {
    run();
}

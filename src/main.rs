use raytracing::run_same_as_bench;
use tracy_full::alloc::GlobalAllocator;

#[global_allocator]
static ALLOC: GlobalAllocator = GlobalAllocator::new();

fn main() {
    run_same_as_bench();
}

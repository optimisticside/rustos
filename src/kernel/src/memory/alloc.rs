use core::alloc;
use core::alloc::{GlobalAlloc, Layout};

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!(
        "Allocation error with size {} and layout {}",
        layout.size(),
        layout.align()
    )
}

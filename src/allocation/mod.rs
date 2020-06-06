use core::intrinsics;

mod allocator;
use allocator::Allocator;

// Setup our own global allocator for the kernel
#[global_allocator]
static KERNEL_ALLOCATOR: Allocator = Allocator;

// Register our own allocation error handler.
// Called when the kernel fails to allocate memory
#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
  dbg!("\nKernel out of memory error!");
  dbg!("Tried to allocate layout: size=", layout.size(), " align=", layout.align());
  intrinsics::abort()
}

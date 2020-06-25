use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

// FIXME: This is just temporary and probably completely broken.
const MB: usize = 0x100000;
const KERNEL_START_ADR: usize = 0x2000;
const KERNEL_END_ADR: usize = KERNEL_START_ADR + 4 * MB;

struct AllocatorImpl { current: *mut u8 }

// Very stupid allocator TM.
// Just forwards the pointer, never freeing memory.
// This is just something to get started.
impl AllocatorImpl {
  unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
    let res = self.current;
    self.current = self.current.add(layout.size());
    if (self.current as usize) >= KERNEL_END_ADR {
      return null_mut();
    }
    res
  }
}

static mut KERNEL_ALLOCATOR: AllocatorImpl = AllocatorImpl { current: KERNEL_START_ADR as *mut u8 };

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 { KERNEL_ALLOCATOR.alloc(layout) }
  unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

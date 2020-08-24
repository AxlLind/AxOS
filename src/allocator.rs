use crate::mem::page_table::{page_map_addr, translate_addr};
use crate::mem::VirtAddr;
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use lazy_static::lazy_static;
use spin::Mutex;

const MB: usize = 0x10_0000;
const HEAP_START_ADDR: usize = 0x4444_4400_0000;
const HEAP_END_ADDR: usize = HEAP_START_ADDR + 2 * MB;

struct KernelHeapAllocator {
  ptr: usize,
}

impl KernelHeapAllocator {
  fn new() -> Self {
    Self {
      ptr: HEAP_START_ADDR,
    }
  }

  // stupid bump allocator tm
  unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
    let size = layout.size();
    let align = layout.align();
    if self.ptr + layout.size() > HEAP_END_ADDR {
      return null_mut();
    }
    // align the ptr
    if self.ptr & (align - 1) != 0 {
      self.ptr = ((self.ptr + align - 1) / align) * align;
    }
    let res = self.ptr as *mut u8;
    self.ptr += size;
    res
  }

  unsafe fn free(&mut self, _layout: Layout) {}
}

lazy_static! {
  static ref KERNEL_ALLOCATOR: Mutex<KernelHeapAllocator> = Mutex::new(KernelHeapAllocator::new());
}

struct AllocatorWrapper;

unsafe impl GlobalAlloc for AllocatorWrapper {
  unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    KERNEL_ALLOCATOR.lock().alloc(layout)
  }
  unsafe fn dealloc(&self, _ptr: *mut u8, layout: Layout) {
    KERNEL_ALLOCATOR.lock().free(layout)
  }
}

#[global_allocator]
static ALLOCATOR: AllocatorWrapper = AllocatorWrapper;

// Called when the kernel fails to allocate memory
#[alloc_error_handler]
fn alloc_error(layout: core::alloc::Layout) -> ! {
  panic!("Kernel OOM, layout {:?}", layout);
}

pub fn initialize() {
  // map all the heap pages
  for page_addr in (HEAP_START_ADDR..HEAP_END_ADDR).step_by(0x1000) {
    page_map_addr(VirtAddr::new(page_addr as u64));
  }
  assert!(translate_addr(VirtAddr::new(HEAP_START_ADDR as u64)).is_some());
}

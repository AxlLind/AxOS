use super::{PhysAddr, VirtAddr};
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

pub struct FrameAllocator {
  memory_map:    &'static MemoryMap,
  current_index: usize,
}

lazy_static! {
  static ref FRAME_ALLOCATOR: Mutex<FrameAllocator> = {
    // Super-unsafe but FrameAllocator::initialize
    // should be called before ever touching this!
    let allocator = FrameAllocator {
      memory_map:    unsafe { &* VirtAddr::new(0).as_ptr() },
      current_index: 0,
    };
    Mutex::new(allocator)
  };
}

// FIXME: This is just a bump allocator to get us started
impl FrameAllocator {
  pub fn the() -> MutexGuard<'static, FrameAllocator> {
    FRAME_ALLOCATOR.lock()
  }

  pub fn initialize(memory_map: &'static MemoryMap) {
    Self::the().memory_map = memory_map;
  }

  pub fn alloc(&mut self) -> Option<PhysAddr> {
    let addr = self
      .memory_map
      .iter()
      .filter(|region| region.region_type == MemoryRegionType::Usable)
      .flat_map(|region| {
        let start = region.range.start_addr();
        let end = region.range.end_addr();
        (start..end).step_by(0x1000)
      })
      .nth(self.current_index)
      .map(PhysAddr::new);
    self.current_index += 1;
    addr
  }

  pub fn free(&mut self, frame: PhysAddr) {
    // TODO: Actually do something here
    assert!(frame.is_page_aligned());
  }
}

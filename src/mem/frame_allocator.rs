use super::PhysAddr;
use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

pub struct FrameAllocator {
  memory_map:    Option<&'static MemoryMap>,
  current_index: usize,
}

lazy_static! {
  static ref FRAME_ALLOCATOR: Mutex<FrameAllocator> = {
    let allocator = FrameAllocator {
      memory_map:    None,
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
    Self::the().memory_map = Some(memory_map);
  }

  pub fn alloc(&mut self) -> Option<PhysAddr> {
    let addr = self
      .memory_map
      .unwrap()
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

use core::mem::size_of;
use super::DescriptorTablePtr;

// This flag must be set for code segments.
const EXECUTABLE: u64 = 1 << 43;
// This flag must be set for user segments (in contrast to system segments).
const USER_SEGMENT: u64 = 1 << 44;
// Must be set for any segment, causes a segment not present exception if not set.
const PRESENT: u64 = 1 << 47;
// Must be set for long mode code segments.
const LONG_MODE: u64 = 1 << 53;

pub fn kernel_code_segment() -> u64 {
  PRESENT | EXECUTABLE | LONG_MODE | USER_SEGMENT
}

pub fn null_segment() -> u64 { 0 }

#[derive(Debug)]
pub struct GlobalDescriptorTable {
  entries: [u64; 8],
  size: u8,
}

impl GlobalDescriptorTable {
  pub fn new() -> Self {
    Self {
      entries: [0; 8],
      size: 0,
    }
  }

  pub fn push(&mut self, segment: u64) {
    self.entries[self.size as usize] = segment;
    self.size += 1;
  }

  pub fn load(&'static self) {
    let ptr = DescriptorTablePtr {
      size: self.size as u16 * size_of::<u64>() as u16 - 1,
      base_ptr: self.entries.as_ptr() as u64,
    };
    unsafe { asm!("lgdt [{}]", in(reg) &ptr); }
  }
}

pub fn current_cs() -> u16 {
  let segment;
  unsafe { asm!("mov {:x}, cs", out(reg) segment); }
  segment
}

// Note that you cannot just 'move cs, {}' to
// change the cs register. Instead we need to
// put the value and return address on the
// stack and do a 'far return'.
pub unsafe fn set_cs(segment_index: u16) {
  let cs = segment_index << 3;
  asm!("
    push {}
    lea  rax, 1f
    push rax
    retfq
    1:",
    in(reg) cs as u64,
  )
}

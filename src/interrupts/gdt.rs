use super::DescriptorTablePtr;
use core::mem::size_of;
use core::ops::{Index, IndexMut};

// Reference: https://wiki.osdev.org/TSS
#[repr(C, packed)]
pub struct TaskSegmentSelector {
  reserved1: u32,
  rsp: [u64; 3],
  reserved2: u64,
  ist: [u64; 7],
  reserved3: u64,
  reserved4: u16,
  io_base_ptr: u16,
}

impl TaskSegmentSelector {
  pub fn new() -> Self {
    Self {
      rsp: [0; 3],
      ist: [0; 7],
      io_base_ptr: 0,
      reserved1: 0,
      reserved2: 0,
      reserved3: 0,
      reserved4: 0,
    }
  }

  pub fn set_interrupt_stack(&mut self, i: usize, stack: &'static [u8]) {
    let stack_ptr = stack.as_ptr() as u64;
    let stack_size = stack.len() as u64;
    self.ist[i - 1] = stack_ptr + stack_size;
  }
}

#[repr(C)]
pub struct GlobalDescriptorTable([u64; 4]);

impl GlobalDescriptorTable {
  pub fn new() -> Self {
    Self([0; 4])
  }

  // Safe since the GDT is static
  pub fn load(&'static self) {
    let ptr = DescriptorTablePtr::ptr_to(self);
    unsafe { asm!("lgdt [{}]", in(reg) &ptr) };
  }
}

impl Index<usize> for GlobalDescriptorTable {
  type Output = u64;

  fn index(&self, i: usize) -> &Self::Output {
    &self.0[i]
  }
}

impl IndexMut<usize> for GlobalDescriptorTable {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.0[i]
  }
}

const EXECUTABLE: u64 = 1 << 43; // Must be set for code segments.
const USER_SEGMENT: u64 = 1 << 44; // Must be set for user segments
const PRESENT: u64 = 1 << 47; // Must be set for any segment
const LONG_MODE: u64 = 1 << 53; // Must be set for long mode code segments.

pub fn kernel_code_segment() -> u64 {
  PRESENT | EXECUTABLE | LONG_MODE | USER_SEGMENT
}

pub fn null_segment() -> u64 {
  0
}

// The layout of the TSS descriptor is a bit messy.
// See section 6.2.3 of https://www.intel.com/content/dam/support/us/en/documents/processors/pentium4/sb/25366821.pdf
pub fn tss_segment(tss: &'static TaskSegmentSelector) -> (u64, u64) {
  let tss_ptr = tss as *const _ as u64;
  let ptr_low = tss_ptr & 0xffffff; // 0:23
  let ptr_mid = (tss_ptr >> 24) & 0xff; // 24:31
  let ptr_high = tss_ptr >> 32; // 32:63
  let mut segment_low = size_of::<TaskSegmentSelector>() as u64 - 1;
  segment_low |= (ptr_low) << 16;
  segment_low |= 0b1001 << 40; // type 64-bit TSS (available)
  segment_low |= PRESENT;
  segment_low |= ptr_mid << 56;
  (segment_low, ptr_high)
}

pub fn current_cs() -> u16 {
  let segment;
  unsafe { asm!("mov {:x}, cs", out(reg) segment) };
  segment
}

// You cannot just 'move cs, {}' to change the cs
// register. We need to push the new cs value and
// a return address on the stack and do a 'far return'.
// Unsafe since the caller has to provide a valid index.
pub unsafe fn set_cs(segment_index: u64) {
  asm!("
    push {}
    lea rax, 1f
    push rax
    retfq
    1:",
    in(reg) segment_index,
  )
}

pub unsafe fn load_tss(segment_index: u16) {
  asm!("ltr {:x}", in(reg) segment_index);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test_case]
  fn size_check() {
    use core::mem::size_of;
    assert_eq!(size_of::<TaskSegmentSelector>(), 104);
    assert_eq!(size_of::<GlobalDescriptorTable>(), 8 * 4);
  }
}

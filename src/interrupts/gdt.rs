use core::mem::size_of;
use super::DescriptorTablePtr;

// Reference: https://wiki.osdev.org/TSS
#[derive(Debug,Clone,Copy,Default)]
#[repr(C,packed)]
pub struct TaskSegmentSelector {
  reserved1: u32,
  rsp: [(u32,u32); 3],
  reserved2: u64,
  ist: [(u32,u32); 7],
  reserved3: u64,
  iopb_offset: u32,
}

impl TaskSegmentSelector {
  pub fn new() -> Self {
    assert_eq!(size_of::<Self>(), 104);
    Self::default()
  }

  pub fn set_interrupt_stack(&mut self, i: usize, stack_ptr: u64) {
    let ptr_low = stack_ptr as u32;
    let ptr_high = (stack_ptr >> 32) as u32;
    self.ist[i-1] = (ptr_low, ptr_high);
  }

  pub fn _print(&self) {
    dbg!("{:#x?}", self);
  }
}

#[derive(Debug)]
#[repr(C)]
pub struct GlobalDescriptorTable {
  entries: [u64; 8],
  size: usize,
}

impl GlobalDescriptorTable {
  pub fn new() -> Self {
    Self {
      entries: [0; 8],
      size: 0,
    }
  }

  pub fn push(&mut self, segment: u64) {
    self.entries[self.size] = segment;
    self.size += 1;
  }

  // Safe since the GDT is static
  pub fn load(&'static self) {
    let ptr = DescriptorTablePtr {
      size: self.size as u16 * size_of::<u64>() as u16 - 1,
      base_ptr: self.entries.as_ptr() as u64,
    };
    unsafe { asm!("lgdt [{}]", in(reg) &ptr); }
  }
}

const EXECUTABLE:   u64 = 1 << 43; // Must be set for code segments.
const USER_SEGMENT: u64 = 1 << 44; // Must be set for user segments
const PRESENT:      u64 = 1 << 47; // Must be set for any segment
const LONG_MODE:    u64 = 1 << 53; // Must be set for long mode code segments.

pub fn kernel_code_segment() -> u64 {
  PRESENT | EXECUTABLE | LONG_MODE | USER_SEGMENT
}

pub fn null_segment() -> u64 { 0 }

// The layout of the TSS descriptor is messy. See section 6.2.3 of
// https://www.intel.com/content/dam/support/us/en/documents/processors/pentium4/sb/25366821.pdf
pub fn tss_segment(tss: &TaskSegmentSelector) -> (u64,u64) {
  let tss_ptr = tss as *const _ as u64;
  let ptr_low = tss_ptr & 0xffffff;     // 0:23
  let ptr_mid = (tss_ptr >> 24) & 0xff; // 24:31
  let ptr_high = tss_ptr >> 32;         // 32:63
  let mut segment_low = size_of::<TaskSegmentSelector>() as u64;
  segment_low |= (ptr_low) << 16;
  segment_low |= 0x89 << 40; // type
  segment_low |= PRESENT;
  segment_low |= ptr_mid << 56;
  (segment_low, ptr_high)
}

pub fn current_cs() -> u16 {
  let segment;
  unsafe { asm!("mov {:x}, cs", out(reg) segment); }
  segment
}

// You cannot just 'move cs, {}' to change the cs
// register. We need to push the new cs value and
// a return address on the stack and do a 'far return'.
// Unsafe since the caller has to provide a valid index.
pub unsafe fn set_cs(segment_index: u16) {
  let cs = segment_index << 3;
  asm!("
    push {}
    lea rax, 1f
    push rax
    retfq
    1:",
    in(reg) cs as u64,
  )
}

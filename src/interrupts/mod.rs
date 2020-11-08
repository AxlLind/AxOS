use crate::hang;
use crate::io;
use crate::keyboard;
use core::mem::size_of;
use lazy_static::lazy_static;

pub mod gdt;
pub mod idt;
pub mod pic;

use gdt::{GlobalDescriptorTable, TaskSegmentSelector};
use idt::InterruptDescriptorTable;

// Used to load the IDT and GDT tables
#[repr(packed)]
struct DescriptorTablePtr(u16, u64);

impl DescriptorTablePtr {
  fn ptr_to<T>(t: &T) -> Self {
    let size = size_of::<T>() as u16 - 1;
    let ptr = t as *const _ as u64;
    Self(size, ptr)
  }
}

// Pushed on the stack by the CPU before calling the interrupt handler
// For some interrupts an error code is also pushed in the stack.
// References:
// https://os.phil-opp.com/cpu-exceptions/#the-interrupt-stack-frame
// https://wiki.osdev.org/Exceptions
#[derive(Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
  instruction_ptr: u64,
  code_segment: u64,
  cpu_flags: u64,
  stack_ptr: u64,
  stack_segment: u64,
}

extern "x86-interrupt" fn breakpoint_handler(frame: &mut InterruptStackFrame) {
  dbg!("breakpoint interrupt!");
  dbg!("{:x?}", frame);
  hang();
}

extern "x86-interrupt" fn timer_handler(_: &mut InterruptStackFrame) {
  unsafe { pic::end_of_interrupt(0) };
}

extern "x86-interrupt" fn keyboard_handler(_: &mut InterruptStackFrame) {
  let scan_code = io::read(0x60);
  keyboard::handle_keyboard_event(scan_code);
  unsafe { pic::end_of_interrupt(1) };
}

extern "x86-interrupt" fn double_fault_handler(
  frame: &mut InterruptStackFrame,
  _err_code: u64,
) -> ! {
  dbg!("double fault interrupt!");
  dbg!("{:x?}", frame);
  hang();
}

extern "x86-interrupt" fn page_fault_handler(frame: &mut InterruptStackFrame, err_code: u64) -> ! {
  dbg!("page fault interrupt!");
  dbg!("{:x?}", frame);
  dbg!("errcode {:x}", err_code);
  hang();
}

extern "x86-interrupt" fn general_protection_fault_handler(
  frame: &mut InterruptStackFrame,
  err_code: u64,
) -> ! {
  dbg!("general protection fault interrupt!");
  dbg!("{:x?}", frame);
  dbg!("errcode {:x}", err_code);
  hang();
}

lazy_static! {
  static ref TSS: TaskSegmentSelector = {
    let mut tss = TaskSegmentSelector::new();
    // TODO: Allocate this memory instead
    static mut INTERRUPT_STACK: [u8; 4096] = [0; 4096];
    tss.set_interrupt_stack(1, unsafe { &INTERRUPT_STACK });
    tss
  };

  static ref GDT: GlobalDescriptorTable = {
    let mut gdt = GlobalDescriptorTable::new();
    let tss_segment = gdt::tss_segment(&TSS);
    gdt[0] = gdt::null_segment();
    gdt[1] = gdt::kernel_code_segment();
    gdt[2] = tss_segment.0;
    gdt[3] = tss_segment.1;
    gdt
  };

  static ref IDT: InterruptDescriptorTable = {
    let mut idt = InterruptDescriptorTable::new();
    idt[3].set_handler(breakpoint_handler as usize);
    idt[8].set_handler(double_fault_handler as usize).with_ist(1);
    idt[13].set_handler(general_protection_fault_handler as usize);
    idt[14].set_handler(page_fault_handler as usize);
    idt[32].set_handler(timer_handler as usize);
    idt[33].set_handler(keyboard_handler as usize);
    idt
  };
}

pub fn initialize() {
  GDT.load();
  unsafe { gdt::set_cs(8) };
  unsafe { gdt::load_tss(16) };
  IDT.load();
  pic::initialize();
  unsafe { asm!("sti") };
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test_case]
  fn size_check() {
    use core::mem::size_of;
    assert_eq!(size_of::<DescriptorTablePtr>(), 10);
    assert_eq!(size_of::<InterruptStackFrame>(), 40);
  }
}

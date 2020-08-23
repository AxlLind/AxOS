#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(custom_test_frameworks)]
#![test_runner(ax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

// This test verifies that we can successfully catch a kernel
// stack overflow and execute the double fault handler, using
// the TaskSegmentSelector and interrupt stack table mechanism.

use ax_os::dbg;
use ax_os::interrupts::{gdt, idt, InterruptStackFrame};
use gdt::{GlobalDescriptorTable, TaskSegmentSelector};
use idt::InterruptDescriptorTable;
use lazy_static::lazy_static;

extern "x86-interrupt" fn double_fault(_: &mut InterruptStackFrame, _: u64) {
  dbg!("[success]");
  ax_os::qemu_exit_success();
}

lazy_static! {
  static ref TSS: TaskSegmentSelector = {
    let mut tss = TaskSegmentSelector::new();
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
    idt[0x08].set_handler(double_fault as usize).with_ist(1);
    idt
  };
}

fn initialize(_: &'static bootloader::BootInfo) {
  GDT.load();
  unsafe { gdt::set_cs(8) };
  unsafe { gdt::load_tss(16) };
  IDT.load();
}

ax_os::test_prelude!(initialize);

#[test_case]
fn kernel() {
  #[allow(unconditional_recursion)]
  fn stack_overflow() {
    stack_overflow();
  }

  stack_overflow();
}

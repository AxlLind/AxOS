#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(ax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(dead_code))]
#![allow(clippy::identity_op)]
#![allow(unused_unsafe)]

extern crate alloc;

use ax_os::{hang, indexable_from_field};
use bootloader::BootInfo;
use core::panic::PanicInfo;

#[macro_use]
mod dbg_print;
mod allocation;
mod interrupts;
mod io;
mod mem;
mod serial_port;
mod vga_device;
use mem::frame_allocator::FrameAllocator;
use vga_device::{VgaColor, VgaDevice};

fn initialize() {
  dbg_print::initialize();
  interrupts::initialize();
}

#[cfg(test)]
ax_os::test_prelude!(initialize);

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
  dbg!("Kernel panicked!");
  dbg!("Error: {}", info);
  ax_os::hang();
}

#[cfg(not(test))]
#[no_mangle]
pub fn _start(info: &'static BootInfo) -> ! {
  initialize();
  FrameAllocator::initialize(&info.memory_map);
  let frame = FrameAllocator::the().alloc();
  dbg!("{:x?}", frame);
  let mut vga = VgaDevice::new();
  for (i, &c) in b"Hello world".iter().enumerate() {
    vga.write_char(i, i, c, VgaColor::Green, VgaColor::Black);
  }
  dbg!("VGA: 0xb8001 -> {:x}", unsafe {
    mem::page_table::translate_addr(mem::VirtAddr::new(0xb8001))
      .unwrap()
      .as_u64()
  });
  ax_os::hlt_loop();
}

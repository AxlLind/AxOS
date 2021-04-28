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

#[macro_use]
extern crate alloc;

use ax_os::{hang, indexable_from_field};
use bootloader::BootInfo;
use core::panic::PanicInfo;

#[macro_use]
mod dbg_print;
mod allocator;
mod interrupts;
mod io;
mod keyboard;
mod mem;
mod serial_port;
mod vga;

use mem::frame_allocator::FrameAllocator;
use vga::VgaDevice;

fn initialize(info: &'static BootInfo) {
  dbg_print::initialize();
  FrameAllocator::initialize(&info.memory_map);
  allocator::initialize();
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
  initialize(&info);
  let v = vec![1; 100];
  dbg!("{:x?} -> {}", (&v[42]) as *const _, v[42]);
  let mut vga = VgaDevice::new();
  for (i, &c) in b"Hello world".iter().enumerate() {
    vga.set_color(unsafe { core::mem::transmute(i as u8 + 1) });
    vga.write_char(i, i, c);
  }
  ax_os::hlt_loop();
}

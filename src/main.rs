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

extern crate alloc;

use ax_os::{hang, indexable_from_field};
use core::panic::PanicInfo;

#[macro_use]
mod dbg_print;
mod allocation;
mod interrupts;
mod io;
mod mem;
mod serial_port;
mod vga_device;
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
pub fn _start(_: bootloader::BootInfo) -> ! {
  initialize();
  let mut vga = VgaDevice::new();
  for (i, &c) in b"Hello world".iter().enumerate() {
    vga.write_char(i, i, c, VgaColor::Green, VgaColor::Black);
  }
  ax_os::hlt_loop();
}

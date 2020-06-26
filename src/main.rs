#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(ax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

#[macro_use]
mod dbg_print;
mod allocation;
mod interrupts;
mod io;
mod serial_port;
mod vga_device;
use vga_device::VgaColor;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  dbg!("Kernel panicked!");
  dbg!("Error: {}", info);
  ax_os::hang();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  ax_os::test_panic_handler(info);
}

#[no_mangle]
pub fn _start() -> ! {
  dbg_print::initialize();
  interrupts::initialize();

  #[cfg(test)]
  test_main();

  let mut vga_device = vga_device::VgaDevice::new();
  for (i, &c) in b"Hello world".iter().enumerate() {
    vga_device.write_char(i, i, c, VgaColor::Green, VgaColor::Black);
  }
  ax_os::hlt_loop();
}

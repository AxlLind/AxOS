#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]

#![test_runner(ax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

#[macro_use]
mod dbg_print;
mod allocation;
mod io;
mod serial_port;
mod vga_device;
mod interrupts;
use vga_device::VgaColor;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
  dbg!("Kernel panicked!");
  if let Some(msg) = info.payload().downcast_ref::<&str>() {
    dbg!("Panic payload: {}", msg);
  }
  if let Some(l) = info.location() {
    dbg!("Panic in {} {}:{}", l.file(), l.line(), l.column());
  }
  loop {}
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
  for (i,&c) in b"Hello world".iter().enumerate() {
    vga_device.write_char(i, i, c, VgaColor::Green, VgaColor::Black);
  }
  loop {
    unsafe { asm!("hlt"); }
  }
}

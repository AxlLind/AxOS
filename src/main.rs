#![no_std]
#![no_main]

#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(alloc_error_handler)]

#[macro_use] mod dbg_print;

// import the alloc crate (we are no_std + alloc)
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate lazy_static;

use core::panic::PanicInfo;

mod allocation;
mod io_port;
mod serial_port;
mod vga_device;
mod interrupts;
use vga_device::VgaColor;

#[panic_handler]
#[no_mangle]
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

#[no_mangle]
pub fn _start() -> ! {
  dbg_print::initialize();
  interrupts::initialize();

  let mut vga_device = vga_device::VgaDevice::new();
  for (i,&c) in b"Hello world".iter().enumerate() {
    vga_device.write_char(i, i, c, VgaColor::Green, VgaColor::Black);
  }
  loop {
    unsafe { asm!("hlt"); }
  }
}

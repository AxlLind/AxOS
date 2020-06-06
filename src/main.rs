#![no_std]
#![no_main]

#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(alloc_error_handler)]

#[macro_use] mod dbg_print;

// import the alloc crate (we are no_std + alloc)
#[macro_use]
extern crate alloc;

use core::intrinsics;
use core::panic::PanicInfo;

mod allocation;
mod serial_port;
mod vga_device;

#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
  dbg!("\nKernel panicked!");
  if let Some(msg) = info.payload().downcast_ref::<&str>() {
    dbg!("Panic payload: {}", msg);
  }
  if let Some(l) = info.location() {
    dbg!("Panic in {} {}:{}", l.file(), l.line(), l.column());
  }
  intrinsics::abort()
}

fn vga_test() {
  let mut vga_device = vga_device::VgaDevice::new();
  let s = "hello world";
  for _ in 0..1000 {
    for b in s.bytes() {
      vga_device.write_char(b);
    }
    for _ in 0..10000 {}
  }
}

#[no_mangle]
pub fn _start() -> ! {
  dbg_print::initialize();
  vga_test();

  dbg!("Hello {}", "world");
  dbg!("Handles numbers: {} {}", 11, -1337);
  dbg!("with edge cases: {} {} {}", 0, u64::MAX, i64::MIN);
  dbg!("And characters: {}{}{}{}", 'A', 'x', 'O', 'S');
  let v = vec![0;1];
  dbg!("{}", v[0]);

  loop {}
}

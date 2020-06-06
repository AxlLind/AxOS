#![no_std]
#![no_main]

#![feature(core_intrinsics)]
#![feature(asm)]
#![feature(alloc_error_handler)]

#[macro_use] mod dbg_print;

// import the alloc crate (we are no_std + alloc)
#[macro_use]
extern crate alloc;
use alloc::vec::Vec;

use core::intrinsics;
use core::panic::PanicInfo;

mod allocation;
mod serial_port;

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

#[no_mangle]
pub fn _start() -> ! {
  dbg_print::initialize_debug_port();
  let vga_mem = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

  let s = b"hello world";
  for i in 0..s.len() {
    vga_mem[2*i] = s[i];
    vga_mem[2*i + 1] = 0x02;
  }

  dbg!("Hello {}", "world");
  dbg!("Handles numbers: {} {}", 11, -1337);
  dbg!("with edge cases: {} {} {}", 0, u64::MAX, i64::MIN);
  dbg!("And characters: {}{}{}{}", 'A', 'x', 'O', 'S');

  loop {}
}

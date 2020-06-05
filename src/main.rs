#![feature(core_intrinsics)]
#![feature(asm)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

mod serial_port;

#[macro_use]
mod dbg_print;
use dbg_print::*;

#[panic_handler]
#[no_mangle]
fn panic(info: &PanicInfo) -> ! {
  dbg!("\nKernel panicked!");
  if let Some(msg) = info.payload().downcast_ref::<&str>() {
    dbg!("Panic payload: ", msg);
  }
  if let Some(l) = info.location() {
    dbg!("Panic in ", l.file(), ' ', l.line(), ':', l.column());
  }
  intrinsics::abort()
}

#[no_mangle]
pub fn _start() -> ! {
  dbg_print::init_debug_port();
  let vga_mem = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

  let s = b"hello world";
  for i in 0..s.len() {
    vga_mem[2*i] = s[i];
    vga_mem[2*i + 1] = 0x02;
  }

  dbg!("Hello ", "world");
  dbg!("Can now print number! ", 11, ' ', -1337, ' ', i32::MIN);
  dbg!("And characters: ", 'A', 'x', 'O', 'S');

  loop {}
}

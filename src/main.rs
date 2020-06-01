#![feature(core_intrinsics)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
  intrinsics::abort()
}

#[no_mangle]
pub fn _start() -> ! {
  let vga_mem = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

  let s = b"hello world";
  for i in 0..s.len() {
    vga_mem[2*i] = s[i];
    vga_mem[2*i + 1] = 0x02;
  }

  loop {}
}

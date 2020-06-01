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
  loop {}
}

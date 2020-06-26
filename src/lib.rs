#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

#[macro_use]
pub mod dbg_print;
mod io;
mod serial_port;

pub fn hlt_loop() -> ! {
  loop {
    unsafe {
      asm!("hlt");
    }
  }
}

pub fn hang() -> ! {
  unsafe {
    asm!("cli; hlt");
  }
  unreachable!();
}

pub fn qemu_exit_success() -> ! {
  io::send(0xf4, 0x10);
  hang();
}

pub fn qemu_exit_failure() -> ! {
  io::send(0xf4, 0x11);
  hang();
}

pub trait TestCase {
  fn run(&self);
}

impl<T> TestCase for T
where
  T: Fn(),
{
  fn run(&self) {
    dbg_no_ln!("{}\t", core::any::type_name::<T>());
    self();
    dbg!("[success]");
  }
}

pub fn test_runner(tests: &[&dyn TestCase]) -> ! {
  match tests.len() {
    0 => {}
    1 => dbg!("Running 1 test"),
    i => dbg!("Running {} tests", i),
  }
  for test in tests {
    test.run();
  }
  qemu_exit_success();
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
  dbg!("[failed]\n");
  dbg!("Error: {}\n", info);
  qemu_exit_failure();
}

#[macro_export]
macro_rules! test_prelude {
  () => {
    #[cfg(test)]
    #[no_mangle]
    pub extern "C" fn _start() -> ! {
      test_main();
      $crate::hlt_loop();
    }

    #[cfg(test)]
    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
      $crate::test_panic_handler(info);
    }
  };
}

test_prelude!();

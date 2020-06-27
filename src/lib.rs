#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[macro_use]
pub mod dbg_print;
mod io;
mod serial_port;

pub fn hlt_loop() -> ! {
  loop {
    unsafe { asm!("hlt") };
  }
}

pub fn hang() -> ! {
  unsafe { asm!("cli; hlt") };
  unreachable!();
}

fn qemu_exit_success() -> ! {
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

impl<T: Fn()> TestCase for T {
  fn run(&self) {
    dbg_no_ln!("{}\t", core::any::type_name::<T>());
    self();
    dbg!("[success]");
  }
}

pub fn test_runner(tests: &[&dyn TestCase]) -> ! {
  for test in tests {
    test.run();
  }
  qemu_exit_success();
}

#[macro_export]
macro_rules! test_prelude {
  ($init:expr) => {
    #[no_mangle]
    pub extern "C" fn _start() -> ! {
      $init();
      test_main();
      $crate::hlt_loop();
    }

    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! {
      $crate::dbg!("[failed]");
      $crate::dbg!("Error: {}", info);
      $crate::qemu_exit_failure();
    }
  };
  () => {
    $crate::test_prelude!(|| {});
  };
}

#[cfg(test)]
test_prelude!();

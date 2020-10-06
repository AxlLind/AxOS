#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(asm)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::identity_op)]
#![allow(clippy::missing_safety_doc)] // library lints
#![allow(clippy::new_without_default)]

#[macro_use]
pub mod dbg_print;
pub mod interrupts;
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

pub fn qemu_exit_success() -> ! {
  io::send(0xf4, 0x10);
  unreachable!();
}

pub fn qemu_exit_failure() -> ! {
  io::send(0xf4, 0x11);
  unreachable!();
}

pub trait TestCase {
  fn name(&self) -> (&'static str, &'static str) {
    core::any::type_name::<Self>()
      .split("::")
      .filter(|&module| module != "tests")
      .fold(("", ""), |(_, module), name| (module, name))
  }
  fn run(&self);
}

impl<T: Fn()> TestCase for T {
  fn run(&self) {
    self();
  }
}

pub fn test_runner(tests: &[&dyn TestCase]) -> ! {
  unsafe { asm!("cli") }; // no external interrupts during testing
  for test in tests {
    let (module, name) = test.name();
    let padding = 40 - module.len() - name.len();
    dbg_no_ln!("{}::{}", module, name);
    dbg_no_ln!("{:<1$}", "", padding);
    test.run();
    dbg!("[success]");
  }
  qemu_exit_success();
}

#[macro_export]
macro_rules! test_prelude {
  ($($init_fn:expr)?) => {
    #[panic_handler]
    fn panic_handler(info: &core::panic::PanicInfo) -> ! {
      $crate::dbg!("[failed]");
      $crate::dbg!("Error: {}", info);
      $crate::qemu_exit_failure();
    }

    #[allow(unreachable_code)]
    #[no_mangle]
    pub extern "C" fn _start(info: &'static bootloader::BootInfo) -> ! {
      $($init_fn(&info);)?
      test_main();
      unreachable!();
    }
  };
}

#[macro_export]
macro_rules! indexable_from_field {
  ($T:ty, $field:tt, $Out:ty) => {
    impl core::ops::Index<usize> for $T {
      type Output = $Out;
      fn index(&self, i: usize) -> &Self::Output {
        &self.$field[i]
      }
    }
    impl core::ops::IndexMut<usize> for $T {
      fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.$field[i]
      }
    }
  };
}

// do not run any tests from this file
#[cfg(test)]
fn exit_immediately(_: &'static bootloader::BootInfo) {
  qemu_exit_success();
}

#[cfg(test)]
test_prelude!(exit_immediately);

use crate::serial_port;
use core::fmt;

// QEMU accepts debug output on the COM1 serial port
// Reference: https://wiki.osdev.org/Serial_Ports
const COM1: u16 = 0x3f8;

pub fn initialize() {
  serial_port::initialize(COM1);
}

pub struct DebugPrinter;

impl fmt::Write for DebugPrinter {
  fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
    for b in s.bytes() {
      serial_port::send(COM1, b);
    }
    Ok(())
  }
}

#[doc(hidden)]
pub fn __print(args: fmt::Arguments) {
  use fmt::Write;
  DebugPrinter.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! dbg_no_ln {
  ($($e:tt)+) => { $crate::dbg_print::__print(format_args!($($e)+)) };
}

#[macro_export]
macro_rules! dbg {
  () => { $crate::dbg_no_ln!("\n") };
  ($($e:tt)+) => { $crate::dbg_no_ln!("{}\n", format_args!($($e)+)) };
}

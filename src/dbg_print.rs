use core::fmt;
use crate::serial_port;

// QEMU accepts debug output on the COM1 serial port
// Reference: https://wiki.osdev.org/Serial_Ports
const COM1: u16 = 0x3f8;

pub fn initialize() { serial_port::initialize(COM1); }

pub struct DebugPrinter;

impl fmt::Write for DebugPrinter {
  fn write_str(&mut self, s: &str) -> Result<(),fmt::Error> {
    for b in s.bytes() { serial_port::send(COM1, b); }
    Ok(())
  }
}

#[allow(unused_macros)]
macro_rules! dbg_no_ln {
  ($($e:expr),+ $(,)?) => {{
    use core::fmt::Write;
    write!($crate::dbg_print::DebugPrinter, $($e),+).unwrap();
  }}
}

macro_rules! dbg {
  ($($e:expr),+ $(,)?) => {{
    use core::fmt::Write;
    writeln!($crate::dbg_print::DebugPrinter, $($e),+).unwrap();
  }}
}

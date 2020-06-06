use core::fmt;
use crate::serial_port;

// QEMU accepts debug output on the COM1 serial port
// Reference: https://wiki.osdev.org/Serial_Ports
const COM1: u16 = 0x3f8;

pub fn initialize_debug_port() {
  serial_port::initialize(COM1);
}

fn serial_print_byte(b: u8) {
  serial_port::send(COM1, b);
}

pub struct DebugPrinter;

impl fmt::Write for DebugPrinter {
  fn write_str(&mut self, s: &str) -> Result<(),fmt::Error> {
    s.bytes().for_each(serial_print_byte);
    Ok(())
  }
}

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

use crate::serial_port::SerialPort;

// QEMU accepts debug output on the COM1 serial port
// Reference: https://wiki.osdev.org/Serial_Ports
const COM1: u16 = 0x3f8;
static DEBUG_SERIAL_PORT: SerialPort = SerialPort(COM1);

pub fn init_debug_port() {
  SerialPort::initialize(COM1);
}

fn serial_print_byte(b: u8) {
  DEBUG_SERIAL_PORT.send(b);
}

pub trait DebugPrintable {
  fn print_debug(&self);
}

impl DebugPrintable for str {
  fn print_debug(&self) {
    self.bytes().for_each(serial_print_byte);
  }
}

impl DebugPrintable for char {
  fn print_debug(&self) {
    serial_print_byte(*self as u8);
  }
}

impl DebugPrintable for u64 {
  fn print_debug(&self) {
    if *self == 0 { return serial_print_byte(b'0'); }
    // u64 is at most 20 chars
    let mut buffer = [b'0'; 20];
    let (mut n, mut len) = (*self, 0);
    while n != 0 {
      buffer[len] += (n % 10) as u8;
      n /= 10;
      len += 1;
    }
    (0..len).rev()
      .map(|i| buffer[i])
      .for_each(serial_print_byte);
  }
}

impl DebugPrintable for i64 {
  fn print_debug(&self) {
    let mut n = *self;
    if n < 0 {
      serial_print_byte(b'-');
      if n != i64::MIN { n = -n; }
    }
    (n as u64).print_debug();
  }
}

macro_rules! impl_printable_from {
  ($A:ty, [$($T:ty),+]) => {
    $(impl DebugPrintable for $T {
      fn print_debug(&self) { (*self as $A).print_debug(); }
    })+
  };
}
impl_printable_from!{ u64, [u32, u16, u8, usize] }
impl_printable_from!{ i64, [i32, i16, i8, isize] }

macro_rules! dbg_no_ln {
  ($($e:expr),+ $(,)?) => {{
    $($e.print_debug();)+
  }}
}

macro_rules! dbg {
  ($($e:expr),+ $(,)?) => { dbg_no_ln!($($e),+,'\n') }
}

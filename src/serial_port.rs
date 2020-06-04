#![allow(unused)]

/*
  This is a low level abstraction of a serial port.
  Initializes the port on creation. It does not
  check that this port has already been initialized,
  that is up to the creator of the SerialPort struct!

  References:
  https://wiki.osdev.org/Serial_Ports
  https://c9x.me/x86/html/file_module_x86_id_222.html
  https://c9x.me/x86/html/file_module_x86_id_139.html
*/

pub trait SerialPortType {
  fn serial_send(port: u16, v: Self);
  fn serial_read(port: u16) -> Self;
}

impl SerialPortType for u8 {
  fn serial_send(port: u16, v: Self) {
    unsafe { asm!("out dx,al", in("dx") port, in("al") v); }
  }

  fn serial_read(port: u16) -> Self {
    let i;
    unsafe { asm!("in al,dx", out("al") i, in("dx") port); }
    i
  }
}

impl SerialPortType for u16 {
  fn serial_send(port: u16, v: Self) {
    unsafe { asm!("out dx,ax", in("dx") port, in("ax") v); }
  }

  fn serial_read(port: u16) -> Self {
    let i;
    unsafe { asm!("in ax,dx", out("ax") i, in("dx") port); }
    i
  }
}

impl SerialPortType for u32 {
  fn serial_send(port: u16, v: Self) {
    unsafe { asm!("out dx,eax", in("dx") port, in("eax") v); }
  }

  fn serial_read(port: u16) -> Self {
    let i;
    unsafe { asm!("in eax,dx", out("eax") i, in("dx") port); }
    i
  }
}

pub struct SerialPort(u16);

impl SerialPort {
  pub fn initialize(port: u16) -> Self {
    u8::serial_send(port + 1, 0x00); // Disable all interrupts
    u8::serial_send(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
    u8::serial_send(port + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
    u8::serial_send(port + 1, 0x00); //                  (hi byte)
    u8::serial_send(port + 3, 0x03); // 8 bits, no parity, one stop bit
    u8::serial_send(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
    u8::serial_send(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
    Self(port)
  }

  pub fn read<T: SerialPortType>(&self) -> T {
    while u8::serial_read(self.0 + 5) & 1 == 0 {}
    T::serial_read(self.0)
  }

  pub fn send<T: SerialPortType>(&self, t: T) {
    while u8::serial_read(self.0 + 5) & 0x20 == 0 {}
    T::serial_send(self.0, t);
  }
}
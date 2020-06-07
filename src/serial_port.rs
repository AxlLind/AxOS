#![allow(unused)]
use crate::io_port::IoPortType;

/*
  Low level helpers for reading/writing
  to the CPU's serial ports. References:
  https://wiki.osdev.org/Serial_Ports
  https://c9x.me/x86/html/file_module_x86_id_222.html
  https://c9x.me/x86/html/file_module_x86_id_139.html
*/

pub fn initialize(port: u16) {
  u8::io_send(port + 1, 0x00); // Disable all interrupts
  u8::io_send(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
  u8::io_send(port + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
  u8::io_send(port + 1, 0x00); //                  (hi byte)
  u8::io_send(port + 3, 0x03); // 8 bits, no parity, one stop bit
  u8::io_send(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
  u8::io_send(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
}

pub fn read<T: IoPortType>(port: u16) -> T {
  while u8::io_read(port + 5) & 1 == 0 {}
  T::io_read(port)
}

pub fn send<T: IoPortType>(port: u16, t: T) {
  while u8::io_read(port + 5) & 0x20 == 0 {}
  T::io_send(port, t);
}

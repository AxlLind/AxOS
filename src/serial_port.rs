#![allow(unused)]
use crate::io;

/*
  Low level helpers for reading/writing
  to the CPU's serial ports. References:
  https://wiki.osdev.org/Serial_Ports
  https://c9x.me/x86/html/file_module_x86_id_222.html
  https://c9x.me/x86/html/file_module_x86_id_139.html
*/

pub fn initialize(port: u16) {
  io::send(port + 1, 0x00); // Disable all interrupts
  io::send(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
  io::send(port + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
  io::send(port + 1, 0x00); //                  (hi byte)
  io::send(port + 3, 0x03); // 8 bits, no parity, one stop bit
  io::send(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
  io::send(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
}

pub fn read(port: u16) -> u8 {
  while io::read(port + 5) & 1 == 0 {}
  io::read(port)
}

pub fn send(port: u16, v: u8) {
  while io::read(port + 5) & 0x20 == 0 {}
  io::send(port, v);
}

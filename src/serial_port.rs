#![allow(unused)]
use crate::io_port::{io_send, io_read};

/*
  Low level helpers for reading/writing
  to the CPU's serial ports. References:
  https://wiki.osdev.org/Serial_Ports
  https://c9x.me/x86/html/file_module_x86_id_222.html
  https://c9x.me/x86/html/file_module_x86_id_139.html
*/

pub fn initialize(port: u16) {
  io_send(port + 1, 0x00); // Disable all interrupts
  io_send(port + 3, 0x80); // Enable DLAB (set baud rate divisor)
  io_send(port + 0, 0x03); // Set divisor to 3 (lo byte) 38400 baud
  io_send(port + 1, 0x00); //                  (hi byte)
  io_send(port + 3, 0x03); // 8 bits, no parity, one stop bit
  io_send(port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
  io_send(port + 4, 0x0B); // IRQs enabled, RTS/DSR set
}

pub fn read(port: u16) -> u8 {
  while io_read(port + 5) & 1 == 0 {}
  io_read(port)
}

pub fn send(port: u16, v: u8) {
  while io_read(port + 5) & 0x20 == 0 {}
  io_send(port, v);
}

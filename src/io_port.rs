#![allow(unused)]

/*
  Low level helpers for reading/writing
  to the CPU's io ports. References:
  https://wiki.osdev.org/Port_IO
  https://wiki.osdev.org/I/O_Ports
  https://c9x.me/x86/html/file_module_x86_id_222.html
  https://c9x.me/x86/html/file_module_x86_id_139.html
*/

pub trait IoPortType {
  fn io_send(port: u16, v: Self);
  fn io_read(port: u16) -> Self;
}

impl IoPortType for u8 {
  fn io_send(port: u16, v: Self) {
    unsafe { asm!("out dx,al", in("dx") port, in("al") v); }
  }

  fn io_read(port: u16) -> Self {
    let i;
    unsafe { asm!("in al,dx", out("al") i, in("dx") port); }
    i
  }
}

impl IoPortType for u16 {
  fn io_send(port: u16, v: Self) {
    unsafe { asm!("out dx,ax", in("dx") port, in("ax") v); }
  }

  fn io_read(port: u16) -> Self {
    let i;
    unsafe { asm!("in ax,dx", out("ax") i, in("dx") port); }
    i
  }
}

impl IoPortType for u32 {
  fn io_send(port: u16, v: Self) {
    unsafe { asm!("out dx,eax", in("dx") port, in("eax") v); }
  }

  fn io_read(port: u16) -> Self {
    let i;
    unsafe { asm!("in eax,dx", out("eax") i, in("dx") port); }
    i
  }
}

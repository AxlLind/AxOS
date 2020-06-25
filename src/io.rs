#![allow(unused)]

/*
  Low level assembly wrappers for reading
  and writing to the CPU's io ports.
  References:
  https://wiki.osdev.org/Port_IO
  https://wiki.osdev.org/I/O_Ports
  https://c9x.me/x86/html/file_module_x86_id_222.html
  https://c9x.me/x86/html/file_module_x86_id_139.html
*/

#[inline(always)]
pub fn send(port: u16, v: u8) {
  unsafe { asm!("out dx,al", in("dx") port, in("al") v); }
}

#[inline(always)]
pub fn read(port: u16) -> u8 {
  let i;
  unsafe { asm!("in al,dx", out("al") i, in("dx") port); }
  i
}

#[inline(always)]
pub fn send_u16(port: u16, v: u16) {
  unsafe { asm!("out dx,ax", in("dx") port, in("ax") v); }
}

#[inline(always)]
pub fn read_u16(port: u16) -> u16 {
  let i;
  unsafe { asm!("in ax,dx", out("ax") i, in("dx") port); }
  i
}

#[inline(always)]
pub fn send_u32(port: u16, v: u32) {
  unsafe { asm!("out dx,eax", in("dx") port, in("eax") v); }
}

#[inline(always)]
pub fn read_u32(port: u16) -> u32 {
  let i;
  unsafe { asm!("in eax,dx", out("eax") i, in("dx") port); }
  i
}

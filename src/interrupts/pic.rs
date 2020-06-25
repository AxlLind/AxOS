use crate::io_port::IoPortType;

const PIC1_CMD: u16 = 0x20; const PIC1_DATA: u16 = 0x21;
const PIC2_CMD: u16 = 0xa0; const PIC2_DATA: u16 = 0xa1;

pub fn initialize() {
  u8::io_send(PIC1_CMD,  0x11); // start initialization
  u8::io_send(PIC2_CMD,  0x11); // ..
  u8::io_send(PIC1_DATA, 0x20); // specify the vector offset
  u8::io_send(PIC2_DATA, 0x28); // ..
  u8::io_send(PIC1_DATA, 0x04); // let the pics know about each other
  u8::io_send(PIC2_DATA, 0x02); // ..
  u8::io_send(PIC1_DATA, 0x01); // tell them what type of pic they are
  u8::io_send(PIC2_DATA, 0x01); // ..
  u8::io_send(PIC1_DATA, 0xfe); // enable/disable certain interrupts
  u8::io_send(PIC2_DATA, 0xff); // ..
}

// unsafe since irq has to match the current interrupt
pub unsafe fn end_of_interrupt(irq: u8) {
  if irq >= 8 { u8::io_send(PIC2_CMD, 0x20); }
  u8::io_send(PIC1_CMD, 0x20);
}

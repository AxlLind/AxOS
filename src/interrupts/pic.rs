use crate::io;

const PIC1_CMD: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;

const PIC2_CMD: u16 = 0xa0;
const PIC2_DATA: u16 = 0xa1;

pub fn initialize() {
  io::send(PIC1_CMD, 0x11); // start initialization
  io::send(PIC2_CMD, 0x11); // ..
  io::send(PIC1_DATA, 0x20); // specify the vector offset
  io::send(PIC2_DATA, 0x28); // ..
  io::send(PIC1_DATA, 0x04); // let the pics know about each other
  io::send(PIC2_DATA, 0x02); // ..
  io::send(PIC1_DATA, 0x01); // tell them what type of pic they are
  io::send(PIC2_DATA, 0x01); // ..
  io::send(PIC1_DATA, 0xfe); // enable/disable certain interrupts
  io::send(PIC2_DATA, 0xff); // ..
}

// unsafe since irq has to match the current interrupt
pub unsafe fn end_of_interrupt(irq: u8) {
  if irq >= 8 {
    io::send(PIC2_CMD, 0x20);
  }
  io::send(PIC1_CMD, 0x20);
}

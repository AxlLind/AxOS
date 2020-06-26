#![allow(unused)]
use core::slice;

const ROWS: usize = 25;
const COLS: usize = 80;

#[repr(u8)] // force rust to represent this as we expect
#[derive(Debug, Clone, Copy)]
pub enum VgaColor {
  Black         = 0x0,
  Blue          = 0x1,
  Green         = 0x2,
  Cyan          = 0x3,
  Red           = 0x4,
  Magenta       = 0x5,
  Brown         = 0x6,
  Gray          = 0x7,
  DarkGray      = 0x8,
  BrightBlue    = 0x9,
  BrightGreen   = 0xA,
  BrightCyan    = 0xB,
  BrightRed     = 0xC,
  BrightMagenta = 0xD,
  Yellow        = 0xE,
  White         = 0xF,
}

#[repr(C)] // force rust to represent this as we expect
#[derive(Debug, Clone, Copy)]
struct VgaChar {
  c: u8,
  color: u8,
}

type Buffer = [[VgaChar; COLS]; ROWS];

pub struct VgaDevice {
  mem: &'static mut Buffer,
  row: usize,
  col: usize,
}

impl VgaDevice {
  pub fn new() -> Self {
    // Map the range 0xb8000-0xb8FA0 directly as a 25x80 2d array.
    let mem = unsafe { &mut *(0xb8000 as *mut Buffer) };
    Self {
      mem,
      row: 0,
      col: 0,
    }
  }

  pub fn write_char(
    &mut self,
    row: usize,
    col: usize,
    c: u8,
    text_color: VgaColor,
    background_color: VgaColor,
  ) {
    let color = text_color as u8 | (background_color as u8) << 4;
    self.mem[row][col] = VgaChar { c, color };
  }
}

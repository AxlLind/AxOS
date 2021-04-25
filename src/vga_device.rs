#![allow(unused)]

const ROWS: usize = 25;
const COLS: usize = 80;

#[repr(u8)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct VgaChar {
  c:     u8,
  color: u8,
}

type VgaBuffer = [[VgaChar; COLS]; ROWS];

pub struct VgaDevice {
  buf:   &'static mut VgaBuffer,
  color: u8,
  row:   usize,
  col:   usize,
}

impl VgaDevice {
  pub fn new() -> Self {
    // Map the range 0xb8000-0xb8fa0 directly as a 25x80 2d array.
    let buf = unsafe { &mut *(0xb8000 as *mut VgaBuffer) };
    Self {
      buf,
      color: 0x02, // green text, black background
      row: 0,
      col: 0,
    }
  }

  pub fn reset_color(&mut self) {
    self.color = 0x02;
  }

  pub fn set_color(&mut self, color: VgaColor) {
    self.color &= 0xf0;
    self.color |= color as u8;
  }

  pub fn set_background_color(&mut self, color: VgaColor) {
    self.color &= 0x0f;
    self.color |= (color as u8) << 4;
  }

  pub fn write_char(&mut self, row: usize, col: usize, c: u8) {
    self.buf[row][col] = VgaChar {
      c,
      color: self.color,
    };
  }
}

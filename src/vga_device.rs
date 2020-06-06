#![allow(unused)]
use core::slice;

const ROWS: usize = 25;
const COLS: usize = 80;

pub enum VgaColor {
  Black = 0x0,
  Blue = 0x1,
  Green = 0x2,
  Cyan = 0x3,
  Red = 0x4,
  Magenta = 0x5,
  Brown = 0x6,
  Gray = 0x7,
  DarkGray = 0x8,
  BrightBlue = 0x9,
  BrightGreen = 0xA,
  BrightCyan = 0xB,
  BrightRed = 0xC,
  BrightMagenta = 0xD,
  Yellow = 0xE,
  White = 0xF,
}

pub struct VgaDevice {
  mem: &'static mut [u8],
  row: usize,
  col: usize,
}

impl VgaDevice {
  pub fn new() -> Self {
    let mem = unsafe {
      slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000)
    };
    for i in 0..4000 { mem[i] = 0; }
    Self { mem, row: 0, col: 0 }
  }

  pub fn write_char(&mut self, c: u8) {
    let idx = self.row * COLS + self.col;
    self.mem[idx * 2] = c;
    self.mem[idx * 2 + 1] = VgaColor::Green as u8;
    self.col += 1;
    if self.col == COLS {
      self.col = 0;
      self.row += 1;
      if self.row == ROWS {
        self.scroll();
      }
    }
  }

  pub fn scroll(&mut self) {
    let offset = COLS * 2;
    let size = 4000 - offset;
    for i in 0..size {
      self.mem[i] = self.mem[i + offset];
    }
    for i in 0..offset {
      self.mem[size + i] = 0;
    }
    self.row -= 1;
  }
}

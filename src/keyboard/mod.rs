#![allow(unused)]
mod scan_set_1;
use scan_set_1::Key;

pub const CTRL: u16 = 1 << 0;
pub const SHIFT_LEFT: u16 = 1 << 1;
pub const SHIFT_RIGHT: u16 = 1 << 2;
pub const ALT: u16 = 1 << 3;
pub const ALT_GR: u16 = 1 << 4;
pub const SUPER_KEY_LEFT: u16 = 1 << 5;
pub const SUPER_KEY_RIGHT: u16 = 1 << 6;
pub const CAPS_LOCK: u16 = 1 << 7;
pub const NUM_LOCK: u16 = 1 << 8;
pub const SCROLL_LOCK: u16 = 1 << 9;

#[derive(Copy, Clone, Debug)]
pub struct KeyModifiers(u16);

impl KeyModifiers {
  pub fn ctrl(&self) -> bool {
    self.is_set(CTRL)
  }
  pub fn shift(&self) -> bool {
    self.is_set(SHIFT_LEFT | SHIFT_RIGHT)
  }
  pub fn alt(&self) -> bool {
    self.is_set(ALT)
  }
  pub fn alt_gr(&self) -> bool {
    self.is_set(ALT_GR)
  }
  pub fn super_key(&self) -> bool {
    self.is_set(SUPER_KEY_LEFT | SUPER_KEY_RIGHT)
  }
  pub fn caps_lock(&self) -> bool {
    self.is_set(CAPS_LOCK)
  }
  pub fn num_lock(&self) -> bool {
    self.is_set(CAPS_LOCK)
  }
  pub fn scroll_lock(&self) -> bool {
    self.is_set(SCROLL_LOCK)
  }

  fn set(&mut self, modifier: u16, set: bool) {
    self.0 &= !modifier;
    self.0 |= modifier * (set as u16);
  }

  fn toggle(&mut self, modifier: u16) {
    self.0 ^= modifier;
  }

  fn is_set(&self, mask: u16) -> bool {
    self.0 & mask > 0
  }
}

// safety: keyboard events don't overlap, so this is safe for reads/writes
static mut MODIFIERS: KeyModifiers = KeyModifiers(0);

fn update_modifiers(key: Key, pressed: bool) {
  let modifiers = unsafe { &mut MODIFIERS };
  match (pressed, key) {
    // hold-down keys are set true on key-down and false on key-up
    (_, Key::Alt) => modifiers.set(ALT, pressed),
    (_, Key::Ctrl) => modifiers.set(CTRL, pressed),
    (_, Key::LeftShift) => modifiers.set(SHIFT_LEFT, pressed),
    (_, Key::RightShift) => modifiers.set(SHIFT_RIGHT, pressed),
    // lock keys are toggled on key-down event
    (true, Key::NumLock) => modifiers.toggle(NUM_LOCK),
    (true, Key::CapsLock) => modifiers.toggle(CAPS_LOCK),
    (true, Key::ScrollLock) => modifiers.toggle(SCROLL_LOCK),
    _ => {}
  }
}

pub fn handle_keyboard_event(scan_code: u8) {
  let (key, pressed) = match scan_set_1::decode_key(scan_code) {
    Some(pair) => pair,
    None => return dbg!("Warn: Invalid scan code {}", scan_code),
  };
  update_modifiers(key, pressed);
  if pressed {
    if let Some(c) = key.to_ascii(unsafe { MODIFIERS }) {
      dbg_no_ln!("{}", c);
    }
  }
}

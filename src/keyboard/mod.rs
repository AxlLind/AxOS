#![allow(unused)]
mod scan_set_1;
use scan_set_1::Key;

bitflags::bitflags! {
  pub struct KeyModifiers: u16 {
    const CTRL            = 1 << 0;
    const SHIFT_LEFT      = 1 << 1;
    const SHIFT_RIGHT     = 1 << 2;
    const ALT             = 1 << 3;
    const ALT_GR          = 1 << 4;
    const SUPER_KEY_LEFT  = 1 << 5;
    const SUPER_KEY_RIGHT = 1 << 6;
    const CAPS_LOCK       = 1 << 7;
    const NUM_LOCK        = 1 << 8;
    const SCROLL_LOCK     = 1 << 9;
  }
}

impl KeyModifiers {
  pub fn ctrl(&self) -> bool {
    self.intersects(Self::CTRL)
  }
  pub fn shift(&self) -> bool {
    self.intersects(Self::SHIFT_LEFT | Self::SHIFT_RIGHT)
  }
  pub fn alt(&self) -> bool {
    self.intersects(Self::ALT)
  }
  pub fn alt_gr(&self) -> bool {
    self.intersects(Self::ALT_GR)
  }
  pub fn super_key(&self) -> bool {
    self.intersects(Self::SUPER_KEY_LEFT | Self::SUPER_KEY_RIGHT)
  }
  pub fn caps_lock(&self) -> bool {
    self.intersects(Self::CAPS_LOCK)
  }
  pub fn num_lock(&self) -> bool {
    self.intersects(Self::CAPS_LOCK)
  }
  pub fn scroll_lock(&self) -> bool {
    self.intersects(Self::SCROLL_LOCK)
  }

  fn update(&mut self, key: Key, pressed: bool) {
    match (pressed, key) {
      // hold-down keys are set true on key-down and false on key-up
      (_, Key::Alt) => self.set(KeyModifiers::ALT, pressed),
      (_, Key::Ctrl) => self.set(KeyModifiers::CTRL, pressed),
      (_, Key::LeftShift) => self.set(KeyModifiers::SHIFT_LEFT, pressed),
      (_, Key::RightShift) => self.set(KeyModifiers::SHIFT_RIGHT, pressed),
      // lock keys are toggled on key-down event
      (true, Key::NumLock) => self.toggle(KeyModifiers::NUM_LOCK),
      (true, Key::CapsLock) => self.toggle(KeyModifiers::CAPS_LOCK),
      (true, Key::ScrollLock) => self.toggle(KeyModifiers::SCROLL_LOCK),
      _ => {}
    }
  }
}

// safety: keyboard events don't overlap, can safely read/write
static mut MODIFIERS: KeyModifiers = KeyModifiers::empty();

pub fn handle_keyboard_event(scan_code: u8) {
  let (key, pressed) = match scan_set_1::decode_key(scan_code) {
    Some(pair) => pair,
    None => return dbg!("Warn: Invalid scan code {}", scan_code),
  };
  unsafe { MODIFIERS.update(key, pressed) };
  if pressed {
    if let Some(c) = key.to_ascii(unsafe { MODIFIERS }) {
      dbg_no_ln!("{}", c);
    }
  }
}

use super::KeyModifiers;
use core::mem;

#[allow(unused)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Key {
  _Unknown     = 0x00,
  Escape       = 0x01,
  Num1         = 0x02,
  Num2         = 0x03,
  Num3         = 0x04,
  Num4         = 0x05,
  Num5         = 0x06,
  Num6         = 0x07,
  Num7         = 0x08,
  Num8         = 0x09,
  Num9         = 0x0a,
  Num0         = 0x0b,
  Minus        = 0x0c,
  EqualSign    = 0x0d,
  Backspace    = 0x0e,
  Tab          = 0x0f,
  Q            = 0x10,
  W            = 0x11,
  E            = 0x12,
  R            = 0x13,
  T            = 0x14,
  Y            = 0x15,
  U            = 0x16,
  I            = 0x17,
  O            = 0x18,
  P            = 0x19,
  LeftBracket  = 0x1a,
  RightBracket = 0x1b,
  Enter        = 0x1c,
  Ctrl         = 0x1d,
  A            = 0x1e,
  S            = 0x1f,
  D            = 0x20,
  F            = 0x21,
  G            = 0x22,
  H            = 0x23,
  J            = 0x24,
  K            = 0x25,
  L            = 0x26,
  Semicolon    = 0x27,
  SingleQuote  = 0x28,
  Backtick     = 0x29,
  LeftShift    = 0x2a,
  BackSlash    = 0x2b,
  Z            = 0x2c,
  X            = 0x2d,
  C            = 0x2e,
  V            = 0x2f,
  B            = 0x30,
  N            = 0x31,
  M            = 0x32,
  Comma        = 0x33,
  Period       = 0x34,
  Slash        = 0x35,
  RightShift   = 0x36,
  PadStar      = 0x37,
  Alt          = 0x38,
  Space        = 0x39,
  CapsLock     = 0x3a,
  F1           = 0x3b,
  F2           = 0x3c,
  F3           = 0x3d,
  F4           = 0x3e,
  F5           = 0x3f,
  F6           = 0x40,
  F7           = 0x41,
  F8           = 0x42,
  F9           = 0x43,
  F10          = 0x44,
  NumLock      = 0x45,
  ScrollLock   = 0x46,
  Pad7         = 0x47,
  Pad8         = 0x48,
  Pad9         = 0x49,
  PadMinus     = 0x4a,
  Pad4         = 0x4b,
  Pad5         = 0x4c,
  Pad6         = 0x4d,
  PadPlus      = 0x4e,
  Pad1         = 0x4f,
  Pad2         = 0x50,
  Pad3         = 0x51,
  Pad0         = 0x52,
  PadPeriod    = 0x53,
  _Unknown1    = 0x54,
  _Unknown2    = 0x55,
  _Unknown3    = 0x56,
  F11          = 0x57,
  F12          = 0x58,
}

static ASCII_KEY_MAP: &[u8] = b"\0\x1b1234567890-=\x08\tqwertyuiop[]\n\0asdfghjkl;\'`\0\\zxcvbnm,./\0*\0 \0\0\0\0\0\0\0\0\0\0\0\0\0789-456+1230.\0\0\0\0\0";
static ASCII_SHIFT_KEY_MAP: &[u8] = b"\0\0!@#$%^&*()_+\0\0QWERTYUIOP{}\0\0ASDFGHJKL:\"~\0|ZXCVBNM<>?\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

impl Key {
  pub fn from_code(scan_code: u8) -> Option<Self> {
    match scan_code {
      0 | 0x54 | 0x55 | 0x56 => None,
      1..=0x58 => Some(unsafe { mem::transmute(scan_code) }),
      _ => None,
    }
  }

  #[rustfmt::skip]
  pub fn is_letter(&self) -> bool {
    use Key::*;
    matches!(self, A | B | C | D | E | F | G | H | I | J | K | L | M | N | O | P | Q | R | S | T | U | V | W | X | Y | Z)
  }

  pub fn to_ascii(self, modifiers: KeyModifiers) -> Option<char> {
    let shifted = modifiers.shift() || (modifiers.caps_lock() && self.is_letter());
    let map = if shifted {
      ASCII_SHIFT_KEY_MAP
    } else {
      ASCII_KEY_MAP
    };
    match map[self as usize] {
      0 => None,
      b => Some(b as char),
    }
  }
}

pub fn decode_key(scan_code: u8) -> Option<(Key, bool)> {
  let (key_code, pressed) = if scan_code > 0x80 {
    (scan_code - 0x80, false)
  } else {
    (scan_code, true)
  };
  let key = Key::from_code(key_code)?;
  Some((key, pressed))
}

mod tests {
  use super::*;

  #[test_case]
  fn decode_valid() {
    assert_eq!(decode_key(0x10), Some((Key::Q, true)));
    assert_eq!(decode_key(0x90), Some((Key::Q, false)));

    assert_eq!(decode_key(0x58), Some((Key::F12, true)));
    assert_eq!(decode_key(0x58 + 0x80), Some((Key::F12, false)));
  }

  #[test_case]
  fn decode_invalid() {
    assert_eq!(decode_key(0), None);
    assert_eq!(decode_key(0x54), None);
    assert_eq!(decode_key(0x55), None);
    assert_eq!(decode_key(0x56), None);

    assert_eq!(decode_key(0x80 + 0), None);
    assert_eq!(decode_key(0x80 + 0x54), None);
    assert_eq!(decode_key(0x80 + 0x55), None);
    assert_eq!(decode_key(0x80 + 0x56), None);
  }

  #[test_case]
  fn ascii_with_modifiers() {
    let no_modifiers = KeyModifiers::empty();
    let with_caps = KeyModifiers::CAPS_LOCK;
    let with_shift = KeyModifiers::SHIFT_LEFT;
    let with_caps_and_shift = with_caps | with_shift;

    assert_eq!(Key::A.to_ascii(no_modifiers), Some('a'));
    assert_eq!(Key::A.to_ascii(with_caps), Some('A'));
    assert_eq!(Key::A.to_ascii(with_shift), Some('A'));
    assert_eq!(Key::A.to_ascii(with_caps_and_shift), Some('A'));

    assert_eq!(Key::Minus.to_ascii(no_modifiers), Some('-'));
    assert_eq!(Key::Comma.to_ascii(no_modifiers), Some(','));
    assert_eq!(Key::Slash.to_ascii(no_modifiers), Some('/'));

    assert_eq!(Key::Minus.to_ascii(with_caps), Some('-'));
    assert_eq!(Key::Comma.to_ascii(with_caps), Some(','));
    assert_eq!(Key::Slash.to_ascii(with_caps), Some('/'));

    assert_eq!(Key::Minus.to_ascii(with_shift), Some('_'));
    assert_eq!(Key::Comma.to_ascii(with_shift), Some('<'));
    assert_eq!(Key::Slash.to_ascii(with_shift), Some('?'));

    assert_eq!(Key::Minus.to_ascii(with_caps_and_shift), Some('_'));
    assert_eq!(Key::Comma.to_ascii(with_caps_and_shift), Some('<'));
    assert_eq!(Key::Slash.to_ascii(with_caps_and_shift), Some('?'));

    assert_eq!(Key::CapsLock.to_ascii(no_modifiers), None);
    assert_eq!(Key::CapsLock.to_ascii(with_caps), None);
    assert_eq!(Key::CapsLock.to_ascii(with_shift), None);
    assert_eq!(Key::CapsLock.to_ascii(with_caps_and_shift), None);
  }

  #[test_case]
  fn invalid_letters() {
    let modifiers = KeyModifiers::empty();
    assert_eq!(Key::_Unknown.to_ascii(modifiers), None);
    assert_eq!(Key::_Unknown1.to_ascii(modifiers), None);
    assert_eq!(Key::_Unknown2.to_ascii(modifiers), None);
    assert_eq!(Key::_Unknown3.to_ascii(modifiers), None);
    assert_eq!(Key::F1.to_ascii(modifiers), None);
  }
}

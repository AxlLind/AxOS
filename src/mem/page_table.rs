#![allow(dead_code)]
use super::PhysAddr;
use crate::indexable_from_field;

const PRESENT: u64 = 1 << 0;
const WRITABLE: u64 = 1 << 1;
const USER_ACCESSIBLE: u64 = 1 << 2;
const WRITE_THROUGH: u64 = 1 << 3;
const DISABLE_CACHE: u64 = 1 << 4;
const ACCESSED: u64 = 1 << 5;
const DIRTY: u64 = 1 << 6;
const HUGE: u64 = 1 << 7;
const GLOBAL: u64 = 1 << 8;
const NON_EXECUTABLE: u64 = 1 << 63;

const PHYS_ADDR_MASK: u64 = 0x000f_ffff_ffff_f000;

// Reference: https://os.phil-opp.com/paging-introduction/#page-table-format
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct PageTableEntry(pub u64);

impl PageTableEntry {
  pub fn addr(&mut self) -> PhysAddr {
    PhysAddr::new(self.0 & PHYS_ADDR_MASK)
  }

  pub unsafe fn set_addr(&mut self, addr: PhysAddr) -> &mut Self {
    // addr has to be page aligned and small enough
    assert_eq!(addr.as_u64() & !PHYS_ADDR_MASK, 0);
    self.0 = (self.0 & !PHYS_ADDR_MASK) | addr.as_u64();
    self
  }

  pub fn unused(&self) -> bool {
    self.0 == 0
  }

  pub fn present(&self) -> bool {
    self.is_bit_set(PRESENT)
  }
  pub fn writable(&self) -> bool {
    self.is_bit_set(WRITABLE)
  }
  pub fn user_accessible(&self) -> bool {
    self.is_bit_set(USER_ACCESSIBLE)
  }
  pub fn accessed(&self) -> bool {
    self.is_bit_set(ACCESSED)
  }
  pub fn dirty(&self) -> bool {
    self.is_bit_set(DIRTY)
  }
  pub fn non_executable(&self) -> bool {
    self.is_bit_set(NON_EXECUTABLE)
  }

  pub fn set_present(&mut self, b: bool) -> &mut Self {
    self.set_bit(PRESENT, b)
  }
  pub fn set_writable(&mut self, b: bool) -> &mut Self {
    self.set_bit(WRITABLE, b)
  }
  pub fn set_user_accessible(&mut self, b: bool) -> &mut Self {
    self.set_bit(USER_ACCESSIBLE, b)
  }
  pub fn set_non_executable(&mut self, b: bool) -> &mut Self {
    self.set_bit(NON_EXECUTABLE, b)
  }

  fn is_bit_set(&self, bit: u64) -> bool {
    self.0 & bit != 0
  }

  fn set_bit(&mut self, bit: u64, b: bool) -> &mut Self {
    self.0 &= !bit;
    self.0 |= bit * (b as u64);
    self
  }
}

#[repr(C, align(4096))]
pub struct PageTable([PageTableEntry; 512]);

impl PageTable {
  pub fn new() -> Self {
    Self([PageTableEntry(0); 512])
  }
}

indexable_from_field!(PageTable, 0, PageTableEntry);

#[cfg(test)]
mod tests {
  use super::*;

  #[test_case]
  fn size_check() {
    use core::mem::size_of;
    assert_eq!(size_of::<PageTableEntry>(), 8);
    assert_eq!(size_of::<PageTable>(), 4096);
  }

  #[test_case]
  fn page_entry_bit_setting() {
    let mut entry = PageTableEntry(0);
    macro_rules! bit_test {
      ($check:tt, $set:tt) => {{
        assert!(!entry.$check());
        assert!(entry.$set(true).$check());
        assert!(entry.$set(true).$check());
        assert!(!entry.$set(false).$check());
        assert!(!entry.$set(false).$check());
      }};
    }
    bit_test!(present, set_present);
    bit_test!(writable, set_writable);
    bit_test!(non_executable, set_non_executable);
    bit_test!(user_accessible, set_user_accessible);
  }
}

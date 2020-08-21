#![allow(dead_code)]
use super::{PhysAddr, VirtAddr};
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
pub struct PageTableEntry(u64);

impl PageTableEntry {
  pub fn addr(&self) -> PhysAddr {
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

pub fn cr3() -> (u64, u64) {
  let cr3: u64;
  unsafe { asm!("mov {}, cr3", out(reg) cr3) };
  (cr3 & PHYS_ADDR_MASK, cr3 & !PHYS_ADDR_MASK)
}

pub unsafe fn active_level_four_table() -> &'static mut PageTable {
  let (cr3, _) = cr3();
  let addr = PhysAddr::new(cr3).to_virt();
  &mut *addr.as_mut_ptr()
}

pub unsafe fn translate_addr(addr: VirtAddr) -> Option<PhysAddr> {
  let indexes = [
    (addr.as_u64() >> 39) & 0x1ff,
    (addr.as_u64() >> 30) & 0x1ff,
    (addr.as_u64() >> 21) & 0x1ff,
    (addr.as_u64() >> 12) & 0x1ff,
  ];
  let level_four_page = PhysAddr::new(cr3().0);
  indexes
    .iter()
    .fold(Some(level_four_page), |table_addr, &index| {
      let table_ptr = table_addr?.to_virt().as_ptr::<PageTable>();
      let table = unsafe { &*table_ptr };
      let entry = table[index as usize];
      if entry.unused() {
        return None;
      }
      Some(entry.addr())
    })
    .map(|res| {
      let offset = addr.as_u64() & 0xfff;
      PhysAddr::new(res.as_u64() + offset)
    })
}

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

  #[test_case]
  fn virt_to_phys_translation() {
    // works since we know the VGA buffer is identity-mapped
    let virt_addr = VirtAddr::new(0xb8001);
    let phys_addr = unsafe { translate_addr(virt_addr).unwrap() };
    assert_eq!(phys_addr.as_u64(), 0xb8001);
  }
}

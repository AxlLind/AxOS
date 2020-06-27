use super::gdt::current_cs;
use super::DescriptorTablePtr;
use core::ops::{Index, IndexMut};

// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table#IDT_in_IA-32e_Mode_.2864-bit_IDT.29
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IdtEntry {
  ptr_low:  u16,
  selector: u16,
  options:  u16,
  ptr_mid:  u16,
  ptr_high: u32,
  reserved: u32,
}

impl IdtEntry {
  fn unimplemented() -> Self {
    Self {
      ptr_low:  0,
      selector: 0,
      options:  0xe00,
      ptr_mid:  0,
      ptr_high: 0,
      reserved: 0,
    }
  }

  pub fn set_handler(&mut self, fn_ptr: usize) -> &mut Self {
    self.selector = current_cs();
    self.ptr_low = fn_ptr as u16;
    self.ptr_mid = (fn_ptr >> 16) as u16;
    self.ptr_high = (fn_ptr >> 32) as u32;
    self.options |= 1 << 15;
    self
  }

  pub fn with_ist(&mut self, stack_index: u16) {
    self.options |= stack_index;
  }
}

#[repr(C)]
pub struct InterruptDescriptorTable([IdtEntry; 256]);

impl InterruptDescriptorTable {
  pub fn new() -> Self {
    Self([IdtEntry::unimplemented(); 256])
  }

  // Safe since the IDT is static
  pub fn load(&'static self) {
    let ptr = DescriptorTablePtr::ptr_to(self);
    unsafe { asm!("lidt [{}]", in(reg) &ptr) };
  }
}

impl Index<usize> for InterruptDescriptorTable {
  type Output = IdtEntry;

  fn index(&self, i: usize) -> &Self::Output {
    &self.0[i]
  }
}

impl IndexMut<usize> for InterruptDescriptorTable {
  fn index_mut(&mut self, i: usize) -> &mut Self::Output {
    &mut self.0[i]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test_case]
  fn size_check() {
    use core::mem::size_of;
    assert_eq!(size_of::<IdtEntry>(), 16);
    assert_eq!(size_of::<InterruptDescriptorTable>(), 256 * 16);
  }

  #[test_case]
  fn idt_entry_set_fns() {
    let mut entry = IdtEntry::unimplemented();
    assert_eq!(entry.options, 0xe00);

    entry.set_handler(0x0000111122223333);
    assert_eq!(entry.selector, current_cs());
    assert_eq!(entry.options, 0x8e00);
    assert_eq!(entry.ptr_low, 0x3333);
    assert_eq!(entry.ptr_mid, 0x2222);
    assert_eq!(entry.ptr_high, 0x1111);

    entry.with_ist(5);
    assert_eq!(entry.options, 0x8e05);
  }
}

use core::mem::size_of;
use super::DescriptorTablePtr;
use super::gdt::current_cs;

// Reference: https://wiki.osdev.org/Interrupt_Descriptor_Table#IDT_in_IA-32e_Mode_.2864-bit_IDT.29
#[derive(Clone,Copy)]
#[repr(C)]
struct IdtEntry {
  ptr_low: u16,
  gdt_selector: u16,
  options: u16,
  ptr_mid: u16,
  ptr_high: u32,
  reserved: u32,
}

#[repr(C)]
pub struct InterruptDescriptorTable([IdtEntry; 256]);

impl InterruptDescriptorTable {
  pub fn new() -> Self {
    let unimplemented_entry = IdtEntry {
      ptr_low: 0,
      gdt_selector: 0,
      options: 0xe00,
      ptr_mid: 0,
      ptr_high: 0,
      reserved: 0,
    };
    Self([unimplemented_entry; 256])
  }

  pub fn set_handler(&mut self, i: usize, fn_ptr: u64) {
    self.0[i].gdt_selector = current_cs();
    self.0[i].ptr_low = fn_ptr as u16;
    self.0[i].ptr_mid = (fn_ptr >> 16) as u16;
    self.0[i].ptr_high = (fn_ptr >> 32) as u32;
    self.0[i].options |= 1 << 15;
  }

  // Safe since the IDT is static
  pub fn load(&'static self) {
    let ptr = DescriptorTablePtr {
      size: size_of::<Self>() as u16 - 1,
      base_ptr: self as *const _ as u64,
    };
    unsafe { asm!("lidt [{}]", in(reg) &ptr); }
  }
}

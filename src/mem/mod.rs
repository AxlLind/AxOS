#![allow(dead_code)]
pub mod frame_allocator;
pub mod page_table;

pub const PHYS_MEM_OFFSET: u64 = 0x20000000000; // specified in Cargo.toml

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
  pub fn new(addr: u64) -> Self {
    Self(addr)
  }

  pub fn as_u64(&self) -> u64 {
    self.0
  }

  pub fn as_mut_ptr<T>(&self) -> *mut T {
    self.0 as *mut T
  }

  pub fn as_ptr<T>(&self) -> *const T {
    self.0 as *const T
  }

  pub fn is_page_aligned(&self) -> bool {
    self.0.trailing_zeros() >= 12
  }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct PhysAddr(u64);

impl PhysAddr {
  pub fn new(addr: u64) -> Self {
    assert!(addr.leading_zeros() >= 10);
    Self(addr)
  }

  pub fn as_u64(&self) -> u64 {
    self.0
  }

  pub fn to_virt(&self) -> VirtAddr {
    VirtAddr::new(self.0 + PHYS_MEM_OFFSET)
  }

  pub fn is_page_aligned(&self) -> bool {
    self.0.trailing_zeros() >= 12
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test_case]
  fn size_check() {
    use core::mem::size_of;
    assert_eq!(size_of::<VirtAddr>(), 8);
    assert_eq!(size_of::<PhysAddr>(), 8);
  }

  #[test_case]
  fn virt_addr_as_ptr() {
    let stack_int = 1337u64;
    let ptr = &stack_int as *const _ as u64;
    let virt = VirtAddr::new(ptr);
    let maybe_int = unsafe { *virt.as_ptr() };
    assert_eq!(stack_int, maybe_int);
  }

  #[test_case]
  fn phys_addr_to_virt() {
    // Use the VGA buffer as a test case since we know it is identity mapped
    let ptr = unsafe { PhysAddr::new(0xb8000).to_virt().as_mut_ptr::<u8>() };
    unsafe { *ptr = 42 };
    let value_at_virt_addr = unsafe { *VirtAddr::new(0xb8000).as_mut_ptr::<u8>() };
    assert_eq!(value_at_virt_addr, 42);
  }
}

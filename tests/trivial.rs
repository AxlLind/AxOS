#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ax_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

ax_os::test_prelude!();

#[test_case]
fn trivial_test() {
  assert_eq!(1, 1);
}

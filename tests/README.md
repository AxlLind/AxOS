# Integration tests
This folder contains integration tests. They are completely separate binaries that test some specific feature where a simple unit test does not suffice. For example in [`stack_overflow.rs`](./stack_overflow.rs) we have to set up our custom [GDT](../src/interrupts/gdt.rs) and double fault handler to be able to test that we handle a kernel stack overflow.

### How to make an integration test
Making an integration test is fairly simple. Here is a skeleton of what you need to get going:

```Rust
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
```

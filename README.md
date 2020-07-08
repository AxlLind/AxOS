# AxOS
A toy OS in Rust made just to learn more about operating systems and for fun.

:warning: This is very much a work in progress

![hello world screenshot](./screenshots/hello-world.png)

## Status
- [x] Up and running
- [x] Debug printing via Qemu stdout
- [x] Interrupts handled via IDT
- [x] External interrupts enabled via PIC
- [ ] Proper memory paging handling
- [ ] Kernel heap allocation

## Development
To get started run the [`setup.sh`](./setup.sh) script. Commands:

```sh
./setup.sh   # install required dependencies
cargo xrun   # run the operating system
cargo xcheck # check for warnings/errors without running
cargo xtest  # run all unit and integration tests
```

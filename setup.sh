#! /bin/bash

# General rust toolchains are needed
if ! rustup --version > /dev/null 2>&1 ; then
  curl https://sh.rustup.rs -sSf | sh ||
    { echo 'Installing rustup failed' ; exit 1; }
fi

# Rust nightly compiler is needed for certain features
if ! rustup show | grep nightly > /dev/null 2>&1 ; then
  rustup update nightly ||
    { echo 'Installing rust nightly failed' ; exit 1; }
fi

# Builds bootable images from rust source
if ! bootimage --help > /dev/null 2>&1 ; then
  cargo install bootimage ||
    { echo 'Installing bootimage failed' ; exit 1; }
fi

# Required to build our on stdlib from source
if ! cargo xbuild --version > /dev/null 2>&1 ; then
  cargo install cargo-xbuild ||
    { echo 'Installing cargo-xbuild failed' ; exit 1; }
fi

# Source files for rust core, needed to compile our own stdlib
if ! rustup component list --toolchain=nightly | grep "rust-src.*installed" > /dev/null 2>&1 ; then
  rustup component add rust-src --toolchain=nightly ||
    { echo 'Installing rust-src failed' ; exit 1; }
fi

# QEMU is required to run the OS in a VM
if ! qemu-system-x86_64 --version > /dev/null 2>&1 ; then
  echo "Please install qemu: https://www.qemu.org/download/"
  exit 1
fi

echo "All requirements should be installed!"

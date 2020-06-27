#!/bin/sh

function fail {
  echo "Installing $1 failed!"
  exit 1
}

# use the custom githooks
git config core.hooksPath .githooks

# General rust toolchains are needed
if ! rustup --version > /dev/null 2>&1 ; then
  curl https://sh.rustup.rs -sSf | sh || fail "rustup"
fi

# Used by some custom scripts
if ! rg --version > /dev/null 2>&1 ; then
  cargo install ripgrep || fail "ripgrep"
fi

# Rust nightly compiler is needed for certain features
if ! rustup show | grep nightly > /dev/null 2>&1 ; then
  rustup update nightly || fail "rust nightly"
fi

# Builds bootable images from rust source
if ! bootimage --help > /dev/null 2>&1 ; then
  cargo install bootimage || fail "bootimage"
fi

# Required to custom build to our target
if ! cargo xbuild --version > /dev/null 2>&1 ; then
  cargo install cargo-xbuild || fail "cargo-xbuild"
fi

# Source files for rust core, needed to compile core to our target
if ! rustup component list --toolchain=nightly | grep "rust-src.*installed" > /dev/null 2>&1 ; then
  rustup component add rust-src --toolchain=nightly || fail "rust-src"
fi

# Source files for rust core, needed to compile core to our target
if ! rustup component list --toolchain=nightly | grep "llvm-tools-preview.*installed" > /dev/null 2>&1 ; then
  rustup component add llvm-tools-preview --toolchain=nightly || fail "rust-src"
fi

# Required VM to run the OS in
if ! qemu-system-x86_64 --version > /dev/null 2>&1 ; then
  echo "Please install qemu: https://www.qemu.org/download/"
  exit 1
fi

echo "All requirements should now be installed!"

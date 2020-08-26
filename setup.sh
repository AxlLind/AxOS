#!/bin/sh

function fail {
  echo "Installing $1 failed!"
  exit 1
}

# Use custom githooks
git config core.hooksPath .github
echo "Setting custom githooks path"

# General rust toolchains are needed
if rustup --version > /dev/null 2>&1 ; then
  echo "Already installed: rustup"
else
  curl https://sh.rustup.rs -sSf | sh || fail "rustup"
fi

# Rust nightly compiler is needed for certain features
if rustup show | grep nightly > /dev/null 2>&1 ; then
  echo "Already installed: rust nightly"
else
  rustup update nightly || fail "rust nightly"
fi

# Builds bootable images from rust source
if bootimage --help > /dev/null 2>&1 ; then
  echo "Already installed: bootimage"
else
  cargo install bootimage || fail "bootimage"
fi

# Required to build to our custom target
if cargo xbuild --version > /dev/null 2>&1 ; then
  echo "Already installed: cargo-xbuild"
else
  cargo install cargo-xbuild || fail "cargo-xbuild"
fi

# Source files for rust core, needed to compile core to our custom target
if rustup component list | grep "rust-src.*installed" > /dev/null 2>&1 ; then
  echo "Already installed: rust-src"
else
  rustup component add rust-src || fail "rust-src"
fi

# Needed by bootimage to map our kernel binary
if rustup component list | grep "llvm-tools-preview.*installed" > /dev/null 2>&1 ; then
  echo "Already installed: llvm-tools-preview"
else
  rustup component add llvm-tools-preview || fail "rust-src"
fi

# Required VM to run the OS in
if qemu-system-x86_64 --version > /dev/null 2>&1 ; then
  echo "Already installed: qemu"
else
  echo "Please install qemu: https://www.qemu.org/download/"
  exit 1
fi

echo "All requirements should now be installed!"

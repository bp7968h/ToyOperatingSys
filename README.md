# Toy Operating System in Rust

This project is a small, educational operating system written in Rust, following Philipp Oppermann's [*Writing an OS in Rust*](https://os.phil-opp.com/) blog series. The goal is to explore the low-level workings of operating systems while learning the unsafe Rust programming language.

## Features

- **No Standard Library**: This OS runs in `#![no_std]` and `#![no_main]` environments, meaning it doesn't rely on Rust's standard library.
- **VGA Text Mode Output**: Print strings directly to the VGA buffer in text mode.
- **Basic Kernel**: A minimal kernel that serves as the starting point for further development.
- **Cross-Compilation**: Builds for a bare-metal target (no operating system).
- **Bootloader**: Uses the `bootloader` crate for creating a bootable binary.

## Prerequisites

Before building and running the OS, you need to have the following tools installed:

- [Rust](https://www.rust-lang.org/tools/install)
- `nightly` rust toolchain
  ```bash
  rustup install nightly
  ```
- `rustup` component for cross-compilation
  ```bash
  rustup target add x86_64-unknown-none
  ```
- `rust-src` source code for standard library
  ```bash
  rustup component add rust-src --toolchain nightly
  ```
- `bootimage` for creating a bootable image
  ```bash
  cargo install bootimage
  ```
- `llvm-tools-preview` from llvm tools
  ```bash
  rustup component add llvm-tools-preview
  ```
- [QEMU](https://www.qemu.org/download/#linux)

## Cloning the Repository

To get started, clone this repository to your local machine, all files are necessary to run this project.

```bash
git clone https://github.com/bp7968h/ToyOperatingSys.git
cd ToyOperatingSys
```

## Running
While running the bootimage is setup to run with qemu emulator without needing hardware.
```bash
cargo run
```

## Acknowledgements

- [Philipp Oppermann's Blog Series](https://os.phil-opp.com/), which served as the main guide for building this project.
- The Rust OS development community.
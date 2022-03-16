#!/bin/sh

# Apple arm64
cargo build --release --target=aarch64-apple-darwin

# Apple X86_64
cargo build --release --target=x86_64-apple-darwin

# Linux X86_64
CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl

# Windows X86_64
cargo build --release --target=x86_64-pc-windows-gnu

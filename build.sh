#!/bin/sh

binDir="./bin"

version="1.3.0"

# Apple arm64
cargo build --release --target=aarch64-apple-darwin

cp target/aarch64-apple-darwin/release/sign-message-rs $binDir/MAC_ARM64_V$version.bin

# Apple X86_64
cargo build --release --target=x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/sign-message-rs $binDir/MAC_X86_64_V$version.bin

# Linux X86_64
CROSS_COMPILE=x86_64-linux-musl- cargo build --release --target x86_64-unknown-linux-musl

cp target/x86_64-unknown-linux-musl/release/sign-message-rs $binDir/Linux_X86_64_V$version.bin
# Windows X86_64
cargo build --release --target=x86_64-pc-windows-gnu

cp target/x86_64-pc-windows-gnu/release/sign-message-rs.exe $binDir/Windows_X86_64_V$version.exe

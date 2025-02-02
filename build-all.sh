#!/bin/bash
set -e

if [[ "$OSTYPE" != "darwin"* ]]; then
    echo "This script must be run on macOS"
    exit 1
fi

mkdir -p target/release/output

if ! command -v x86_64-w64-mingw32-gcc &> /dev/null; then
    echo "Installing MinGW-w64..."
    brew install mingw-w64
fi

if ! command -v x86_64-linux-musl-gcc &> /dev/null; then
    echo "Installing Linux cross compiler..."
    brew install FiloSottile/musl-cross/musl-cross
fi

rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add x86_64-apple-darwin

echo "Building all platforms..."

echo "Building Windows version..."
cross build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/hardware_id.exe target/release/output/

echo "Building Linux version..."
CROSS_COMPILE=x86_64-linux-musl- cross build --release --target x86_64-unknown-linux-musl
cp target/x86_64-unknown-linux-musl/release/hardware_id target/release/output/hardware_id_linux

echo "Building macOS Intel version..."
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/hardware_id target/release/output/hardware_id_mac

echo "Build completed!"
ls -l target/release/output/ 
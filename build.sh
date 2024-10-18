#!/bin/bash

# Script to build Nimbus for Windows and Linux

# Ensure we're in the project root directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Cargo.toml not found. Please run this script from the project root directory."
    exit 1
fi

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check if Rust and Cargo are installed
if ! command_exists cargo; then
    echo "Error: Rust and Cargo are required but not installed. Please install them first."
    exit 1
fi

# Build for Linux (current platform)
echo "Building for Linux..."
cargo build --release

# Check if the Windows GNU target is installed
if ! rustc -vV | grep -q "host: x86_64-pc-windows-gnu"; then
    echo "Installing Windows GNU target..."
    rustup target add x86_64-pc-windows-gnu
fi

# Check if MinGW-w64 is installed
if ! command_exists x86_64-w64-mingw32-gcc; then
    echo "Error: MinGW-w64 is required for Windows builds but not installed."
    echo "Please install it using your system's package manager."
    echo "For example, on Ubuntu or Debian:"
    echo "sudo apt-get install mingw-w64"
    exit 1
fi

# Build for Windows
echo "Building for Windows..."
cargo build --release --target x86_64-pc-windows-gnu

# Create a directory for the builds
mkdir -p builds

# Move the Linux build
mv target/release/nimbus builds/nimbus-linux

# Move the Windows build
mv target/x86_64-pc-windows-gnu/release/nimbus.exe builds/nimbus-windows.exe

echo "Build complete. Executables are in the 'builds' directory."
#!/bin/bash
set -e

# Mahoraga Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/your-repo/mahoraga/main/scripts/install.sh | bash

REPO="your-username/mahoraga"
INSTALL_DIR="${HOME}/.local/bin"

echo "Installing Mahoraga..."

# Check for required dependencies
if ! command -v cargo &> /dev/null; then
    echo "Error: Rust/Cargo is required but not installed."
    echo "Please install Rust from https://rustup.rs and try again."
    exit 1
fi

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Clone or download the repository
TEMP_DIR=$(mktemp -d)
cd "$TEMP_DIR"

echo "Downloading Mahoraga..."
if command -v git &> /dev/null; then
    git clone --depth 1 "https://github.com/${REPO}.git" mahoraga
else
    curl -sL "https://github.com/${REPO}/archive/main.tar.gz" | tar xz
    mv mahoraga-main mahoraga
fi

cd mahoraga

# Build the project
echo "Building Mahoraga..."
cargo build --release

# Install binary
echo "Installing binary to $INSTALL_DIR..."
cp target/release/mahoraga "$INSTALL_DIR/"

# Cleanup
cd /
rm -rf "$TEMP_DIR"

echo ""
echo "Mahoraga installed successfully!"
echo ""
echo "Make sure $INSTALL_DIR is in your PATH."
echo "Run 'mahoraga summon' to start the prompt validator."
echo ""

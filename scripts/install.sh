#!/bin/bash
set -e

# Mahoraga Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/your-repo/mahoraga/main/scripts/install.sh | bash

REPO="your-username/mahoraga"
INSTALL_DIR="${HOME}/.local/bin"

echo "Installing Mahoraga..."

# Check for required dependencies
if ! command -v node &> /dev/null; then
    echo "Error: Node.js is required but not installed."
    echo "Please install Node.js (v18 or higher) and try again."
    exit 1
fi

if ! command -v npm &> /dev/null && ! command -v pnpm &> /dev/null; then
    echo "Error: npm or pnpm is required but not installed."
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

# Install dependencies and build
echo "Installing dependencies..."
if command -v pnpm &> /dev/null; then
    pnpm install
    pnpm build
else
    npm install
    npm run build
fi

# Install globally
echo "Installing globally..."
if command -v pnpm &> /dev/null; then
    pnpm link --global
else
    npm link
fi

# Cleanup
cd /
rm -rf "$TEMP_DIR"

echo ""
echo "Mahoraga installed successfully!"
echo ""
echo "Run 'mahoraga summon' to start the prompt validator."
echo ""

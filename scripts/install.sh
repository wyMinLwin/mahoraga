#!/bin/bash
set -e

# Mahoraga Installation Script
# Usage: curl -fsSL https://raw.githubusercontent.com/wyMinLwin/mahoraga/main/scripts/install.sh | bash

REPO="wyMinLwin/mahoraga"
BIN_NAME="mahoraga"
INSTALL_DIR="${MAHORAGA_INSTALL_DIR:-${HOME}/.local/bin}"

info() { printf "\033[1;34m%s\033[0m\n" "$1"; }
success() { printf "\033[1;32m%s\033[0m\n" "$1"; }
error() { printf "\033[1;31mError: %s\033[0m\n" "$1" >&2; exit 1; }

detect_platform() {
    local os arch

    case "$(uname -s)" in
        Linux*)  os="unknown-linux-gnu" ;;
        Darwin*) os="apple-darwin" ;;
        *)       error "Unsupported OS: $(uname -s)" ;;
    esac

    case "$(uname -m)" in
        x86_64|amd64)  arch="x86_64" ;;
        arm64|aarch64) arch="aarch64" ;;
        *)             error "Unsupported architecture: $(uname -m)" ;;
    esac

    echo "${arch}-${os}"
}

get_latest_version() {
    if command -v curl &> /dev/null; then
        curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p'
    elif command -v wget &> /dev/null; then
        wget -qO- "https://api.github.com/repos/${REPO}/releases/latest" | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p'
    else
        error "curl or wget is required"
    fi
}

download() {
    local url="$1" dest="$2"
    if command -v curl &> /dev/null; then
        curl -fsSL "$url" -o "$dest"
    elif command -v wget &> /dev/null; then
        wget -qO "$dest" "$url"
    fi
}

install_from_release() {
    local platform="$1"
    local version="$2"
    local archive="${BIN_NAME}-${version}-${platform}.tar.gz"
    local url="https://github.com/${REPO}/releases/download/${version}/${archive}"

    info "Downloading ${BIN_NAME} ${version} for ${platform}..."

    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    download "$url" "${tmp_dir}/${archive}" || return 1

    tar xzf "${tmp_dir}/${archive}" -C "$tmp_dir"

    mkdir -p "$INSTALL_DIR"
    cp "${tmp_dir}/${BIN_NAME}-${version}-${platform}/${BIN_NAME}" "$INSTALL_DIR/"
    chmod +x "${INSTALL_DIR}/${BIN_NAME}"
}

install_from_source() {
    info "No pre-built binary found. Building from source..."

    if ! command -v cargo &> /dev/null; then
        error "Rust/Cargo is required to build from source. Install from https://rustup.rs"
    fi

    local tmp_dir
    tmp_dir=$(mktemp -d)
    trap 'rm -rf "$tmp_dir"' EXIT

    if command -v git &> /dev/null; then
        git clone --depth 1 "https://github.com/${REPO}.git" "${tmp_dir}/mahoraga"
    else
        download "https://github.com/${REPO}/archive/main.tar.gz" "${tmp_dir}/source.tar.gz"
        tar xzf "${tmp_dir}/source.tar.gz" -C "$tmp_dir"
        mv "${tmp_dir}/mahoraga-main" "${tmp_dir}/mahoraga"
    fi

    info "Building (this may take a minute)..."
    cd "${tmp_dir}/mahoraga"
    cargo build --release

    mkdir -p "$INSTALL_DIR"
    cp "target/release/${BIN_NAME}" "$INSTALL_DIR/"
    chmod +x "${INSTALL_DIR}/${BIN_NAME}"
}

check_path() {
    if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
        echo ""
        info "Add ${INSTALL_DIR} to your PATH:"
        echo ""
        echo "  # Add to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
        echo "  export PATH=\"${INSTALL_DIR}:\$PATH\""
        echo ""
    fi
}

main() {
    echo ""
    info "Installing Mahoraga..."
    echo ""

    local platform version
    platform=$(detect_platform)

    version=$(get_latest_version)
    if [ -z "$version" ]; then
        info "No releases found. Building from source..."
        install_from_source
    elif install_from_release "$platform" "$version"; then
        : # success
    else
        install_from_source
    fi

    echo ""
    success "Mahoraga installed successfully to ${INSTALL_DIR}/${BIN_NAME}"
    check_path
    echo "  Run 'mahoraga summon' to start."
    echo ""
}

main

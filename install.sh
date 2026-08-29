#!/usr/bin/env bash
set -e

REPO="reneboygarcia/webp-converter"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

echo "📥 Installing webp-converter..."

# Detect OS and Architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    PLATFORM="macos"
    ;;
  Linux)
    PLATFORM="linux"
    ;;
  *)
    echo "❌ Unsupported operating system: $OS"
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64)
    ARCH_NAME="x86_64"
    ;;
  arm64|aarch64)
    ARCH_NAME="arm64"
    ;;
  *)
    echo "❌ Unsupported architecture: $ARCH"
    exit 1
    ;;
esac

# Get latest release tag from GitHub
LATEST_TAG=$(curl -sL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_TAG" ]; then
  echo "⚠️ Could not resolve latest release version tag. Falling back to v0.2.8."
  LATEST_TAG="v0.2.8"
fi

VERSION="${LATEST_TAG#v}"
TARBALL="webp-converter-${PLATFORM}-${ARCH_NAME}.tar.gz"
DOWNLOAD_URL="https://github.com/reneboygarcia/webp-converter/releases/download/${LATEST_TAG}/${TARBALL}"

TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

echo "📦 Downloading webp-converter ${LATEST_TAG} for ${PLATFORM}-${ARCH_NAME}..."

if curl -sL "$DOWNLOAD_URL" -o "$TEMP_DIR/$TARBALL"; then
  tar -xzf "$TEMP_DIR/$TARBALL" -C "$TEMP_DIR"
  
  if [ ! -w "$INSTALL_DIR" ]; then
    echo "🔑 Requesting sudo permission to install binary into $INSTALL_DIR..."
    sudo mv "$TEMP_DIR/webp-convert" "$INSTALL_DIR/webp-convert"
    sudo chmod +x "$INSTALL_DIR/webp-convert"
    sudo ln -sf "$INSTALL_DIR/webp-convert" "$INSTALL_DIR/webp-conv"
  else
    mv "$TEMP_DIR/webp-convert" "$INSTALL_DIR/webp-convert"
    chmod +x "$INSTALL_DIR/webp-convert"
    ln -sf "$INSTALL_DIR/webp-convert" "$INSTALL_DIR/webp-conv"
  fi

  echo "✔ Successfully installed webp-converter to $INSTALL_DIR/webp-convert"
  echo "✔ Created command alias 'webp-conv' -> '$INSTALL_DIR/webp-convert'"
  echo ""
  echo "Run 'webp-convert' or 'webp-conv' to start!"
else
  echo "⚠️ Binary release archive not found for ${PLATFORM}-${ARCH_NAME}. Building from source via Homebrew / Cargo..."
  if command -v brew >/dev/null 2>&1; then
    brew install reneboygarcia/homebrew-tap/webp-converter
    echo "✔ Successfully installed webp-converter via Homebrew!"
  elif command -v cargo >/dev/null 2>&1; then
    cargo install --git "https://github.com/reneboygarcia/webp-converter.git" webp-converter
    echo "✔ Successfully installed webp-converter via cargo!"
  else
    echo "❌ Neither Homebrew nor Cargo found. Please install Homebrew or Rust."
    exit 1
  fi
fi

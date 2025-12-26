#!/bin/sh
set -e

# anyform installer
# Usage: curl -fsSL https://raw.githubusercontent.com/epenabella/anyform/main/install.sh | sh

REPO="epenabella/anyform"
BINARY_NAME="anyform"
INSTALL_DIR="/usr/local/bin"

# Detect OS
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Map architecture
case "$ARCH" in
  x86_64) ARCH="amd64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

# Map OS
case "$OS" in
  linux) OS="linux" ;;
  darwin) OS="darwin" ;;
  *) echo "Unsupported OS: $OS"; exit 1 ;;
esac

# Get latest version
echo "Fetching latest version..."
VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep tag_name | cut -d '"' -f 4)

if [ -z "$VERSION" ]; then
  echo "Error: Could not determine latest version"
  exit 1
fi

# Download
BINARY="${BINARY_NAME}-${OS}-${ARCH}"
URL="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY}"
CHECKSUM_URL="https://github.com/${REPO}/releases/download/${VERSION}/checksums.txt"

echo "Downloading ${BINARY_NAME} ${VERSION} for ${OS}/${ARCH}..."
curl -sL "$URL" -o /tmp/${BINARY_NAME}

# Verify checksum
echo "Verifying checksum..."
curl -sL "$CHECKSUM_URL" -o /tmp/checksums.txt
EXPECTED_CHECKSUM=$(grep "${BINARY}" /tmp/checksums.txt | awk '{print $1}')

if [ -n "$EXPECTED_CHECKSUM" ]; then
  if command -v sha256sum > /dev/null 2>&1; then
    ACTUAL_CHECKSUM=$(sha256sum /tmp/${BINARY_NAME} | awk '{print $1}')
  elif command -v shasum > /dev/null 2>&1; then
    ACTUAL_CHECKSUM=$(shasum -a 256 /tmp/${BINARY_NAME} | awk '{print $1}')
  else
    echo "Warning: No sha256sum or shasum found, skipping checksum verification"
    ACTUAL_CHECKSUM="$EXPECTED_CHECKSUM"
  fi

  if [ "$EXPECTED_CHECKSUM" != "$ACTUAL_CHECKSUM" ]; then
    echo "Error: Checksum verification failed"
    echo "Expected: $EXPECTED_CHECKSUM"
    echo "Actual:   $ACTUAL_CHECKSUM"
    rm -f /tmp/${BINARY_NAME} /tmp/checksums.txt
    exit 1
  fi
  echo "Checksum verified."
fi

chmod +x /tmp/${BINARY_NAME}

# Install
echo "Installing to ${INSTALL_DIR}/${BINARY_NAME}..."
if [ -w "$INSTALL_DIR" ]; then
  mv /tmp/${BINARY_NAME} "${INSTALL_DIR}/${BINARY_NAME}"
else
  sudo mv /tmp/${BINARY_NAME} "${INSTALL_DIR}/${BINARY_NAME}"
fi

rm -f /tmp/checksums.txt

echo ""
echo "${BINARY_NAME} ${VERSION} installed successfully!"
echo ""
${BINARY_NAME} --version

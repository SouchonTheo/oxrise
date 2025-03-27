#!/bin/bash

if [ "$(uname)" != "Darwin" ]; then
  echo "This installer is only available for macOS."
  exit 1
fi

BINARY_URL="https://raw.githubusercontent.com/SouchonTheo/oxrise/main/release/oxrise"

TARGET_DIR="/usr/local/bin"
TARGET_BIN="${TARGET_DIR}/oxrise"

echo "Downloading oxrise for macOS..."
curl -sSfL "$BINARY_URL" -o oxrise || {
  echo "Error downloading the binary."
  exit 1
}

chmod +x oxrise

if [ "$(id -u)" -ne 0 ]; then
  echo "Moving the binary to ${TARGET_DIR} (sudo required)..."
  sudo mv oxrise "$TARGET_BIN" || {
    echo "Error moving the binary."
    exit 1
  }
else
  mv oxrise "$TARGET_BIN" || {
    echo "Error moving the binary."
    exit 1
  }
fi

echo "Installation complete. You can now run 'oxrise' from your terminal."


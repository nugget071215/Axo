#!/usr/bin/env bash

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TARGET="/etc/nixos/Axo"

echo "[*] Creating local bin directory..."
mkdir -p "$HOME/.local/bin"

echo "[*] Installing Axo system files..."

if [ "$EUID" -ne 0 ]; then
    sudo -v
fi

sudo mkdir -p /etc/nixos

# copy instead of mv (safer + repeatable)
sudo cp -r "$SCRIPT_DIR" "$TARGET"

echo "[*] Ensuring PATH setup..."
if ! grep -q 'HOME/.local/bin' "$HOME/.bashrc"; then
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$HOME/.bashrc"
fi

echo ""
echo "[✓] Installation complete"
echo ""
echo "Next step:"
echo "  Add this to your configuration.nix:"
echo "  imports = [ /etc/nixos/Axo/axo.nix ];"

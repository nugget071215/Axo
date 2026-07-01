#!/usr/bin/env bash

mkdir -p ~/.local/bin

mv axo ~

sudo mv ~/Axo /etc/nixos

echo 'export PATH="$HOME/.local/bin:$PATH"' >>"$HOME/.bashrc"

echo "Put ./axo/axo.nix in the imports section of your configuration.nix file and rebuild to finish installation."

#!/bin/bash

set -e

echo "Installing slplus via cargo..."
cargo install --git https://github.com/CallMeAlphabet/slplus

BIN_PATH=$(which slplus 2>/dev/null || echo "$HOME/.cargo/bin/slplus")

if [ ! -f "$BIN_PATH" ]; then
    echo "Error: slplus binary not found after cargo install."
    exit 1
fi

TYPOS=("sl" "gti" "gerp" "sudp" "cst" "vom")

echo "Copying binary to /usr/local/bin for typos..."
for typo in "${TYPOS[@]}"; do
    sudo cp "$BIN_PATH" "/usr/local/bin/$typo"
    sudo chmod +x "/usr/local/bin/$typo"
    echo "Installed /usr/local/bin/$typo"
done

echo "Done! Try typing one of the typos: ${TYPOS[*]}"

#!/bin/bash

set -e

echo "Installing fastannoy via cargo..."
cargo install --git https://github.com/CallMeAlphabet/fastannoy

BIN_PATH=$(which fastannoy 2>/dev/null || echo "$HOME/.cargo/bin/fastannoy")

if [ ! -f "$BIN_PATH" ]; then
    echo "Error: fastannoy binary not found after cargo install."
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

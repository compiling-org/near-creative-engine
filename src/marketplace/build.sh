#!/bin/bash

# Build script for Creative Marketplace

echo "Building Creative Marketplace..."

# Check if wasm32-unknown-unknown target is installed
if ! rustup target list | grep -q "wasm32-unknown-unknown (installed)"; then
    echo "Installing wasm32-unknown-unknown target..."
    rustup target add wasm32-unknown-unknown
fi

# Build the contract
echo "Compiling contract..."
cargo build --target wasm32-unknown-unknown --release

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
    echo "WASM file location: target/wasm32-unknown-unknown/release/creative_marketplace.wasm"
    
    # Copy to a more convenient location
    mkdir -p ../../wasm-contracts
    cp target/wasm32-unknown-unknown/release/creative_marketplace.wasm ../../wasm-contracts/creative_marketplace.wasm
    echo "Copied WASM file to ../../wasm-contracts/creative_marketplace.wasm"
else
    echo "Build failed!"
    exit 1
fi
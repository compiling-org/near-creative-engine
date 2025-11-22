#!/bin/bash

# Build script for NEAR WASM contract

echo "Building NEAR WASM contract..."

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
    echo "WASM file location: target/wasm32-unknown-unknown/release/nft_near_wasm.wasm"
    
    # Copy to a more convenient location
    cp target/wasm32-unknown-unknown/release/nft_near_wasm.wasm ../wasm-contracts/nft_near_wasm.wasm
    echo "Copied WASM file to ../wasm-contracts/nft_near_wasm.wasm"
else
    echo "Build failed!"
    exit 1
fi
#!/bin/bash

# Build script for Polkadot client

echo "Building Polkadot client..."

# Build the client
echo "Compiling client..."
cargo build --lib

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi
#!/bin/bash

# Build script for IPFS integration

echo "Building IPFS integration..."

# Build the integration
echo "Compiling integration..."
cargo build --lib

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "Build successful!"
else
    echo "Build failed!"
    exit 1
fi
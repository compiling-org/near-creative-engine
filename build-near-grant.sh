#!/bin/bash
# Build script for NEAR Grant - Fractal Studio & WGSL Studio
# Can be run independently from other grants

echo "============================================"
echo "Building NEAR Grant Components"
echo "Fractal Studio & WGSL Studio"
echo "============================================"

# Build NEAR WASM contracts
echo ""
echo "📦 Building NEAR smart contracts..."
cd src/near-wasm
cargo build --target wasm32-unknown-unknown --release

if [ $? -eq 0 ]; then
    echo "✅ NEAR contracts built successfully"
    echo "📁 Output: target/wasm32-unknown-unknown/release/"
else
    echo "❌ NEAR contract build failed"
    exit 1
fi

# Build core Rust client (dependency)
echo ""
echo "📦 Building core Rust client..."
cd ../rust-client
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Core library built successfully"
else
    echo "⚠️  Core library build failed (optional dependency)"
fi

# Build WASM for browser
echo ""
echo "📦 Building WASM for browser deployment..."
wasm-pack build --target web --out-dir ../../test-website/wasm-near

if [ $? -eq 0 ]; then
    echo "✅ Browser WASM built successfully"
    echo "📁 Output: test-website/wasm-near/"
else
    echo "⚠️  Browser WASM build failed (optional)"
fi

cd ../..

# Copy specific documentation to grant repository
echo ""
echo "📄 Copying specific documentation..."
cp NEAR_SPECIFIC_README.md ../grant-repositories/near-creative-engine/README.md
cp NEAR_SPECIFIC_TECHNICAL_ARCHITECTURE.md ../grant-repositories/near-creative-engine/TECHNICAL_ARCHITECTURE.md  
cp NEAR_SPECIFIC_IMPLEMENTATION_REPORT.md ../grant-repositories/near-creative-engine/IMPLEMENTATION_REPORT.md

echo ""
echo "============================================"
echo "✅ NEAR Grant Build Complete!"
echo "============================================"
echo ""
echo "Deployment files:"
echo "  - Smart contracts: src/near-wasm/target/wasm32-unknown-unknown/release/"
echo "  - Frontend: test-website/index.html (Fractal Studio tab)"
echo ""
echo "To deploy:"
echo "  1. Deploy contracts to NEAR testnet"
echo "  2. Update contract IDs in test-website/blockchain.js"
echo "  3. Serve test-website/ on web server"
echo ""

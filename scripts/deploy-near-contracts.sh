#!/bin/bash

# NEAR Contract Deployment Script
# This script deploys the actual NEAR smart contracts to testnet

echo "🚀 Deploying NEAR Contracts to Testnet..."

# Check if near CLI is installed
if ! command -v near &> /dev/null; then
    echo "❌ NEAR CLI not found. Installing..."
    npm install -g near-cli
fi

# Check if we're logged in
echo "🔑 Checking NEAR login status..."
if ! near state $(whoami).testnet &> /dev/null; then
    echo "❌ Not logged in to NEAR. Please run: near login"
    exit 1
fi

# Build the contracts first
echo "🔨 Building NEAR contracts..."
cd src/near-wasm

# Build the main contract
echo "Building main NFT contract..."
cargo build --target wasm32-unknown-unknown --release

if [ $? -ne 0 ]; then
    echo "❌ Contract build failed"
    exit 1
fi

# Create account for the contract if it doesn't exist
CONTRACT_ACCOUNT="biometric-soulbound-nft.$(whoami).testnet"
echo "📋 Creating contract account: $CONTRACT_ACCOUNT"

# Check if account exists
if ! near state $CONTRACT_ACCOUNT &> /dev/null; then
    echo "Creating new account for contract..."
    near create-account $CONTRACT_ACCOUNT --masterAccount $(whoami).testnet --initialBalance 10
else
    echo "Account already exists"
fi

# Deploy the contract
echo "📤 Deploying contract to $CONTRACT_ACCOUNT..."
near deploy $CONTRACT_ACCOUNT \
    --wasmFile target/wasm32-unknown-unknown/release/near_wasm.wasm \
    --accountId $(whoami).testnet

if [ $? -eq 0 ]; then
    echo "✅ Contract deployed successfully!"
    echo "Contract ID: $CONTRACT_ACCOUNT"
    echo "Explorer: https://explorer.testnet.near.org/accounts/$CONTRACT_ACCOUNT"
    
    # Update the config file with the deployed contract
    echo "📝 Updating configuration..."
    cd ../..
    sed -i "s/biometric-soulbound-nft.kenchen.testnet/$CONTRACT_ACCOUNT/g" src/config/blockchain-config.js
    sed -i "s/biometric-soulbound-nft.kenchen.testnet/$CONTRACT_ACCOUNT/g" src/utils/unified-wallet-connector.js
    
    echo "✅ Configuration updated!"
else
    echo "❌ Contract deployment failed"
    exit 1
fi

echo "🎉 NEAR contract deployment complete!"
echo "Next steps:"
echo "1. Test the contract with: npm run test:near"
echo "2. Check the contract on: https://explorer.testnet.near.org/accounts/$CONTRACT_ACCOUNT"
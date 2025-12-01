#!/bin/bash

# NEAR Soulbound NFT Contract Deployment Script
# This script deploys the biometric soulbound NFT contract to NEAR testnet

set -e

echo "🚀 Deploying NEAR Soulbound NFT Contract..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
CONTRACT_NAME="biometric-soulbound-nft"
ACCOUNT_ID=${NEAR_ACCOUNT_ID:-"kenchen.testnet"}  # Use environment variable or default
INITIAL_OWNER=${NEAR_ACCOUNT_ID:-"kenchen.testnet"}

# Check if near-cli is installed
if ! command -v near &> /dev/null; then
    echo -e "${RED}Error: near-cli is not installed${NC}"
    echo "Please install near-cli: npm install -g near-cli"
    exit 1
fi

# Check if rust is installed
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust is not installed${NC}"
    echo "Please install Rust: https://rustup.rs/"
    exit 1
fi

echo -e "${YELLOW}Building contract...${NC}"
cd contracts/near/soulbound-nft

# Build the contract
echo "Compiling Rust contract..."
cargo build --target wasm32-unknown-unknown --release

# Check if build was successful
if [ ! -f "target/wasm32-unknown-unknown/release/biometric_soulbound_nft.wasm" ]; then
    echo -e "${RED}Error: Contract compilation failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Contract built successfully!${NC}"

# Deploy to testnet
echo -e "${YELLOW}Deploying to NEAR testnet...${NC}"

# Create subaccount for contract (with error handling)
echo "Creating subaccount for contract..."
if ! near create-account $CONTRACT_NAME.$ACCOUNT_ID --masterAccount $ACCOUNT_ID --initialBalance 10; then
    echo -e "${YELLOW}Subaccount may already exist or creation failed, continuing with deployment...${NC}"
fi

# Deploy contract with better error handling
echo "Deploying contract..."
near deploy $CONTRACT_NAME.$ACCOUNT_ID \
    target/wasm32-unknown-unknown/release/biometric_soulbound_nft.wasm \
    --accountId $ACCOUNT_ID \
    --initFunction new \
    --initArgs '{"owner_id": "'$INITIAL_OWNER'", "metadata": {"spec": "nft-1.0.0", "name": "Biometric Soulbound NFT", "symbol": "BSNFT", "icon": null, "base_uri": null, "reference": null, "reference_hash": null}}' \
    --gas 300000000000000

echo -e "${GREEN}✅ Contract deployed successfully!${NC}"
echo -e "${GREEN}Contract ID: $CONTRACT_NAME.$ACCOUNT_ID${NC}"

# Test the contract
echo -e "${YELLOW}Testing contract functionality...${NC}"

# Test minting a soulbound NFT with better error handling
echo "Testing mint_soulbound function..."
if near call $CONTRACT_NAME.$ACCOUNT_ID mint_soulbound \
    '{"emotion_data": {"primary_emotion": "Focused", "confidence": 0.95, "secondary_emotions": [["Calm", 0.85], ["Alert", 0.75]], "arousal": 0.6, "valence": 0.7}, "quality_score": 0.92, "biometric_hash": "a1b2c3d4e5f6"}' \
    --accountId $ACCOUNT_ID \
    --amount 0.1 \
    --gas 300000000000000; then
    echo -e "${GREEN}✅ Mint test successful!${NC}"
else
    echo -e "${RED}❌ Mint test failed, but contract is deployed${NC}"
fi

# Test viewing tokens with error handling
echo "Testing nft_tokens_for_owner function..."
if near view $CONTRACT_NAME.$ACCOUNT_ID nft_tokens_for_owner '{"account_id": "'$ACCOUNT_ID'"}'; then
    echo -e "${GREEN}✅ View test successful!${NC}"
else
    echo -e "${YELLOW}⚠️  View test failed, but contract may still be functional${NC}"
fi

echo -e "${GREEN}✅ Contract testing completed!${NC}"
echo -e "${YELLOW}Next steps:${NC}"
echo "1. Update CONTRACT_IDS in the frontend with: $CONTRACT_NAME.$ACCOUNT_ID"
echo "2. Test with real EEG data and AI inference"
echo "3. Deploy to mainnet when ready"

# Save contract ID for frontend
echo $CONTRACT_NAME.$ACCOUNT_ID > ../../../src/config/contract-ids.txt
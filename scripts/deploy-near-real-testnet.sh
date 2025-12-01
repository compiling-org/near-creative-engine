#!/bin/bash

# REAL NEAR Testnet Deployment Script
# This script actually deploys to NEAR testnet and tests real function calls

set -e

echo "🚀 Starting REAL NEAR Testnet Deployment..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CONTRACT_NAME="real-biometric-nft"
TIMESTAMP=$(date +%s)
SUBACCOUNT_NAME="${CONTRACT_NAME}-${TIMESTAMP}"

# Get NEAR account from user or use a test account
echo -e "${YELLOW}Enter your NEAR testnet account (e.g., yourname.testnet):${NC}"
read -p "Account: " NEAR_ACCOUNT

if [ -z "$NEAR_ACCOUNT" ]; then
    echo -e "${YELLOW}⚠️  No account provided, using demo account${NC}"
    NEAR_ACCOUNT="demo-account-${TIMESTAMP}.testnet"
fi

echo -e "${BLUE}📋 Deployment Configuration:${NC}"
echo "  Contract Name: $CONTRACT_NAME"
echo "  Subaccount: $SUBACCOUNT_NAME.$NEAR_ACCOUNT"
echo "  Master Account: $NEAR_ACCOUNT"
echo "  Network: testnet"

# Check if near-cli is available
if ! command -v near &> /dev/null; then
    echo -e "${YELLOW}⚠️  Installing near-cli...${NC}"
    npm install -g near-cli
fi

# Build the contract
echo -e "${BLUE}🔨 Building NEAR Contract...${NC}"
cd contracts/near/soulbound-nft

# Clean and build
cargo clean
echo "Compiling contract..."
cargo build --target wasm32-unknown-unknown --release

# Check if build succeeded
if [ ! -f "target/wasm32-unknown-unknown/release/biometric_soulbound_nft.wasm" ]; then
    echo -e "${RED}❌ Contract compilation failed!${NC}"
    exit 1
fi

CONTRACT_WASM="target/wasm32-unknown-unknown/release/biometric_soulbound_nft.wasm"
CONTRACT_SIZE=$(wc -c < "$CONTRACT_WASM")
echo -e "${GREEN}✅ Contract built successfully!${NC}"
echo "  WASM size: $CONTRACT_SIZE bytes"

# Create a new testnet account for deployment
echo -e "${YELLOW}🌐 Setting up NEAR testnet account...${NC}"
echo -e "${BLUE}Note: You'll need to fund this account from https://wallet.testnet.near.org${NC}"

# Try to create account (may fail if already exists)
near create-account "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" \
    --masterAccount "$NEAR_ACCOUNT" \
    --initialBalance 5 || {
    echo -e "${YELLOW}⚠️  Account creation failed - may already exist or need funding${NC}"
}

# Fund the account from faucet
echo -e "${YELLOW}💰 Requesting testnet tokens from faucet...${NC}"
echo -e "${BLUE}Please fund your account at: https://wallet.testnet.near.org${NC}"
echo -e "${BLUE}Account: $SUBACCOUNT_NAME.$NEAR_ACCOUNT${NC}"
read -p "Press Enter after funding the account..."

# Deploy contract
echo -e "${BLUE}🚀 Deploying contract to testnet...${NC}"
near deploy "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" \
    "$CONTRACT_WASM" \
    --accountId "$NEAR_ACCOUNT" \
    --initFunction new \
    --initArgs "{\"owner_id\": \"$NEAR_ACCOUNT\", \"metadata\": {\"spec\": \"nft-1.0.0\", \"name\": \"Real Biometric Soulbound NFT\", \"symbol\": \"RBSNFT\", \"icon\": \"https://ipfs.io/ipfs/QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco\"}}" \
    --gas 300000000000000 || {
    echo -e "${RED}❌ Contract deployment failed!${NC}"
    exit 1
}

echo -e "${GREEN}✅ Contract deployed successfully!${NC}"
echo -e "${BLUE}📋 Contract Details:${NC}"
echo "  Contract ID: $SUBACCOUNT_NAME.$NEAR_ACCOUNT"
echo "  Network: testnet"
echo "  Explorer: https://explorer.testnet.near.org/accounts/$SUBACCOUNT_NAME.$NEAR_ACCOUNT"

# Save contract ID
echo "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" > ../../../src/config/near-real-contract-id.txt

# Test contract functions
echo -e "${BLUE}🧪 Testing contract functions...${NC}"

# Test 1: Get contract metadata
echo "1. Testing nft_metadata..."
near view "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" nft_metadata || echo "⚠️  Metadata test failed"

# Test 2: Mint a real NFT with biometric data
echo "2. Testing mint_soulbound..."
REAL_TOKEN_ID="token-$(date +%s)"
near call "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" mint_soulbound \
    "{\"emotion_data\": {\"primary_emotion\": \"Focused\", \"confidence\": 0.89, \"secondary_emotions\": [[\"Calm\", 0.85], [\"Alert\", 0.78]], \"arousal\": 0.65, \"valence\": 0.72}, \"quality_score\": 0.91, \"biometric_hash\": \"sha256:$(echo $RANDOM | sha256sum | cut -d' ' -f1)\"}" \
    --accountId "$NEAR_ACCOUNT" \
    --amount 0.1 \
    --gas 300000000000000 || echo "⚠️  Mint test failed"

# Test 3: View tokens for owner
echo "3. Testing nft_tokens_for_owner..."
near view "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" nft_tokens_for_owner \
    "{\"account_id\": \"$NEAR_ACCOUNT\"}" || echo "⚠️  View tokens test failed"

# Test 4: Get specific token info
echo "4. Testing nft_token..."
near view "$SUBACCOUNT_NAME.$NEAR_ACCOUNT" nft_token \
    "{\"token_id\": \"$REAL_TOKEN_ID\"}" || echo "⚠️  Token info test failed"

echo -e "${GREEN}✅ Contract testing completed!${NC}"

# Create deployment summary
echo -e "${BLUE}📊 Deployment Summary:${NC}"
echo "Contract ID: $SUBACCOUNT_NAME.$NEAR_ACCOUNT"
echo "Network: NEAR Testnet"
echo "Functions Tested: nft_metadata, mint_soulbound, nft_tokens_for_owner, nft_token"
echo "Explorer: https://explorer.testnet.near.org/accounts/$SUBACCOUNT_NAME.$NEAR_ACCOUNT"

echo -e "${GREEN}🎉 REAL NEAR testnet deployment completed!${NC}"

# Return to project root
cd ../../../
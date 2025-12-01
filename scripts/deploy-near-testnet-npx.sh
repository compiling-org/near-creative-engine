#!/bin/bash

# Enhanced NEAR Testnet Deployment Script - NPX VERSION
# This script handles real testnet deployment using npx for near-cli

set -e

echo "🚀 Starting Real NEAR Testnet Deployment (NPX VERSION)..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
CONTRACT_NAME="biometric-soulbound-nft"
TIMESTAMP=$(date +%s)
SUBACCOUNT_NAME="${CONTRACT_NAME}-${TIMESTAMP}"

# Check for required environment variables
if [ -z "$NEAR_ACCOUNT_ID" ]; then
    echo -e "${YELLOW}⚠️  NEAR_ACCOUNT_ID not set, using default test account${NC}"
    export NEAR_ACCOUNT_ID="kenchen.testnet"
fi

if [ -z "$NEAR_TESTNET_PRIVATE_KEY" ]; then
    echo -e "${YELLOW}⚠️  NEAR_TESTNET_PRIVATE_KEY not set, deployment may fail${NC}"
    echo -e "${BLUE}💡 Set your testnet private key: export NEAR_TESTNET_PRIVATE_KEY='your-private-key'${NC}"
fi

echo -e "${BLUE}📋 Deployment Configuration:${NC}"
echo "  Contract Name: $CONTRACT_NAME"
echo "  Subaccount: $SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"
echo "  Master Account: $NEAR_ACCOUNT_ID"
echo "  Network: testnet"

# Check if near-cli is available via npx
echo -e "${BLUE}🔍 Checking near-cli availability...${NC}"
if ! npx near-cli --version &> /dev/null; then
    echo -e "${RED}❌ near-cli not available via npx${NC}"
    exit 1
fi

# SKIP BUILD STEP - Use pre-compiled WASM
echo -e "${BLUE}🔨 Using pre-compiled NEAR Contract...${NC}"
CONTRACT_WASM="contracts/near/soulbound-nft/biometric_soulbound_nft.wasm"

# Verify WASM exists
if [ ! -f "$CONTRACT_WASM" ]; then
    echo -e "${RED}❌ Pre-compiled contract not found!${NC}"
    echo -e "${BLUE}💡 Run the build process first to compile the contract${NC}"
    exit 1
fi

CONTRACT_SIZE=$(wc -c < "$CONTRACT_WASM")
echo -e "${GREEN}✅ Pre-compiled contract found!${NC}"
echo "  WASM size: $CONTRACT_SIZE bytes"

# Check contract size (NEAR has ~4MB limit)
if [ $CONTRACT_SIZE -gt 4000000 ]; then
    echo -e "${RED}❌ Contract too large for NEAR deployment${NC}"
    exit 1
fi

echo -e "${BLUE}🌐 Deploying to NEAR Testnet...${NC}"

# Create subaccount for contract
echo "Creating subaccount: $SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"
npx near-cli create-account "$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID" \
    --masterAccount "$NEAR_ACCOUNT_ID" \
    --initialBalance 10 || {
    echo -e "${YELLOW}⚠️  Subaccount creation failed, may already exist${NC}"
}

# Deploy contract with initialization
echo "Deploying contract to: $SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"
npx near-cli deploy "$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID" \
    "$CONTRACT_WASM" \
    --accountId "$NEAR_ACCOUNT_ID" \
    --initFunction new \
    --initArgs '{
        "owner_id": "'$NEAR_ACCOUNT_ID'",
        "metadata": {
            "spec": "nft-1.0.0",
            "name": "Biometric Soulbound NFT",
            "symbol": "BSNFT",
            "icon": "https://ipfs.io/ipfs/QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco",
            "base_uri": "https://ipfs.io/ipfs/",
            "reference": null,
            "reference_hash": null
        }
    }' \
    --gas 300000000000000 || {
    echo -e "${RED}❌ Contract deployment failed!${NC}"
    exit 1
}

echo -e "${GREEN}✅ Contract deployed successfully!${NC}"
echo -e "${BLUE}📋 Contract Details:${NC}"
echo "  Contract ID: $SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"
echo "  Network: testnet"
echo "  Explorer: https://explorer.testnet.near.org/accounts/$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"

# Save contract ID for frontend configuration
echo "$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID" > src/config/near-contract-id.txt

# Update JavaScript configuration
echo -e "${BLUE}📝 Updating frontend configuration...${NC}"
cat > src/config/deployed-contracts.js << EOF
// Auto-generated deployed contracts configuration
export const DEPLOYED_CONTRACTS = {
  near: {
    testnet: {
      soulboundNFT: '$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID',
      deployedAt: '$(date -u +%Y-%m-%dT%H:%M:%SZ)',
      deployedBy: '$NEAR_ACCOUNT_ID'
    }
  }
};
EOF

echo -e "${GREEN}✅ Configuration updated!${NC}"

# Test contract functionality
echo -e "${BLUE}🧪 Testing contract functionality...${NC}"

# Test 1: Contract metadata
echo "Testing contract metadata..."
npx near-cli view "$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID" nft_metadata || {
    echo -e "${YELLOW}⚠️  Metadata test failed${NC}"
}

# Test 2: Mint a soulbound NFT with realistic biometric data
echo "Testing biometric NFT minting..."
npx near-cli call "$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID" mint_soulbound \
    '{
        "emotion_data": {
            "primary_emotion": "Focused",
            "confidence": 0.92,
            "secondary_emotions": [["Calm", 0.85], ["Alert", 0.78]],
            "arousal": 0.65,
            "valence": 0.72
        },
        "quality_score": 0.89,
        "biometric_hash": "sha256:a1b2c3d4e5f6789012345678901234567890abcdef"
    }' \
    --accountId "$NEAR_ACCOUNT_ID" \
    --amount 0.1 \
    --gas 300000000000000 || {
    echo -e "${YELLOW}⚠️  Mint test failed${NC}"
}

# Test 3: View owner's tokens
echo "Testing token viewing..."
npx near-cli view "$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID" nft_tokens_for_owner \
    '{"account_id": "'$NEAR_ACCOUNT_ID'"}' || {
    echo -e "${YELLOW}⚠️  View test failed${NC}"
}

echo -e "${GREEN}✅ Deployment and testing completed!${NC}"
echo -e "${BLUE}🎯 Next Steps:${NC}"
echo "1. Update your frontend with: $SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"
echo "2. Test with real biometric data from EEG devices"
echo "3. Integrate with AI emotion detection services"
echo "4. Deploy to mainnet when ready"
echo ""
echo -e "${BLUE}🔗 Useful Links:${NC}"
echo "  NEAR Explorer: https://explorer.testnet.near.org/accounts/$SUBACCOUNT_NAME.$NEAR_ACCOUNT_ID"
echo "  NEAR Wallet: https://wallet.testnet.near.org/"
echo "  Contract Code: contracts/near/soulbound-nft/src/lib.rs"
echo ""
echo -e "${GREEN}🎉 Real NEAR testnet deployment complete!${NC}"
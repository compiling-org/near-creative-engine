/**
 * Fixed NEAR Soulbound NFT Contract
 * Addresses deserialization errors with proper NEP-171 implementation
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue};

pub type TokenId = String;
pub type Balance = U128;
pub type Timestamp = u64;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    owner_id: AccountId,
    tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    tokens_by_id: LookupMap<TokenId, Token>,
    token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    metadata: LazyOption<NFTContractMetadata>,
    // Custom fields for biometric authentication
    biometric_data: LookupMap<TokenId, BiometricData>,
    emotion_history: LookupMap<TokenId, Vec<EmotionRecord>>,
}

/// Custom biometric data structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BiometricData {
    pub biometric_hash: String,
    pub emotion_data: EmotionData,
    pub quality_score: f64,
    pub device_id: String,
    pub timestamp: Timestamp,
    pub verification_method: String,
}

/// Emotion data from AI inference
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionData {
    pub primary_emotion: String,
    pub confidence: f64,
    pub secondary_emotions: Vec<(String, f64)>,
    pub arousal: f64,
    pub valence: f64,
}

/// Historical emotion record
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionRecord {
    pub timestamp: Timestamp,
    pub emotion_data: EmotionData,
    pub context: String,
}

/// Standard Token structure for NEP-171
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Token {
    pub owner_id: AccountId,
}

/// Structure for token metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub media: Option<String>,
    pub media_hash: Option<Base64VecU8>,
    pub copies: Option<u64>,
    pub issued_at: Option<u64>,
    pub expires_at: Option<u64>,
    pub starts_at: Option<u64>,
    pub updated_at: Option<u64>,
    pub extra: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

/// Metadata for the contract itself
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,
    pub name: String,
    pub symbol: String,
    pub icon: Option<String>,
    pub base_uri: Option<String>,
    pub reference: Option<String>,
    pub reference_hash: Option<Base64VecU8>,
}

/// Helper structure for JSON serialization
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
    pub biometric_data: Option<BiometricData>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        Self {
            owner_id: owner_id.clone(),
            tokens_per_owner: LookupMap::new(b"o".to_vec()),
            tokens_by_id: LookupMap::new(b"t".to_vec()),
            token_metadata_by_id: UnorderedMap::new(b"m".to_vec()),
            metadata: LazyOption::new(b"c".to_vec(), Some(&metadata)),
            biometric_data: LookupMap::new(b"b".to_vec()),
            emotion_history: LookupMap::new(b"e".to_vec()),
        }
    }

    #[init]
    #[private]
    pub fn new_default_meta(owner_id: AccountId) -> Self {
        Self::new(
            owner_id,
            NFTContractMetadata {
                spec: NFT_METADATA_SPEC.to_string(),
                name: "Biometric Soulbound NFT".to_string(),
                symbol: "BSNFT".to_string(),
                icon: None,
                base_uri: None,
                reference: None,
                reference_hash: None,
            },
        )
    }

    /// Mint a soulbound NFT with biometric authentication
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        emotion_data: EmotionData,
        quality_score: f64,
        biometric_hash: String,
    ) -> Token {
        // Validate biometric quality
        assert!(quality_score >= 0.7, "Biometric quality too low: {}", quality_score);
        
        // Create biometric data
        let biometric_data = BiometricData {
            biometric_hash: biometric_hash.clone(),
            emotion_data: emotion_data.clone(),
            quality_score,
            device_id: "emotiv_epoc_x".to_string(),
            timestamp: env::block_timestamp(),
            verification_method: "AI-Enhanced".to_string(),
        };
        
        // Create emotion history record
        let emotion_record = EmotionRecord {
            timestamp: env::block_timestamp(),
            emotion_data: emotion_data.clone(),
            context: "Minting".to_string(),
        };
        
        // Create token metadata
        let metadata = TokenMetadata {
            title: Some(format!("Biometric Soulbound Token #{}", token_id)),
            description: Some(format!(
                "AI-verified biometric authentication token. Primary emotion: {} (confidence: {:.2}%)",
                emotion_data.primary_emotion,
                emotion_data.confidence * 100.0
            )),
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: Some(env::block_timestamp()),
            expires_at: None,
            starts_at: Some(env::block_timestamp()),
            updated_at: Some(env::block_timestamp()),
            extra: Some(format!("biometric_hash:{}", biometric_hash)),
            reference: None,
            reference_hash: None,
        };
        
        // Mint the token
        let token = self.internal_mint(token_id.clone(), receiver_id.clone(), Some(metadata));
        
        // Store biometric data
        self.biometric_data.insert(&token_id, &biometric_data);
        
        // Store emotion history
        self.emotion_history.insert(&token_id, &vec![emotion_record]);
        
        // Emit mint event
        env::log_str(&format!(
            "Soulbound NFT minted: {} for {} with emotion: {} (confidence: {:.2})",
            token_id,
            receiver_id,
            emotion_data.primary_emotion,
            emotion_data.confidence
        ));
        
        token
    }

    /// Verify biometric data against stored token
    pub fn verify_biometric(&self, token_id: TokenId, biometric_hash: String) -> bool {
        let biometric_data = self.biometric_data.get(&token_id)
            .expect("Token not found");
        
        biometric_data.biometric_hash == biometric_hash
    }

    /// Get emotion history for a token
    pub fn get_emotion_history(&self, token_id: TokenId) -> Vec<EmotionRecord> {
        self.emotion_history.get(&token_id)
            .unwrap_or_default()
    }

    /// Get biometric data for a token
    pub fn get_biometric_data(&self, token_id: TokenId) -> BiometricData {
        self.biometric_data.get(&token_id)
            .expect("Token not found")
    }

    // NEP-171 compliance: Override transfer to make tokens soulbound (non-transferable)
    pub fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        env::panic_str("Soulbound tokens are non-transferable");
    }

    // NEP-171 compliance: Override transfer call to make tokens soulbound (non-transferable)
    pub fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        env::panic_str("Soulbound tokens are non-transferable");
    }

    // NEP-171 view methods
    pub fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }

    pub fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        let token = self.tokens_by_id.get(&token_id)?;
        let metadata = self.token_metadata_by_id.get(&token_id)?;
        let biometric_data = self.biometric_data.get(&token_id);
        
        Some(JsonToken {
            token_id,
            owner_id: token.owner_id,
            metadata,
            biometric_data,
        })
    }

    pub fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<JsonToken> {
        let tokens_set = self.tokens_per_owner.get(&account_id).unwrap_or_else(|| {
            UnorderedSet::new(
                format!("o{}", account_id).as_bytes().to_vec()
            )
        });
        
        let limit = limit.unwrap_or(100);
        let from_index = from_index.map(|u| u.0).unwrap_or(0);
        
        tokens_set.iter()
            .skip(from_index as usize)
            .take(limit as usize)
            .filter_map(|token_id| self.nft_token(token_id))
            .collect()
    }

    // Helper methods for internal minting
    fn internal_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        metadata: Option<TokenMetadata>,
    ) -> Token {
        // Check if token already exists
        assert!(!self.tokens_by_id.contains_key(&token_id), "Token already exists");
        
        // Create token
        let token = Token { owner_id: receiver_id.clone() };
        
        // Insert token
        self.tokens_by_id.insert(&token_id, &token);
        
        // Add token to owner's set
        self.internal_add_token_to_owner(&receiver_id, &token_id);
        
        // Add metadata if provided
        if let Some(metadata) = metadata {
            self.token_metadata_by_id.insert(&token_id, &metadata);
        }
        
        token
    }

    fn internal_add_token_to_owner(&mut self, owner_id: &AccountId, token_id: &TokenId) {
        let mut tokens_set = self.tokens_per_owner.get(owner_id).unwrap_or_else(|| {
            UnorderedSet::new(
                format!("o{}", owner_id).as_bytes().to_vec()
            )
        });
        
        tokens_set.insert(token_id);
        self.tokens_per_owner.insert(owner_id, &tokens_set);
    }

    /// Frontend-compatible mint function that matches the expected signature
    #[payable]
    pub fn mint_soulbound(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        emotion_data: EmotionData,
        quality_score: f64,
        biometric_hash: String,
    ) -> Token {
        // Delegate to the main nft_mint function
        self.nft_mint(token_id, receiver_id, emotion_data, quality_score, biometric_hash)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor_account_id: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor_account_id);
        builder
    }

    #[test]
    fn test_new() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let contract = Contract::new_default_meta(accounts(0));
        assert_eq!(contract.nft_metadata().name, "Biometric Soulbound NFT");
    }

    #[test]
    fn test_mint_soulbound() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0));

        let emotion_data = EmotionData {
            primary_emotion: "Happy".to_string(),
            confidence: 0.85,
            secondary_emotions: vec![("Excited".to_string(), 0.7)],
            arousal: 0.6,
            valence: 0.8,
        };

        let token = contract.nft_mint(
            "token1".to_string(),
            accounts(0),
            emotion_data,
            0.8,
            "hash123".to_string(),
        );

        assert_eq!(token.owner_id, accounts(0));
        
        // Verify token exists
        let json_token = contract.nft_token("token1".to_string()).unwrap();
        assert_eq!(json_token.token_id, "token1");
        assert_eq!(json_token.owner_id, accounts(0));
    }

    #[test]
    #[should_panic(expected = "Biometric quality too low")]
    fn test_mint_low_quality() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0));

        let emotion_data = EmotionData {
            primary_emotion: "Happy".to_string(),
            confidence: 0.85,
            secondary_emotions: vec![],
            arousal: 0.6,
            valence: 0.8,
        };

        contract.nft_mint(
            "token1".to_string(),
            accounts(0),
            emotion_data,
            0.5, // Low quality score
            "hash123".to_string(),
        );
    }

    #[test]
    fn test_soulbound_transfer_blocked() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        let mut contract = Contract::new_default_meta(accounts(0));

        let emotion_data = EmotionData {
            primary_emotion: "Happy".to_string(),
            confidence: 0.85,
            secondary_emotions: vec![],
            arousal: 0.6,
            valence: 0.8,
        };

        contract.nft_mint(
            "token1".to_string(),
            accounts(0),
            emotion_data,
            0.8,
            "hash123".to_string(),
        );

        // Attempt transfer should panic
        testing_env!(context.predecessor_account_id(accounts(0)).build());
        contract.nft_transfer(accounts(1), "token1".to_string(), None, None);
    }
}
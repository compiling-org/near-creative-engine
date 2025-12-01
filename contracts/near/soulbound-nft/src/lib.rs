/**
 * NEAR Soulbound NFT Contract for Biometric Authentication
 * Based on NEP-171 (NFT) with soulbound (non-transferable) functionality
 * Integrates with AI/ML emotion detection and biometric verification
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U128, U64};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, PromiseOrValue};
mod metadata;

/// This spec can be treated like a version of the standard.
pub const NFT_METADATA_SPEC: &str = "nft-1.0.0";
/// This is the name of the NFT standard we're using
pub const NFT_STANDARD_NAME: &str = "nep171";

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct BiometricSoulboundNFT {
    pub owner_id: AccountId,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub tokens_by_id: LookupMap<TokenId, Token>,
    pub token_metadata_by_id: UnorderedMap<TokenId, TokenMetadata>,
    pub metadata: LazyOption<NFTContractMetadata>,
    // Custom fields for biometric authentication
    pub biometric_data: LookupMap<TokenId, BiometricData>,
    pub emotion_history: LookupMap<TokenId, Vec<EmotionRecord>>,
}

/// Note that token IDs for NFTs are strings on NEAR
pub type TokenId = String;
/// Timestamp in nanoseconds
pub type Timestamp = u64;

/// Custom biometric data structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BiometricData {
    pub biometric_hash: String,      // Hash of biometric features
    pub emotion_data: EmotionData,   // AI-detected emotion data
    pub quality_score: f64,          // Signal quality (0.0 - 1.0)
    pub device_id: String,           // EEG device identifier
    pub timestamp: Timestamp,          // When biometric was captured
    pub verification_method: String, // "AI-Enhanced", "Manual", etc.
}

/// Emotion data from AI inference
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionData {
    pub primary_emotion: String,     // "Happy", "Sad", "Focused", etc.
    pub confidence: f64,             // AI confidence (0.0 - 1.0)
    pub secondary_emotions: Vec<(String, f64)>, // Other emotions with scores
    pub arousal: f64,                // Arousal level (-1.0 to 1.0)
    pub valence: f64,                // Valence level (-1.0 to 1.0)
}

/// Historical emotion record
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionRecord {
    pub timestamp: Timestamp,
    pub emotion_data: EmotionData,
    pub context: String, // "Minting", "Verification", "Transfer Attempt", etc.
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
    pub title: Option<String>, // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
    pub description: Option<String>, // free-form description
    pub media: Option<String>, // URL to associated media, preferably to decentralized, content-addressed storage
    pub media_hash: Option<Base64VecU8>, // base64-encoded sha256 hash of content referenced by the `media` field
    pub copies: Option<u64>, // number of copies of this set of metadata in existence when token was minted.
    pub issued_at: Option<u64>, // When token was issued or minted, Unix epoch in milliseconds
    pub expires_at: Option<u64>, // When token expires, Unix epoch in milliseconds
    pub starts_at: Option<u64>, // When token starts being valid, Unix epoch in milliseconds
    pub updated_at: Option<u64>, // When token was last updated, Unix epoch in milliseconds
    pub extra: Option<String>, // anything extra the NFT wants to store on-chain
    pub reference: Option<String>, // URL to an off-chain JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // base64-encoded sha256 hash of JSON from reference field
}

/// Implementation of Token struct
impl Token {
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
        }
    }
}

#[near_bindgen]
impl BiometricSoulboundNFT {
    #[init]
    #[private]
    pub fn new(owner_id: AccountId, metadata: NFTContractMetadata) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        
        let this = Self {
            owner_id: owner_id.clone(),
            tokens_per_owner: LookupMap::new(b"o".to_vec()),
            tokens_by_id: LookupMap::new(b"t".to_vec()),
            token_metadata_by_id: UnorderedMap::new(b"m".to_vec()),
            metadata: LazyOption::new(b"c".to_vec(), Some(&metadata)),
            biometric_data: LookupMap::new(b"b".to_vec()),
            emotion_history: LookupMap::new(b"e".to_vec()),
        };
        
        this
    }

    #[payable]
    pub fn mint_soulbound(
        &mut self,
        emotion_data: EmotionData,
        quality_score: f64,
        biometric_hash: String,
    ) -> Token {
        let token_id = format!("biometric_{}_{}", env::signer_account_id(), env::block_timestamp());
        
        // Validate biometric quality
        assert!(quality_score >= 0.7, "Biometric quality too low: {}", quality_score);
        
        let owner_id = env::signer_account_id();
        
        // Create biometric data
        let biometric_data = BiometricData {
            biometric_hash: biometric_hash.clone(),
            emotion_data: emotion_data.clone(),
            quality_score,
            device_id: "emotiv_epoc_x".to_string(), // Will be passed as parameter in real implementation
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
        let token = self.internal_mint(token_id.clone(), owner_id.clone(), Some(metadata));
        
        // Store biometric data
        self.biometric_data.insert(&token_id, &biometric_data);
        
        // Store emotion history
        self.emotion_history.insert(&token_id, &vec![emotion_record]);
        
        // Emit mint event
        env::log_str(&format!(
            "Soulbound NFT minted: {} for {} with emotion: {} (confidence: {:.2})",
            token_id,
            owner_id,
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

    /// Override transfer to make tokens soulbound (non-transferable)
    #[payable]
    pub fn nft_transfer(
        &mut self,
        _receiver_id: AccountId,
        _token_id: TokenId,
        _approval_id: Option<u64>,
        _memo: Option<String>,
    ) {
        env::panic_str("Soulbound tokens are non-transferable");
    }

    /// Override transfer call to make tokens soulbound (non-transferable)
    #[payable]
    pub fn nft_transfer_call(
        &mut self,
        _receiver_id: AccountId,
        _token_id: TokenId,
        _approval_id: Option<u64>,
        _memo: Option<String>,
        _msg: String,
    ) -> PromiseOrValue<bool> {
        env::panic_str("Soulbound tokens are non-transferable");
    }

    // Helper methods for internal minting
    fn internal_mint(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        metadata: Option<TokenMetadata>,
    ) -> Token {
        // Check if token already exists
        assert!(!self.tokens_by_id.contains_key(&token_id), "Token already exists");
        
        // Create token
        let token = Token::new(owner_id.clone());
        
        // Insert token
        self.tokens_by_id.insert(&token_id, &token);
        
        // Add token to owner's set
        self.internal_add_token_to_owner(&owner_id, &token_id);
        
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

    // View methods
    pub fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }

    pub fn nft_token(&self, token_id: TokenId) -> Option<JsonToken> {
        let token = self.tokens_by_id.get(&token_id)?;
        let metadata = self.token_metadata_by_id.get(&token_id)?;
        let biometric_data = self.biometric_data.get(&token_id)?;
        
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
}

/// Helper structure for JSON serialization
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
    pub biometric_data: BiometricData,
}

/// Metadata for the contract itself
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTContractMetadata {
    pub spec: String,              // required, essentially a version like "nft-1.0.0"
    pub name: String,              // required, ex. "Mochi Rising - Digital Edition" or "Metaverse 3"
    pub symbol: String,            // required, ex. "MOCHI"
    pub icon: Option<String>,      // Data URL
    pub base_uri: Option<String>, // Central gateway for your assets
    pub reference: Option<String>, // URL to a JSON file with more info
    pub reference_hash: Option<Base64VecU8>, // Base64-encoded sha256 hash of JSON from reference field
}

impl Default for NFTContractMetadata {
    fn default() -> Self {
        Self {
            spec: NFT_METADATA_SPEC.to_string(),
            name: "Biometric Soulbound NFT".to_string(),
            symbol: "BSNFT".to_string(),
            icon: None,
            base_uri: None,
            reference: None,
            reference_hash: None,
        }
    }
}

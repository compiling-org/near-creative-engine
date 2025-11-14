//! Soulbound token functionality for creative identity

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;

/// Soulbound token representing creative identity
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SoulboundToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
    pub identity_data: IdentityData,
    pub minted_at: Timestamp,
    pub soulbound: bool,
}

/// Identity data for soulbound tokens
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct IdentityData {
    pub creative_profile: CreativeProfile,
    pub achievements: Vec<String>,
    pub verified: bool,
    pub reputation_score: f32,
}

/// Creative profile information
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CreativeProfile {
    pub primary_skill: String,
    pub experience_level: String,
    pub preferred_medium: String,
    pub collaboration_interest: bool,
}

impl SoulboundToken {
    /// Create a new soulbound token
    pub fn new(
        token_id: TokenId,
        owner_id: AccountId,
        metadata: TokenMetadata,
        identity_data: IdentityData,
    ) -> Self {
        Self {
            token_id,
            owner_id,
            metadata,
            identity_data,
            minted_at: env::block_timestamp(),
            soulbound: true,
        }
    }
}

impl Default for IdentityData {
    fn default() -> Self {
        Self {
            creative_profile: CreativeProfile::default(),
            achievements: vec![],
            verified: false,
            reputation_score: 0.0,
        }
    }
}

impl Default for CreativeProfile {
    fn default() -> Self {
        Self {
            primary_skill: "generalist".to_string(),
            experience_level: "beginner".to_string(),
            preferred_medium: "digital".to_string(),
            collaboration_interest: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_contract_standards::non_fungible_token::TokenId;

    #[test]
    fn test_soulbound_token_creation() {
        let token_id: TokenId = "soulbound1".to_string();
        let owner_id: AccountId = "creator.testnet".parse().unwrap();
        
        let metadata = TokenMetadata {
            title: Some("Creative Identity".to_string()),
            description: Some("Soulbound token for creative identity".to_string()),
            media: None,
            media_hash: None,
            copies: None,
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        
        let identity_data = IdentityData::default();
        
        let soulbound_token = SoulboundToken::new(
            token_id.clone(),
            owner_id.clone(),
            metadata.clone(),
            identity_data.clone(),
        );
        
        assert_eq!(soulbound_token.token_id, token_id);
        assert_eq!(soulbound_token.owner_id, owner_id);
        assert_eq!(soulbound_token.metadata.title, Some("Creative Identity".to_string()));
        assert!(soulbound_token.soulbound);
    }

    #[test]
    fn test_identity_data_default() {
        let identity = IdentityData::default();
        assert!(!identity.verified);
        assert_eq!(identity.reputation_score, 0.0);
        assert!(identity.achievements.is_empty());
    }

    #[test]
    fn test_creative_profile_default() {
        let profile = CreativeProfile::default();
        assert_eq!(profile.primary_skill, "generalist");
        assert_eq!(profile.experience_level, "beginner");
        assert_eq!(profile.preferred_medium, "digital");
    }
}
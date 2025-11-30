//! Enhanced Soulbound token with AI/ML integration patterns from NEAR SDK and biometric data

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp, Balance};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::U128;

/// Enhanced soulbound token with AI/ML biometric integration
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EnhancedSoulboundToken {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: TokenMetadata,
    pub identity_data: EnhancedIdentityData,
    pub minted_at: Timestamp,
    pub soulbound: bool,
    pub biometric_hash: Option<Vec<u8>>,
    pub ai_model_version: String,
}

/// Enhanced identity data with biometric and AI components
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EnhancedIdentityData {
    pub creative_profile: CreativeProfile,
    pub achievements: Vec<String>,
    pub verified: bool,
    pub reputation_score: f32,
    pub biometric_data: BiometricData,
    pub ai_insights: AIInsights,
    pub collaboration_history: Vec<CollaborationRecord>,
}

/// Biometric data for enhanced identity verification
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BiometricData {
    pub eeg_fingerprint: Option<Vec<f32>>,  // Brain wave patterns
    pub emotional_signature: Option<Vec<f32>>,  // Emotional AI signature
    pub creative_patterns: Option<Vec<f32>>,  // Creative process patterns
    pub last_updated: Timestamp,
}

/// AI-generated insights about the creator
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIInsights {
    pub creativity_score: f32,
    pub collaboration_compatibility: f32,
    pub skill_recommendations: Vec<String>,
    pub predicted_success_rate: f32,
    pub personality_traits: Vec<String>,
}

/// Collaboration history record
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CollaborationRecord {
    pub partner_id: AccountId,
    pub project_id: String,
    pub success_rating: f32,
    pub timestamp: Timestamp,
    pub skills_contributed: Vec<String>,
}

/// Creative profile information
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CreativeProfile {
    pub primary_skill: String,
    pub experience_level: String,
    pub preferred_medium: String,
    pub collaboration_interest: bool,
    pub skill_tags: Vec<String>,
    pub hourly_rate: Option<Balance>,
}

/// Enhanced soulbound token contract state
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EnhancedSoulboundContract {
    pub tokens: LookupMap<TokenId, EnhancedSoulboundToken>,
    pub owner_to_tokens: LookupMap<AccountId, Vector<TokenId>>,
    pub biometric_registry: LookupMap<AccountId, BiometricData>,
    pub ai_model_registry: LookupMap<String, Vec<u8>>, // Model hash to model data
    pub total_supply: u64,
}

impl EnhancedSoulboundToken {
    /// Create a new enhanced soulbound token with AI/ML integration
    pub fn new(
        token_id: TokenId,
        owner_id: AccountId,
        metadata: TokenMetadata,
        identity_data: EnhancedIdentityData,
        biometric_hash: Option<Vec<u8>>,
        ai_model_version: String,
    ) -> Self {
        Self {
            token_id,
            owner_id,
            metadata,
            identity_data,
            minted_at: env::block_timestamp(),
            soulbound: true,
            biometric_hash,
            ai_model_version,
        }
    }

    /// Update biometric data with privacy preservation
    pub fn update_biometric_data(&mut self, new_biometric_data: BiometricData) {
        require!(self.soulbound, "Cannot update biometric data for non-soulbound token");
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can update biometric data");
        
        self.identity_data.biometric_data = new_biometric_data;
        
        // Generate new biometric hash for verification
        self.biometric_hash = Some(env::sha256(
            format!("{}{}", self.token_id, env::block_timestamp()).as_bytes()
        ));
    }

    /// Add AI insights from external computation
    pub fn add_ai_insights(&mut self, insights: AIInsights) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can update AI insights");
        self.identity_data.ai_insights = insights;
    }

    /// Record collaboration and update reputation
    pub fn record_collaboration(&mut self, record: CollaborationRecord) {
        require!(env::predecessor_account_id() == self.owner_id, "Only owner can record collaborations");
        
        self.identity_data.collaboration_history.push(record.clone());
        
        // Update reputation score based on collaboration success
        let avg_rating = self.identity_data.collaboration_history.iter()
            .map(|r| r.success_rating)
            .sum::<f32>() / self.identity_data.collaboration_history.len() as f32;
        
        self.identity_data.reputation_score = (self.identity_data.reputation_score * 0.8) + (avg_rating * 0.2);
    }

    /// Verify biometric match (privacy-preserving)
    pub fn verify_biometric(&self, biometric_sample: &[f32]) -> bool {
        if let Some(ref stored_hash) = self.biometric_hash {
            let sample_hash = env::sha256(
                format!("{}{}", self.token_id, biometric_sample.len()).as_bytes()
            );
            stored_hash == &sample_hash
        } else {
            false
        }
    }

    /// Get AI-powered skill recommendations
    pub fn get_skill_recommendations(&self) -> Vec<String> {
        self.identity_data.ai_insights.skill_recommendations.clone()
    }

    /// Calculate collaboration compatibility score
    pub fn calculate_compatibility(&self, other_skills: &[String]) -> f32 {
        let common_skills = self.identity_data.creative_profile.skill_tags.iter()
            .filter(|skill| other_skills.contains(skill))
            .count();
        
        if other_skills.is_empty() {
            0.0
        } else {
            (common_skills as f32 / other_skills.len() as f32) * 
            self.identity_data.ai_insights.collaboration_compatibility
        }
    }
}

impl Default for EnhancedIdentityData {
    fn default() -> Self {
        Self {
            creative_profile: CreativeProfile::default(),
            achievements: vec![],
            verified: false,
            reputation_score: 0.0,
            biometric_data: BiometricData::default(),
            ai_insights: AIInsights::default(),
            collaboration_history: vec![],
        }
    }
}

impl Default for BiometricData {
    fn default() -> Self {
        Self {
            eeg_fingerprint: None,
            emotional_signature: None,
            creative_patterns: None,
            last_updated: env::block_timestamp(),
        }
    }
}

impl Default for AIInsights {
    fn default() -> Self {
        Self {
            creativity_score: 0.5,
            collaboration_compatibility: 0.5,
            skill_recommendations: vec![],
            predicted_success_rate: 0.5,
            personality_traits: vec![],
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
            skill_tags: vec!["creative".to_string(), "beginner".to_string()],
            hourly_rate: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_contract_standards::non_fungible_token::TokenId;

    #[test]
    fn test_enhanced_soulbound_token_creation() {
        let token_id: TokenId = "enhanced_soulbound1".to_string();
        let owner_id: AccountId = "creator.testnet".parse().unwrap();
        
        let metadata = TokenMetadata {
            title: Some("Enhanced Creative Identity".to_string()),
            description: Some("AI-enhanced soulbound token with biometric integration".to_string()),
            media: None,
            media_hash: None,
            copies: Some(1),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: Some("AI_MODEL_VERSION: v1.0".to_string()),
            reference: None,
            reference_hash: None,
        };
        
        let identity_data = EnhancedIdentityData::default();
        let biometric_hash = Some(vec![1, 2, 3, 4, 5]);
        let ai_model_version = "v1.0".to_string();
        
        let enhanced_token = EnhancedSoulboundToken::new(
            token_id.clone(),
            owner_id.clone(),
            metadata.clone(),
            identity_data.clone(),
            biometric_hash.clone(),
            ai_model_version.clone(),
        );
        
        assert_eq!(enhanced_token.token_id, token_id);
        assert_eq!(enhanced_token.owner_id, owner_id);
        assert_eq!(enhanced_token.metadata.title, Some("Enhanced Creative Identity".to_string()));
        assert!(enhanced_token.soulbound);
        assert_eq!(enhanced_token.ai_model_version, ai_model_version);
        assert_eq!(enhanced_token.biometric_hash, biometric_hash);
    }

    #[test]
    fn test_collaboration_compatibility_calculation() {
        let token_id: TokenId = "test_token".to_string();
        let owner_id: AccountId = "creator.testnet".parse().unwrap();
        
        let metadata = TokenMetadata {
            title: Some("Test Token".to_string()),
            description: Some("Test token".to_string()),
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
        
        let mut identity_data = EnhancedIdentityData::default();
        identity_data.creative_profile.skill_tags = vec![
            "rust".to_string(),
            "blockchain".to_string(),
            "ai".to_string(),
        ];
        identity_data.ai_insights.collaboration_compatibility = 0.8;
        
        let enhanced_token = EnhancedSoulboundToken::new(
            token_id,
            owner_id,
            metadata,
            identity_data,
            None,
            "v1.0".to_string(),
        );
        
        let other_skills = vec!["rust".to_string(), "python".to_string()];
        let compatibility = enhanced_token.calculate_compatibility(&other_skills);
        
        // Should be (1/2) * 0.8 = 0.4
        assert_eq!(compatibility, 0.4);
    }
}
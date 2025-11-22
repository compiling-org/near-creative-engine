//! # NEAR WASM Creative Engine
//!
//! WASM-compiled creative engine for NEAR blockchain integration.
//! Provides interactive NFT functionality and Mintbase-compatible contracts.
//! Enhanced with cross-chain bridge capabilities and advanced emotional computing.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near, AccountId, Promise, Timestamp};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_contract_standards::non_fungible_token::core::NonFungibleTokenCore;
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApproval;
use near_sdk::PromiseOrValue;

pub use crate::emotional::*;
pub use crate::interactive::*;
pub use crate::mintbase::*;
pub use crate::soulbound::*;
pub use crate::wgsl_studio::*;
pub use crate::enhanced_soulbound::*;

mod emotional;
mod interactive;
mod mintbase;
mod soulbound;
mod fractal_studio;
mod wgsl_studio;
mod enhanced_soulbound;

/// Main interactive NFT contract
#[near(contract_state)]
pub struct InteractiveNftContract {
    // NEAR NFT standard implementation
    tokens: NonFungibleToken,
    
    // Owner account
    owner_id: AccountId,
    
    // Token metadata storage
    token_metadata: UnorderedMap<TokenId, TokenMetadata>,
    
    // Interaction history tracking
    interaction_history: LookupMap<TokenId, Vec<InteractionEvent>>,
    
    // Emotional state tracking
    emotional_states: LookupMap<TokenId, EmotionalData>,
    
    // Interactive state tracking
    interactive_states: LookupMap<TokenId, InteractiveState>,
    
    // Soulbound token tracking
    soulbound_tokens: LookupMap<TokenId, SoulboundToken>,
    
    // Mintbase integration
    mintbase_integration: MintbaseIntegration,
    
    // Cross-chain bridge tracking
    cross_chain_tokens: LookupMap<TokenId, CrossChainInfo>,
    
    // Token reputation scores
    token_reputations: LookupMap<TokenId, f32>,
    
    // Advanced token analytics
    token_analytics: LookupMap<TokenId, TokenAnalytics>,
}

// Cross-chain information structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CrossChainInfo {
    pub chain_id: String,
    pub target_contract: String,
    pub bridge_status: String, // "pending", "bridged", "failed"
    pub bridge_timestamp: Timestamp,
    pub emotional_metadata: Option<EmotionalData>, // Include emotional data for cross-chain
}

// Token analytics for advanced tracking
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenAnalytics {
    pub creation_timestamp: Timestamp,
    pub total_interactions: u32,
    pub avg_interaction_intensity: f32,
    pub emotional_complexity: f32,
    pub evolution_progress: f32,
    pub community_engagement_score: f32,
}

#[near]
impl InteractiveNftContract {
    /// Initialize the contract with an owner
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            tokens: NonFungibleToken::new(
                b"t".to_vec(),
                owner_id.clone(),
                Some(b"o".to_vec()),
                Some(b"e".to_vec()),
                Some(b"s".to_vec()),
            ),
            owner_id,
            token_metadata: UnorderedMap::new(b"m".to_vec()),
            interaction_history: LookupMap::new(b"h".to_vec()),
            emotional_states: LookupMap::new(b"e".to_vec()),
            interactive_states: LookupMap::new(b"s".to_vec()),
            soulbound_tokens: LookupMap::new(b"b".to_vec()),
            mintbase_integration: MintbaseIntegration::new(),
            cross_chain_tokens: LookupMap::new(b"c".to_vec()),
            token_reputations: LookupMap::new(b"r".to_vec()),
            token_analytics: LookupMap::new(b"a".to_vec()),
        }
    }

    /// Mint a new interactive NFT
    #[payable]
    pub fn mint_interactive_nft(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        initial_emotion: EmotionalData,
    ) -> Token {
        // Mint the NFT using standard NFT functionality
        let token = self.tokens.internal_mint(token_id.clone(), env::predecessor_account_id(), Some(metadata.clone()));
        
        // Store the metadata
        self.token_metadata.insert(&token_id, &metadata);
        
        // Initialize emotional state
        self.emotional_states.insert(&token_id, &initial_emotion);
        
        // Initialize interactive state
        self.interactive_states.insert(&token_id, &InteractiveState::default());
        
        // Initialize interaction history
        self.interaction_history.insert(&token_id, &vec![]);
        
        // Initialize reputation score
        self.token_reputations.insert(&token_id, &0.5); // Default neutral reputation
        
        // Initialize token analytics
        self.token_analytics.insert(&token_id, &TokenAnalytics {
            creation_timestamp: env::block_timestamp(),
            total_interactions: 0,
            avg_interaction_intensity: 0.0,
            emotional_complexity: initial_emotion.emotional_complexity,
            evolution_progress: 0.0,
            community_engagement_score: 0.0,
        });
        
        token
    }

    /// Record a user interaction with an NFT
    pub fn record_interaction(
        &mut self,
        token_id: TokenId,
        event_type: String,
        data: near_sdk::serde_json::Value,
        intensity: f32,
    ) {
        // Create interaction event
        let interaction = InteractionEvent {
            event_type,
            timestamp: env::block_timestamp(),
            user_id: env::predecessor_account_id(),
            data: data.to_string(), // Convert Value to String
            intensity,
            emotional_impact: None, // In a real implementation, this would be calculated
        };

        // Update interaction history
        let mut history = self.interaction_history.get(&token_id).unwrap_or_else(|| vec![]);
        history.push(interaction.clone());
        self.interaction_history.insert(&token_id, &history);

        // Update interactive state
        let mut state = self.interactive_states.get(&token_id).unwrap_or_else(|| InteractiveState::default());
        state.interaction_streak += 1;
        state.last_activity = env::block_timestamp();
        state.creativity_index = (state.creativity_index + intensity) / 2.0;
        
        // Update interaction patterns
        state.update_interaction_patterns(&interaction);
        
        // Update community engagement
        state.update_community_engagement(&interaction.user_id);
        
        // Update interaction history summary
        state.update_interaction_history_summary(&history);
        
        // Adapt behavior
        state.adapt_behavior(&history);
        
        self.interactive_states.insert(&token_id, &state);

        // Update emotional state based on interaction
        let mut emotion = self.emotional_states.get(&token_id).unwrap_or_else(|| EmotionalData::new());
        emotion.timestamp = env::block_timestamp();
        emotion.valence = (emotion.valence + intensity - 0.5) / 2.0;
        emotion.arousal = (emotion.arousal + intensity) / 2.0;
        
        // Add to emotional trajectory
        emotion.add_to_trajectory(EmotionalVector {
            valence: emotion.valence,
            arousal: emotion.arousal,
            dominance: emotion.dominance,
            timestamp: env::block_timestamp(),
        });
        
        // Predict next emotion
        emotion.predict_next_emotion();
        
        self.emotional_states.insert(&token_id, &emotion);
        
        // Update reputation based on interaction quality
        let mut reputation = self.token_reputations.get(&token_id).unwrap_or(0.5);
        // Simple reputation update - positive interactions increase reputation
        if intensity > 0.5 {
            reputation = (reputation + 0.1).min(1.0);
        } else {
            reputation = (reputation - 0.05).max(0.0);
        }
        self.token_reputations.insert(&token_id, &reputation);
        
        // Update token analytics
        if let Some(mut analytics) = self.token_analytics.get(&token_id) {
            analytics.total_interactions += 1;
            analytics.avg_interaction_intensity = (analytics.avg_interaction_intensity * (analytics.total_interactions - 1) as f32 + intensity) / analytics.total_interactions as f32;
            analytics.emotional_complexity = emotion.emotional_complexity;
            
            // Update evolution progress based on interaction streak
            analytics.evolution_progress = (state.interaction_streak as f32 / 100.0).min(1.0);
            
            // Update community engagement score
            analytics.community_engagement_score = state.community_engagement.community_score;
            
            self.token_analytics.insert(&token_id, &analytics);
        }
    }

    /// Get the current emotional state of an NFT
    pub fn get_emotional_state(&self, token_id: TokenId) -> Option<EmotionalData> {
        self.emotional_states.get(&token_id)
    }

    /// Get the current interactive state of an NFT
    pub fn get_interactive_state(&self, token_id: TokenId) -> Option<InteractiveState> {
        self.interactive_states.get(&token_id)
    }

    /// Get interaction history for an NFT
    pub fn get_interaction_history(&self, token_id: TokenId) -> Option<Vec<InteractionEvent>> {
        self.interaction_history.get(&token_id)
    }

    /// Mint a soulbound token
    pub fn mint_soulbound_token(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
        identity_data: IdentityData,
    ) -> Token {
        // Mint the NFT
        let token = self.tokens.internal_mint(token_id.clone(), env::predecessor_account_id(), Some(metadata.clone()));
        
        // Create soulbound token
        let soulbound_token = SoulboundToken {
            token_id: token_id.clone(),
            owner_id: env::predecessor_account_id(),
            metadata,
            identity_data,
            minted_at: env::block_timestamp(),
            soulbound: true,
        };
        
        // Store soulbound token
        self.soulbound_tokens.insert(&token_id, &soulbound_token);
        
        token
    }

    /// Get soulbound token information
    pub fn get_soulbound_token(&self, token_id: TokenId) -> Option<SoulboundToken> {
        self.soulbound_tokens.get(&token_id)
    }

    /// Update Mintbase integration
    pub fn update_mintbase_integration(&mut self, config: MintbaseConfig) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can update Mintbase integration");
        self.mintbase_integration.update_config(config);
    }
    
    /// Register a token for cross-chain bridging
    pub fn register_cross_chain_token(
        &mut self,
        token_id: TokenId,
        chain_id: String,
        target_contract: String,
    ) {
        // Include emotional metadata for cross-chain transfer
        let emotional_metadata = self.emotional_states.get(&token_id);
        
        let cross_chain_info = CrossChainInfo {
            chain_id,
            target_contract,
            bridge_status: "pending".to_string(),
            bridge_timestamp: env::block_timestamp(),
            emotional_metadata,
        };
        
        self.cross_chain_tokens.insert(&token_id, &cross_chain_info);
    }
    
    /// Update cross-chain bridge status
    pub fn update_bridge_status(
        &mut self,
        token_id: TokenId,
        status: String,
    ) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can update bridge status");
        
        if let Some(mut info) = self.cross_chain_tokens.get(&token_id) {
            info.bridge_status = status;
            info.bridge_timestamp = env::block_timestamp();
            self.cross_chain_tokens.insert(&token_id, &info);
        }
    }
    
    /// Get cross-chain information for a token
    pub fn get_cross_chain_info(&self, token_id: TokenId) -> Option<CrossChainInfo> {
        self.cross_chain_tokens.get(&token_id)
    }
    
    /// Get token reputation score
    pub fn get_token_reputation(&self, token_id: TokenId) -> Option<f32> {
        self.token_reputations.get(&token_id)
    }
    
    /// Get token analytics
    pub fn get_token_analytics(&self, token_id: TokenId) -> Option<TokenAnalytics> {
        self.token_analytics.get(&token_id)
    }
    
    /// Get top interacted tokens
    pub fn get_top_interacted_tokens(&self, limit: u32) -> Vec<(TokenId, u32)> {
        let mut token_interactions: Vec<(TokenId, u32)> = self.interactive_states
            .keys_as_vector()
            .iter()
            .map(|token_id| {
                let state = self.interactive_states.get(&token_id).unwrap_or_default();
                (token_id, state.interaction_streak)
            })
            .collect();
        
        // Sort by interaction count
        token_interactions.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Limit results
        token_interactions.truncate(limit as usize);
        
        token_interactions
    }
    
    /// Get trending tokens based on community engagement
    pub fn get_trending_tokens(&self, limit: u32) -> Vec<(TokenId, f32)> {
        let mut trending_tokens: Vec<(TokenId, f32)> = self.interactive_states
            .keys_as_vector()
            .iter()
            .filter_map(|token_id| {
                let state = self.interactive_states.get(&token_id).unwrap_or_default();
                if state.community_engagement.trending {
                    Some((token_id, state.community_engagement.community_score))
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by community score
        trending_tokens.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Limit results
        trending_tokens.truncate(limit as usize);
        
        trending_tokens
    }
    
    /// Predict next emotional state for a token
    pub fn predict_next_emotion(&self, token_id: TokenId) -> Option<EmotionalVector> {
        if let Some(emotion) = self.emotional_states.get(&token_id) {
            let mut emotion_clone = emotion;
            Some(emotion_clone.predict_next_emotion())
        } else {
            None
        }
    }
}

// Implement NEAR NFT standard methods
impl NonFungibleTokenCore for InteractiveNftContract {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
        // For soulbound tokens, prevent transfer
        if let Some(soulbound) = self.soulbound_tokens.get(&token_id) {
            if soulbound.soulbound {
                env::panic_str("Cannot transfer soulbound tokens");
            }
        }
        self.tokens.nft_transfer(receiver_id, token_id, approval_id, memo)
    }

    fn nft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<bool> {
        // For soulbound tokens, prevent transfer
        if let Some(soulbound) = self.soulbound_tokens.get(&token_id) {
            if soulbound.soulbound {
                env::panic_str("Cannot transfer soulbound tokens");
            }
        }
        self.tokens.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg).into()
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }
}

impl NonFungibleTokenEnumeration for InteractiveNftContract {
    fn nft_total_supply(&self) -> U128 {
        self.tokens.nft_total_supply()
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        self.tokens.nft_tokens(from_index, limit)
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.tokens.nft_supply_for_owner(account_id)
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        self.tokens.nft_tokens_for_owner(account_id, from_index, limit)
    }
}

impl NonFungibleTokenApproval for InteractiveNftContract {
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise> {
        // For soulbound tokens, prevent approval
        if let Some(soulbound) = self.soulbound_tokens.get(&token_id) {
            if soulbound.soulbound {
                env::panic_str("Cannot approve soulbound tokens");
            }
        }
        self.tokens.nft_approve(token_id, account_id, msg)
    }

    fn nft_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
        self.tokens.nft_revoke(token_id, account_id)
    }

    fn nft_revoke_all(&mut self, token_id: TokenId) {
        self.tokens.nft_revoke_all(token_id)
    }

    fn nft_is_approved(
        &self,
        token_id: TokenId,
        approved_account_id: AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        self.tokens.nft_is_approved(token_id, approved_account_id, approval_id)
    }
}

// Default implementation for contract initialization
impl Default for InteractiveNftContract {
    fn default() -> Self {
        Self::new(env::current_account_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("contract.testnet".parse().unwrap());
        builder.signer_account_id("user.testnet".parse().unwrap());
        builder.predecessor_account_id("user.testnet".parse().unwrap());
        builder
    }

    #[test]
    fn test_new_contract() {
        let context = get_context().build();
        testing_env!(context);
        
        let contract = InteractiveNftContract::new("owner.testnet".parse().unwrap());
        assert_eq!(contract.nft_total_supply(), U128(0));
    }

    #[test]
    fn test_mint_interactive_nft() {
        let mut context = get_context();
        context.predecessor_account_id("user.testnet".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = InteractiveNftContract::new("owner.testnet".parse().unwrap());
        
        let metadata = TokenMetadata {
            title: Some("Test NFT".to_string()),
            description: Some("A test interactive NFT".to_string()),
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
        
        let emotion = EmotionalData::new();
        
        let token = contract.mint_interactive_nft(
            "token1".to_string(),
            metadata,
            emotion,
        );
        
        assert_eq!(token.token_id, "token1");
        assert_eq!(token.owner_id, "user.testnet".parse().unwrap());
    }
    
    #[test]
    fn test_cross_chain_registration() {
        let mut context = get_context();
        context.predecessor_account_id("user.testnet".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = InteractiveNftContract::new("owner.testnet".parse().unwrap());
        
        contract.register_cross_chain_token(
            "token1".to_string(),
            "solana".to_string(),
            "contract.solana".to_string(),
        );
        
        let cross_chain_info = contract.get_cross_chain_info("token1".to_string());
        assert!(cross_chain_info.is_some());
        assert_eq!(cross_chain_info.unwrap().chain_id, "solana");
    }
    
    #[test]
    fn test_record_interaction() {
        let mut context = get_context();
        context.predecessor_account_id("user.testnet".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = InteractiveNftContract::new("owner.testnet".parse().unwrap());
        
        let metadata = TokenMetadata {
            title: Some("Test NFT".to_string()),
            description: Some("A test interactive NFT".to_string()),
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
        
        let emotion = EmotionalData::new();
        
        contract.mint_interactive_nft(
            "token1".to_string(),
            metadata,
            emotion,
        );
        
        contract.record_interaction(
            "token1".to_string(),
            "view".to_string(),
            near_sdk::serde_json::json!({"page": "gallery"}),
            0.8,
        );
        
        let interaction_history = contract.get_interaction_history("token1".to_string());
        assert!(interaction_history.is_some());
        assert_eq!(interaction_history.unwrap().len(), 1);
    }
}
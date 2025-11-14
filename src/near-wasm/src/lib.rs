//! # NEAR WASM Creative Engine
//!
//! WASM-compiled creative engine for NEAR blockchain integration.
//! Provides interactive NFT functionality and Mintbase-compatible contracts.

use near_sdk::collections::{LookupMap, UnorderedMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near, AccountId, Promise};
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

mod emotional;
mod interactive;
mod mintbase;
mod soulbound;
mod fractal_studio;
mod wgsl_studio;

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
        };

        // Update interaction history
        let mut history = self.interaction_history.get(&token_id).unwrap_or_else(|| vec![]);
        history.push(interaction);
        self.interaction_history.insert(&token_id, &history);

        // Update interactive state
        let mut state = self.interactive_states.get(&token_id).unwrap_or_else(|| InteractiveState::default());
        state.interaction_streak += 1;
        state.last_activity = env::block_timestamp();
        state.creativity_index = (state.creativity_index + intensity) / 2.0;
        self.interactive_states.insert(&token_id, &state);

        // Update emotional state based on interaction
        let mut emotion = self.emotional_states.get(&token_id).unwrap_or_else(|| EmotionalData::new());
        emotion.timestamp = env::block_timestamp();
        emotion.valence = (emotion.valence + intensity - 0.5) / 2.0;
        emotion.arousal = (emotion.arousal + intensity) / 2.0;
        self.emotional_states.insert(&token_id, &emotion);
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
}
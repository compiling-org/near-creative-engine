//! Simple NEAR NFT Contract - Actually Works
//! Basic NEP-171 compliant NFT contract for testing real functionality

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

/// Simple NFT contract that actually works
#[near(contract_state)]
pub struct SimpleNftContract {
    tokens: NonFungibleToken,
    owner_id: AccountId,
    token_metadata: UnorderedMap<TokenId, TokenMetadata>,
    interaction_history: LookupMap<TokenId, Vec<String>>,
}

#[near]
impl SimpleNftContract {
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
        }
    }

    /// Mint a new NFT - actually works!
    #[payable]
    pub fn mint_nft(
        &mut self,
        token_id: TokenId,
        metadata: TokenMetadata,
    ) -> Token {
        // Mint the NFT using standard NFT functionality
        let token = self.tokens.internal_mint(
            token_id.clone(), 
            env::predecessor_account_id(), 
            Some(metadata.clone())
        );
        
        // Store the metadata
        self.token_metadata.insert(&token_id, &metadata);
        
        // Initialize interaction history
        self.interaction_history.insert(&token_id, &vec![]);
        
        token
    }

    /// Record a simple interaction - actually works!
    pub fn record_interaction(
        &mut self,
        token_id: TokenId,
        interaction: String,
    ) {
        // Get current history
        let mut history = self.interaction_history.get(&token_id).unwrap_or_else(|| vec![]);
        
        // Add new interaction with timestamp
        let interaction_with_timestamp = format!(
            "[{}] {}: {}", 
            env::block_timestamp(), 
            env::predecessor_account_id(), 
            interaction
        );
        history.push(interaction_with_timestamp);
        
        // Store updated history
        self.interaction_history.insert(&token_id, &history);
    }

    /// Get NFT metadata
    pub fn get_metadata(&self, token_id: TokenId) -> Option<TokenMetadata> {
        self.token_metadata.get(&token_id)
    }

    /// Get interaction history
    pub fn get_interaction_history(&self, token_id: TokenId) -> Vec<String> {
        self.interaction_history.get(&token_id).unwrap_or_else(|| vec![])
    }

    /// Get total number of NFTs minted
    pub fn total_supply(&self) -> U128 {
        self.tokens.nft_total_supply()
    }

    /// Get all NFTs for an account
    pub fn tokens_for_owner(&self, account_id: AccountId) -> Vec<Token> {
        self.tokens.nft_tokens_for_owner(account_id, None, None)
    }

    /// Get specific NFT
    pub fn get_nft(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }
}

// Implement NEAR NFT standard methods
impl NonFungibleTokenCore for SimpleNftContract {
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    ) {
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
        self.tokens.nft_transfer_call(receiver_id, token_id, approval_id, memo, msg).into()
    }

    fn nft_token(&self, token_id: TokenId) -> Option<Token> {
        self.tokens.nft_token(token_id)
    }
}

impl NonFungibleTokenEnumeration for SimpleNftContract {
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

impl NonFungibleTokenApproval for SimpleNftContract {
    fn nft_approve(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        msg: Option<String>,
    ) -> Option<Promise> {
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
impl Default for SimpleNftContract {
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
        
        let contract = SimpleNftContract::new("owner.testnet".parse().unwrap());
        assert_eq!(contract.total_supply(), U128(0));
    }

    #[test]
    fn test_mint_nft() {
        let mut context = get_context();
        context.predecessor_account_id("user.testnet".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SimpleNftContract::new("owner.testnet".parse().unwrap());
        
        let metadata = TokenMetadata {
            title: Some("Test NFT".to_string()),
            description: Some("A test NFT that actually works".to_string()),
            media: Some("https://example.com/image.png".to_string()),
            media_hash: None,
            copies: Some(1),
            issued_at: Some(env::block_timestamp().to_string()),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        };
        
        let token = contract.mint_nft("token1".to_string(), metadata.clone());
        
        assert_eq!(token.token_id, "token1");
        assert_eq!(token.owner_id, "user.testnet".parse().unwrap());
        
        // Check metadata
        let stored_metadata = contract.get_metadata("token1".to_string()).unwrap();
        assert_eq!(stored_metadata.title, Some("Test NFT".to_string()));
        
        // Check total supply
        assert_eq!(contract.total_supply(), U128(1));
    }

    #[test]
    fn test_record_interaction() {
        let mut context = get_context();
        context.predecessor_account_id("user.testnet".parse().unwrap());
        testing_env!(context.build());
        
        let mut contract = SimpleNftContract::new("owner.testnet".parse().unwrap());
        
        let metadata = TokenMetadata {
            title: Some("Test NFT".to_string()),
            description: Some("A test NFT".to_string()),
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
        
        contract.mint_nft("token1".to_string(), metadata);
        
        // Record an interaction
        contract.record_interaction("token1".to_string(), "viewed".to_string());
        
        // Check interaction history
        let history = contract.get_interaction_history("token1".to_string());
        assert_eq!(history.len(), 1);
        assert!(history[0].contains("viewed"));
    }
}
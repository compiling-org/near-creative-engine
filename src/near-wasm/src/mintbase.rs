//! Mintbase-compatible NFT functionality

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};

/// Mintbase-compatible NFT contract structure
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MintbaseIntegration {
    pub minters: UnorderedMap<AccountId, bool>,
    pub owner_id: AccountId,
    pub treasury_id: AccountId,
    pub minting_fee: u128,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MintingData {
    pub token_id: String,
    pub metadata: near_contract_standards::non_fungible_token::metadata::TokenMetadata,
    pub receiver_id: AccountId,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MintbaseConfig {
    pub treasury_id: AccountId,
    pub minting_fee: u128,
}

impl MintbaseIntegration {
    /// Create a new Mintbase integration
    pub fn new() -> Self {
        Self {
            minters: UnorderedMap::new(b"mi".to_vec()),
            owner_id: env::current_account_id(),
            treasury_id: env::current_account_id(),
            minting_fee: 0,
        }
    }

    /// Update configuration
    pub fn update_config(&mut self, config: MintbaseConfig) {
        self.treasury_id = config.treasury_id;
        self.minting_fee = config.minting_fee;
    }

    /// Add a minter
    pub fn add_minter(&mut self, account_id: &AccountId) {
        self.minters.insert(account_id, &true);
    }

    /// Remove a minter
    pub fn remove_minter(&mut self, account_id: &AccountId) {
        self.minters.remove(account_id);
    }

    /// Check if account is a minter
    pub fn is_minter(&self, account_id: &AccountId) -> bool {
        self.minters.get(account_id).unwrap_or(false)
    }
}

impl Default for MintbaseIntegration {
    fn default() -> Self {
        Self::new()
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
    fn test_mintbase_integration_creation() {
        let context = get_context().build();
        testing_env!(context);
        
        let integration = MintbaseIntegration::new();
        assert_eq!(integration.minting_fee, 0);
    }

    #[test]
    fn test_mintbase_config_update() {
        let context = get_context().build();
        testing_env!(context);
        
        let mut integration = MintbaseIntegration::new();
        let config = MintbaseConfig {
            treasury_id: "treasury.testnet".parse().unwrap(),
            minting_fee: 1000,
        };
        
        integration.update_config(config);
        assert_eq!(integration.minting_fee, 1000);
    }

    #[test]
    fn test_minter_management() {
        let context = get_context().build();
        testing_env!(context);
        
        let mut integration = MintbaseIntegration::new();
        let account_id: AccountId = "minter.testnet".parse().unwrap();
        
        integration.add_minter(&account_id);
        assert!(integration.is_minter(&account_id));
        
        integration.remove_minter(&account_id);
        assert!(!integration.is_minter(&account_id));
    }
}

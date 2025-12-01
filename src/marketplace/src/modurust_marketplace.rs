//! MODURUST Marketplace Integration
//! 
//! Marketplace features for modular tools, patches, and ownership NFTs

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp};
use near_contract_standards::fungible_token::Balance;
use near_contract_standards::non_fungible_token::TokenId;

/// MODURUST tool ownership NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ModurustToolNFT {
    pub token_id: TokenId,
    pub tool_id: String,
    pub tool_name: String,
    pub version: String,
    pub creator: AccountId,
    pub owner: AccountId,
    pub created_at: Timestamp,
    pub tool_type: ToolType,
    pub ipfs_cid: String,
    pub usage_stats: UsageStats,
    pub license: LicenseType,
}

/// Type of modular tool
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ToolType {
    ShaderModule,
    AudioProcessor,
    VisualEffect,
    DataTransform,
    ControlInterface,
    CustomModule,
}

/// License type for tool
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum LicenseType {
    MIT,
    Apache2,
    GPL3,
    Commercial,
    Custom(String),
}

/// Usage statistics for a tool
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UsageStats {
    pub total_uses: u64,
    pub unique_users: u32,
    pub patches_created: u32,
    pub avg_rating: f32,
    pub total_ratings: u32,
}

/// MODURUST patch NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ModurustPatchNFT {
    pub token_id: TokenId,
    pub patch_id: String,
    pub patch_name: String,
    pub creator: AccountId,
    pub created_at: Timestamp,
    pub tools_used: Vec<String>,
    pub ipfs_cid: String,
    pub complexity_score: u32,
}

/// Tool subscription model
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ToolSubscription {
    pub tool_id: String,
    pub subscriber: AccountId,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub price_per_month: Balance,
    pub auto_renew: bool,
}

/// Tool marketplace listing with royalties
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ToolListing {
    pub listing_id: u64,
    pub tool_nft: ModurustToolNFT,
    pub price: Balance,
    pub royalty_percentage: u32, // Basis points (e.g., 500 = 5%)
    pub creator_royalty: AccountId,
    pub subscription_available: bool,
    pub subscription_price: Option<Balance>,
}

impl ModurustToolNFT {
    /// Create a new tool ownership NFT
    pub fn new(
        token_id: TokenId,
        tool_id: String,
        tool_name: String,
        version: String,
        tool_type: ToolType,
        ipfs_cid: String,
        license: LicenseType,
    ) -> Self {
        let creator = env::predecessor_account_id();
        Self {
            token_id,
            tool_id,
            tool_name,
            version,
            creator: creator.clone(),
            owner: creator,
            created_at: env::block_timestamp(),
            tool_type,
            ipfs_cid,
            usage_stats: UsageStats {
                total_uses: 0,
                unique_users: 0,
                patches_created: 0,
                avg_rating: 0.0,
                total_ratings: 0,
            },
            license,
        }
    }

    /// Update usage statistics
    pub fn record_usage(&mut self, _user: &AccountId) {
        self.usage_stats.total_uses += 1;
        // In real implementation, would track unique users properly
    }

    /// Add rating
    pub fn add_rating(&mut self, rating: f32) {
        let total = self.usage_stats.avg_rating * self.usage_stats.total_ratings as f32;
        self.usage_stats.total_ratings += 1;
        self.usage_stats.avg_rating = (total + rating) / self.usage_stats.total_ratings as f32;
    }

    /// Calculate tool value score
    pub fn value_score(&self) -> u32 {
        let mut score = 0u32;
        
        // Usage indicates popularity
        score += (self.usage_stats.total_uses / 10) as u32;
        
        // Unique users
        score += self.usage_stats.unique_users * 10;
        
        // Patches created shows utility
        score += self.usage_stats.patches_created * 5;
        
        // Rating quality
        score += (self.usage_stats.avg_rating * 20.0) as u32;
        
        score
    }
}

impl ModurustPatchNFT {
    /// Create a new patch NFT
    pub fn new(
        token_id: TokenId,
        patch_id: String,
        patch_name: String,
        tools_used: Vec<String>,
        ipfs_cid: String,
    ) -> Self {
        let complexity = tools_used.len() as u32 * 10; // Simple complexity metric
        Self {
            token_id,
            patch_id,
            patch_name,
            creator: env::predecessor_account_id(),
            created_at: env::block_timestamp(),
            tools_used,
            ipfs_cid,
            complexity_score: complexity,
        }
    }
}

impl ToolSubscription {
    /// Create a new subscription
    pub fn new(
        tool_id: String,
        duration_months: u64,
        price_per_month: Balance,
    ) -> Self {
        let now = env::block_timestamp();
        let month_ns = 30 * 24 * 60 * 60 * 1_000_000_000u64; // 30 days in nanoseconds
        Self {
            tool_id,
            subscriber: env::predecessor_account_id(),
            start_time: now,
            end_time: now + (duration_months * month_ns),
            price_per_month,
            auto_renew: false,
        }
    }

    /// Check if subscription is active
    pub fn is_active(&self) -> bool {
        env::block_timestamp() < self.end_time
    }

    /// Calculate remaining time in days
    pub fn days_remaining(&self) -> u64 {
        if !self.is_active() {
            return 0;
        }
        let remaining_ns = self.end_time - env::block_timestamp();
        remaining_ns / (24 * 60 * 60 * 1_000_000_000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_nft_creation() {
        let tool = ModurustToolNFT::new(
            "tool_001".to_string(),
            "tool_id_123".to_string(),
            "Fractal Generator".to_string(),
            "1.0.0".to_string(),
            ToolType::ShaderModule,
            "QmXXXXX".to_string(),
            LicenseType::MIT,
        );

        assert_eq!(tool.tool_name, "Fractal Generator");
        assert_eq!(tool.usage_stats.total_uses, 0);
    }

    #[test]
    fn test_rating_system() {
        let mut tool = ModurustToolNFT::new(
            "tool_001".to_string(),
            "tool_id".to_string(),
            "Test Tool".to_string(),
            "1.0.0".to_string(),
            ToolType::CustomModule,
            "QmXXX".to_string(),
            LicenseType::MIT,
        );

        tool.add_rating(4.5);
        tool.add_rating(5.0);
        tool.add_rating(4.0);

        assert_eq!(tool.usage_stats.total_ratings, 3);
        assert!((tool.usage_stats.avg_rating - 4.5).abs() < 0.1);
    }
}

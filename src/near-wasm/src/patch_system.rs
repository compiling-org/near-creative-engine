//! Patch publication and management system

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise, Timestamp};
use std::collections::HashMap;

/// Published creative patch
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PublishedPatch {
    pub id: String,
    pub title: String,
    pub description: String,
    pub author: AccountId,
    pub tool_type: String,
    pub version: String,
    pub tags: Vec<String>,
    pub ipfs_cid: String,
    pub license: String,
    pub price: Option<Balance>,
    pub downloads: u64,
    pub rating: f32,
    pub total_ratings: u32,
    pub published_at: Timestamp,
    pub last_updated: Timestamp,
    pub fork_count: u32,
    pub dependencies: Vec<String>, // Other patch IDs
    pub compatibility: Vec<String>, // Compatible tool versions
}

/// Patch rating/review
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PatchRating {
    pub user: AccountId,
    pub rating: u8, // 1-5 stars
    pub review: Option<String>,
    pub timestamp: Timestamp,
}

/// Fork relationship
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PatchFork {
    pub original_patch_id: String,
    pub fork_patch_id: String,
    pub forked_by: AccountId,
    pub forked_at: Timestamp,
    pub changes_summary: String,
}

/// Patch collection/series
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PatchCollection {
    pub id: String,
    pub title: String,
    pub description: String,
    pub curator: AccountId,
    pub patches: Vec<String>, // Patch IDs
    pub theme: String,
    pub created_at: Timestamp,
    pub featured: bool,
}

/// Patch marketplace contract
#[derive(BorshDeserialize, BorshSerialize)]
pub struct PatchMarketplaceContract {
    pub published_patches: UnorderedMap<String, PublishedPatch>,
    pub patch_ratings: LookupMap<String, Vec<PatchRating>>, // patch_id -> ratings
    pub user_purchases: LookupMap<AccountId, UnorderedSet<String>>, // user -> purchased patch IDs
    pub patch_forks: LookupMap<String, Vec<PatchFork>>, // original_patch_id -> forks
    pub collections: UnorderedMap<String, PatchCollection>,
    pub user_patches: LookupMap<AccountId, Vec<String>>, // author -> their patch IDs
    pub featured_patches: UnorderedSet<String>,
    pub treasury_id: AccountId,
    pub platform_fee: u8, // Percentage (0-100)
}

impl Default for PatchMarketplaceContract {
    fn default() -> Self {
        Self {
            published_patches: UnorderedMap::new(b"p"),
            patch_ratings: LookupMap::new(b"r"),
            user_purchases: LookupMap::new(b"up"),
            patch_forks: LookupMap::new(b"f"),
            collections: UnorderedMap::new(b"c"),
            user_patches: LookupMap::new(b"u"),
            featured_patches: UnorderedSet::new(b"fp"),
            treasury_id: env::predecessor_account_id(),
            platform_fee: 5, // 5% platform fee
        }
    }
}

#[near_bindgen]
impl PatchMarketplaceContract {
    #[init]
    pub fn new(treasury_id: AccountId, platform_fee: Option<u8>) -> Self {
        Self {
            published_patches: UnorderedMap::new(b"p"),
            patch_ratings: LookupMap::new(b"r"),
            user_purchases: LookupMap::new(b"up"),
            patch_forks: LookupMap::new(b"f"),
            collections: UnorderedMap::new(b"c"),
            user_patches: LookupMap::new(b"u"),
            featured_patches: UnorderedSet::new(b"fp"),
            treasury_id,
            platform_fee: platform_fee.unwrap_or(5),
        }
    }

    /// Publish a new patch to the marketplace
    #[payable]
    pub fn publish_patch(&mut self, patch: PublishedPatch) -> String {
        let author = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        // Validate patch data
        assert_eq!(patch.author, author, "Patch author must match caller");
        assert!(patch.rating >= 0.0 && patch.rating <= 5.0, "Invalid rating");
        assert!(!patch.title.is_empty(), "Title cannot be empty");
        assert!(!patch.ipfs_cid.is_empty(), "IPFS CID cannot be empty");

        // Check if patch ID already exists
        assert!(self.published_patches.get(&patch.id).is_none(), "Patch ID already exists");

        // Minimum deposit for publishing (0.1 NEAR)
        let min_deposit = 100_000_000_000_000_000_000_000; // 0.1 NEAR
        assert!(deposit >= min_deposit, "Minimum deposit: 0.1 NEAR for publishing");

        // Set publication timestamp
        let mut published_patch = patch.clone();
        published_patch.published_at = env::block_timestamp();
        published_patch.last_updated = env::block_timestamp();

        // Store the patch
        self.published_patches.insert(&patch.id, &published_patch);

        // Update user's patches
        let mut user_patches = self.user_patches.get(&author).unwrap_or_default();
        user_patches.push(patch.id.clone());
        self.user_patches.insert(&author, &user_patches);

        // Initialize empty ratings
        self.patch_ratings.insert(&patch.id, &Vec::new());

        // Initialize empty forks
        self.patch_forks.insert(&patch.id, &Vec::new());

        // Transfer deposit to treasury
        Promise::new(self.treasury_id.clone()).transfer(deposit);

        patch.id
    }

    /// Update an existing patch
    pub fn update_patch(&mut self, patch_id: String, updates: near_sdk::serde_json::Value) {
        let author = env::predecessor_account_id();

        if let Some(mut patch) = self.published_patches.get(&patch_id) {
            // Verify ownership
            assert_eq!(patch.author, author, "Only patch author can update");

            // Apply updates (simplified - in practice would parse specific fields)
            if let Some(title) = updates.get("title").and_then(|v| v.as_str()) {
                patch.title = title.to_string();
            }
            if let Some(description) = updates.get("description").and_then(|v| v.as_str()) {
                patch.description = description.to_string();
            }
            if let Some(version) = updates.get("version").and_then(|v| v.as_str()) {
                patch.version = version.to_string();
            }
            if let Some(ipfs_cid) = updates.get("ipfs_cid").and_then(|v| v.as_str()) {
                patch.ipfs_cid = ipfs_cid.to_string();
            }

            patch.last_updated = env::block_timestamp();
            self.published_patches.insert(&patch_id, &patch);
        } else {
            env::panic_str("Patch not found");
        }
    }

    /// Purchase a patch
    #[payable]
    pub fn purchase_patch(&mut self, patch_id: String) {
        let buyer = env::predecessor_account_id();
        let deposit = env::attached_deposit();

        if let Some(patch) = self.published_patches.get(&patch_id) {
            if let Some(price) = patch.price {
                assert!(deposit >= price, "Insufficient payment");

                // Calculate platform fee
                let platform_fee = (price * self.platform_fee as u128) / 100;
                let author_payment = price - platform_fee;

                // Transfer payments
                Promise::new(self.treasury_id.clone()).transfer(platform_fee);
                Promise::new(patch.author.clone()).transfer(author_payment);

                // Record purchase
                let mut user_purchases = self.user_purchases.get(&buyer).unwrap_or_else(|| UnorderedSet::new(b"usp"));
                user_purchases.insert(&patch_id);
                self.user_purchases.insert(&buyer, &user_purchases);

                // Update download count
                let mut updated_patch = patch;
                updated_patch.downloads += 1;
                self.published_patches.insert(&patch_id, &updated_patch);
            } else {
                env::panic_str("Patch is not for sale");
            }
        } else {
            env::panic_str("Patch not found");
        }
    }

    /// Rate a patch
    pub fn rate_patch(&mut self, patch_id: String, rating: u8, review: Option<String>) {
        let rater = env::predecessor_account_id();

        assert!(rating >= 1 && rating <= 5, "Rating must be between 1 and 5");

        if let Some(mut patch) = self.published_patches.get(&patch_id) {
            let mut ratings = self.patch_ratings.get(&patch_id).unwrap_or_default();

            // Remove existing rating from this user
            ratings.retain(|r| r.user != rater);

            // Add new rating
            let new_rating = PatchRating {
                user: rater,
                rating,
                review,
                timestamp: env::block_timestamp(),
            };
            ratings.push(new_rating);

            // Update patch rating
            let total_rating: u32 = ratings.iter().map(|r| r.rating as u32).sum();
            patch.rating = total_rating as f32 / ratings.len() as f32;
            patch.total_ratings = ratings.len() as u32;

            // Save updates
            self.patch_ratings.insert(&patch_id, &ratings);
            self.published_patches.insert(&patch_id, &patch);
        } else {
            env::panic_str("Patch not found");
        }
    }

    /// Fork a patch
    pub fn fork_patch(&mut self, original_patch_id: String, fork_patch_id: String, changes_summary: String) {
        let forker = env::predecessor_account_id();

        // Verify original patch exists
        assert!(self.published_patches.get(&original_patch_id).is_some(), "Original patch not found");

        // Verify fork ID doesn't exist
        assert!(self.published_patches.get(&fork_patch_id).is_none(), "Fork patch ID already exists");

        let fork = PatchFork {
            original_patch_id: original_patch_id.clone(),
            fork_patch_id: fork_patch_id.clone(),
            forked_by: forker,
            forked_at: env::block_timestamp(),
            changes_summary,
        };

        // Add to forks list
        let mut forks = self.patch_forks.get(&original_patch_id).unwrap_or_default();
        forks.push(fork);
        self.patch_forks.insert(&original_patch_id, &forks);

        // Update fork count on original patch
        if let Some(mut original_patch) = self.published_patches.get(&original_patch_id) {
            original_patch.fork_count += 1;
            self.published_patches.insert(&original_patch_id, &original_patch);
        }
    }

    /// Create a patch collection
    pub fn create_collection(&mut self, collection: PatchCollection) -> String {
        let curator = env::predecessor_account_id();

        assert_eq!(collection.curator, curator, "Collection curator must match caller");
        assert!(self.collections.get(&collection.id).is_none(), "Collection ID already exists");

        let mut new_collection = collection.clone();
        new_collection.created_at = env::block_timestamp();

        self.collections.insert(&collection.id, &new_collection);

        collection.id
    }

    /// Add patch to collection
    pub fn add_to_collection(&mut self, collection_id: String, patch_id: String) {
        let curator = env::predecessor_account_id();

        if let Some(mut collection) = self.collections.get(&collection_id) {
            assert_eq!(collection.curator, curator, "Only curator can modify collection");

            // Verify patch exists
            assert!(self.published_patches.get(&patch_id).is_some(), "Patch not found");

            // Add if not already in collection
            if !collection.patches.contains(&patch_id) {
                collection.patches.push(patch_id);
                self.collections.insert(&collection_id, &collection);
            }
        } else {
            env::panic_str("Collection not found");
        }
    }

    /// Feature a patch (admin only)
    pub fn feature_patch(&mut self, patch_id: String, featured: bool) {
        // In practice, this would check for admin permissions
        assert_eq!(env::predecessor_account_id(), self.treasury_id, "Only admin can feature patches");

        if featured {
            self.featured_patches.insert(&patch_id);
        } else {
            self.featured_patches.remove(&patch_id);
        }
    }

    /// Get patch details
    pub fn get_patch(&self, patch_id: String) -> Option<PublishedPatch> {
        self.published_patches.get(&patch_id)
    }

    /// Get user's patches
    pub fn get_user_patches(&self, author: AccountId) -> Vec<PublishedPatch> {
        let patch_ids = self.user_patches.get(&author).unwrap_or_default();
        patch_ids.iter()
            .filter_map(|id| self.published_patches.get(id))
            .collect()
    }

    /// Get featured patches
    pub fn get_featured_patches(&self) -> Vec<PublishedPatch> {
        self.featured_patches.iter()
            .filter_map(|id| self.published_patches.get(&id))
            .collect()
    }

    /// Search patches by tags
    pub fn search_patches(&self, tags: Vec<String>, limit: Option<u32>) -> Vec<PublishedPatch> {
        let limit = limit.unwrap_or(50) as usize;

        self.published_patches.values()
            .filter(|patch| {
                tags.iter().any(|tag| patch.tags.contains(tag))
            })
            .take(limit)
            .collect()
    }

    /// Get patch ratings
    pub fn get_patch_ratings(&self, patch_id: String) -> Vec<PatchRating> {
        self.patch_ratings.get(&patch_id).unwrap_or_default()
    }

    /// Get patch forks
    pub fn get_patch_forks(&self, patch_id: String) -> Vec<PatchFork> {
        self.patch_forks.get(&patch_id).unwrap_or_default()
    }

    /// Get collection details
    pub fn get_collection(&self, collection_id: String) -> Option<PatchCollection> {
        self.collections.get(&collection_id)
    }

    /// Check if user has purchased patch
    pub fn has_purchased(&self, user: AccountId, patch_id: String) -> bool {
        if let Some(purchases) = self.user_purchases.get(&user) {
            purchases.contains(&patch_id)
        } else {
            false
        }
    }

    /// Get marketplace stats
    pub fn get_marketplace_stats(&self) -> near_sdk::serde_json::Value {
        let total_patches = self.published_patches.len();
        let total_collections = self.collections.len();
        let featured_count = self.featured_patches.len();

        near_sdk::serde_json::json!({
            "total_patches": total_patches,
            "total_collections": total_collections,
            "featured_patches": featured_count,
            "platform_fee": self.platform_fee
        })
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
        builder.signer_account_id("alice.testnet".parse().unwrap());
        builder.predecessor_account_id("alice.testnet".parse().unwrap());
        builder.attached_deposit(100_000_000_000_000_000_000_000); // 0.1 NEAR
        builder
    }

    #[test]
    fn test_publish_patch() {
        let context = get_context().build();
        testing_env!(context);

        let mut contract = PatchMarketplaceContract::default();

        let patch = PublishedPatch {
            id: "test_patch".to_string(),
            title: "Test Fractal Shader".to_string(),
            description: "A beautiful fractal shader".to_string(),
            author: "alice.testnet".parse().unwrap(),
            tool_type: "fractal_shader".to_string(),
            version: "1.0.0".to_string(),
            tags: vec!["fractal".to_string(), "shader".to_string()],
            ipfs_cid: "QmTest123".to_string(),
            license: "MIT".to_string(),
            price: Some(1_000_000_000_000_000_000_000_000), // 1 NEAR
            downloads: 0,
            rating: 0.0,
            total_ratings: 0,
            published_at: 0,
            last_updated: 0,
            fork_count: 0,
            dependencies: vec![],
            compatibility: vec!["v1.0+".to_string()],
        };

        let patch_id = contract.publish_patch(patch);
        assert_eq!(patch_id, "test_patch");

        let retrieved = contract.get_patch("test_patch".to_string());
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().title, "Test Fractal Shader");
    }

    #[test]
    fn test_rate_patch() {
        let context = get_context().build();
        testing_env!(context);

        let mut contract = PatchMarketplaceContract::default();

        // First publish a patch
        let patch = PublishedPatch {
            id: "rate_test".to_string(),
            title: "Rating Test".to_string(),
            description: "Test patch for rating".to_string(),
            author: "alice.testnet".parse().unwrap(),
            tool_type: "test".to_string(),
            version: "1.0.0".to_string(),
            tags: vec![],
            ipfs_cid: "QmRateTest".to_string(),
            license: "MIT".to_string(),
            price: None,
            downloads: 0,
            rating: 0.0,
            total_ratings: 0,
            published_at: 0,
            last_updated: 0,
            fork_count: 0,
            dependencies: vec![],
            compatibility: vec![],
        };

        contract.publish_patch(patch);

        // Rate the patch
        contract.rate_patch("rate_test".to_string(), 5, Some("Excellent!".to_string()));

        let ratings = contract.get_patch_ratings("rate_test".to_string());
        assert_eq!(ratings.len(), 1);
        assert_eq!(ratings[0].rating, 5);

        let patch = contract.get_patch("rate_test".to_string()).unwrap();
        assert_eq!(patch.rating, 5.0);
        assert_eq!(patch.total_ratings, 1);
    }
}
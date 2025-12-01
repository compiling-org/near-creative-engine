// Production IPFS Storage Contract for NEAR
// Based on IPFS best practices: https://docs.ipfs.tech/how-to/best-practices-for-nft-data/
// Implements CIDv1, proper metadata structure, and pinning management

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, Timestamp,
};
use std::collections::HashMap;

/// IPFS CID (Content Identifier) - always use CIDv1 in base32
/// Example: bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct CID(pub String);

impl CID {
    /// Validate CID format (CIDv1 base32)
    pub fn validate(&self) -> bool {
        self.0.starts_with("bafy") || self.0.starts_with("bafk")
    }

    /// Convert to IPFS URI
    pub fn to_uri(&self) -> String {
        format!("ipfs://{}", self.0)
    }

    /// Convert to HTTP gateway URL
    pub fn to_gateway_url(&self, gateway: &str) -> String {
        format!("{}/ipfs/{}", gateway, self.0)
    }
}

/// Storage provider types
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum StorageProvider {
    LocalIPFS,         // Local IPFS node
    Web3Storage,       // web3.storage service
    Pinata,            // Pinata pinning service
    NFTStorage,        // nft.storage
    FilecoinDeal,      // Direct Filecoin storage deal
}

/// Pin status for content
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum PinStatus {
    Queued,
    Pinning,
    Pinned,
    Failed,
}

/// Metadata following best practices
/// https://docs.ipfs.tech/how-to/best-practices-for-nft-data/#metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct IPFSMetadata {
    pub name: String,
    pub description: String,
    pub image: String,              // IPFS URI: ipfs://CID
    pub image_data: Option<String>, // Raw SVG data (for on-chain generation)
    pub external_url: Option<String>,
    pub attributes: Vec<MetadataAttribute>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>, // For videos/3D/interactive content
    pub youtube_url: Option<String>,
}

/// Metadata attribute (ERC-721 compatible)
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MetadataAttribute {
    pub trait_type: String,
    pub value: String,
    pub display_type: Option<String>, // "number", "boost_percentage", "boost_number", "date"
}

/// Stored content record
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StoredContent {
    pub cid: CID,
    pub owner: AccountId,
    pub size_bytes: u64,
    pub content_type: String,       // "image/png", "application/json", etc.
    pub pin_status: PinStatus,
    pub providers: Vec<StorageProvider>,
    pub created_at: Timestamp,
    pub last_pinned: Timestamp,
    pub metadata: Option<IPFSMetadata>,
    pub tags: Vec<String>,
}

/// Storage statistics
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageStats {
    pub total_items: u64,
    pub total_bytes: u64,
    pub pinned_items: u64,
    pub failed_pins: u64,
    pub unique_owners: u64,
}

/// Main storage contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct IPFSStorageContract {
    pub owner: AccountId,
    pub content_by_cid: UnorderedMap<CID, StoredContent>,
    pub content_by_owner: LookupMap<AccountId, Vector<CID>>,
    pub total_storage_bytes: u64,
    pub storage_fee_per_mb: Balance, // Fee per MB per year
    pub pinning_services: HashMap<String, String>, // Service name -> API endpoint
}

#[near_bindgen]
impl IPFSStorageContract {
    /// Initialize contract
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner,
            content_by_cid: UnorderedMap::new(b"c"),
            content_by_owner: LookupMap::new(b"o"),
            total_storage_bytes: 0,
            storage_fee_per_mb: 100_000_000_000_000_000_000_000, // 0.1 NEAR per MB per year
            pinning_services: HashMap::new(),
        }
    }

    /// Register content with IPFS CID
    /// Best practice: Upload to IPFS first, then register CID on-chain
    #[payable]
    pub fn register_content(
        &mut self,
        cid: CID,
        size_bytes: u64,
        content_type: String,
        metadata: Option<IPFSMetadata>,
        tags: Vec<String>,
    ) {
        // Validate CID format
        assert!(cid.validate(), "Invalid CID format. Use CIDv1 base32");

        // Calculate required storage fee
        let size_mb = (size_bytes as f64 / 1_000_000.0).ceil() as u128;
        let required_fee = size_mb * self.storage_fee_per_mb;

        assert!(
            env::attached_deposit() >= required_fee,
            "Insufficient storage fee. Required: {}",
            required_fee
        );

        // Create content record
        let content = StoredContent {
            cid: cid.clone(),
            owner: env::predecessor_account_id(),
            size_bytes,
            content_type,
            pin_status: PinStatus::Queued,
            providers: vec![],
            created_at: env::block_timestamp(),
            last_pinned: env::block_timestamp(),
            metadata,
            tags,
        };

        // Store content
        self.content_by_cid.insert(&cid, &content);

        // Update owner's content list
        let owner_id = env::predecessor_account_id();
        let mut owner_content = self
            .content_by_owner
            .get(&owner_id)
            .unwrap_or_else(|| Vector::new(owner_id.as_bytes()));
        owner_content.push(&cid);
        self.content_by_owner.insert(&owner_id, &owner_content);

        // Update stats
        self.total_storage_bytes += size_bytes;

        // Emit event
        env::log_str(&format!(
            "{{\"event\":\"content_registered\",\"cid\":\"{}\",\"owner\":\"{}\",\"size\":{}}}",
            cid.0, owner_id, size_bytes
        ));
    }

    /// Update pin status (called by oracle or keeper)
    pub fn update_pin_status(
        &mut self,
        cid: CID,
        status: PinStatus,
        provider: Option<StorageProvider>,
    ) {
        let mut content = self.content_by_cid.get(&cid).expect("Content not found");

        // Only owner or contract owner can update
        let caller = env::predecessor_account_id();
        assert!(
            caller == content.owner || caller == self.owner,
            "Unauthorized"
        );

        content.pin_status = status;
        content.last_pinned = env::block_timestamp();

        if let Some(prov) = provider {
            if !content.providers.contains(&prov) {
                content.providers.push(prov);
            }
        }

        self.content_by_cid.insert(&cid, &content);

        env::log_str(&format!(
            "{{\"event\":\"pin_status_updated\",\"cid\":\"{}\",\"status\":\"{:?}\"}}",
            cid.0, content.pin_status
        ));
    }

    /// Get content by CID
    pub fn get_content(&self, cid: CID) -> Option<StoredContent> {
        self.content_by_cid.get(&cid)
    }

    /// Get content by owner
    pub fn get_content_by_owner(
        &self,
        owner: AccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<StoredContent> {
        let cids = self.content_by_owner.get(&owner);
        if cids.is_none() {
            return vec![];
        }

        let cids = cids.unwrap();
        let limit = limit.min(50);

        (from_index..from_index + limit)
            .filter_map(|i| cids.get(i))
            .filter_map(|cid| self.content_by_cid.get(&cid))
            .collect()
    }

    /// Generate IPFS URI for content
    pub fn get_ipfs_uri(&self, cid: CID) -> String {
        cid.to_uri()
    }

    /// Generate gateway URL (for HTTP access)
    pub fn get_gateway_url(&self, cid: CID, gateway: Option<String>) -> String {
        let gateway = gateway.unwrap_or_else(|| "https://dweb.link".to_string());
        cid.to_gateway_url(&gateway)
    }

    /// Get storage statistics
    pub fn get_stats(&self) -> StorageStats {
        let total_items = self.content_by_cid.len();
        let mut pinned_items = 0;
        let mut failed_pins = 0;

        for cid in self.content_by_cid.keys() {
            if let Some(content) = self.content_by_cid.get(&cid) {
                match content.pin_status {
                    PinStatus::Pinned => pinned_items += 1,
                    PinStatus::Failed => failed_pins += 1,
                    _ => {}
                }
            }
        }

        StorageStats {
            total_items,
            total_bytes: self.total_storage_bytes,
            pinned_items,
            failed_pins,
            unique_owners: 0, // Would need separate tracking
        }
    }

    /// Helper: Create metadata JSON for IPFS upload
    /// Call this before uploading to IPFS
    pub fn create_metadata_json(
        &self,
        name: String,
        description: String,
        image_cid: CID,
        attributes: Vec<MetadataAttribute>,
    ) -> String {
        let metadata = IPFSMetadata {
            name,
            description,
            image: image_cid.to_uri(),
            image_data: None,
            external_url: None,
            attributes,
            background_color: None,
            animation_url: None,
            youtube_url: None,
        };

        serde_json::to_string(&metadata).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_cid_validation() {
        let valid_cid = CID("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string());
        assert!(valid_cid.validate());

        let uri = valid_cid.to_uri();
        assert!(uri.starts_with("ipfs://"));
    }

    #[test]
    fn test_content_registration() {
        let mut context = VMContextBuilder::new();
        context.predecessor_account_id(accounts(0));
        context.attached_deposit(1_000_000_000_000_000_000_000_000); // 1 NEAR
        testing_env!(context.build());

        let mut contract = IPFSStorageContract::new(accounts(0));

        let cid = CID("bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi".to_string());

        contract.register_content(
            cid.clone(),
            1_000_000, // 1 MB
            "image/png".to_string(),
            None,
            vec!["nft".to_string(), "art".to_string()],
        );

        let content = contract.get_content(cid).unwrap();
        assert_eq!(content.size_bytes, 1_000_000);
        assert_eq!(content.pin_status, PinStatus::Queued);
    }
}

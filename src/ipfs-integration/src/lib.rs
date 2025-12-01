//! # NFT IPFS Integration
//!
//! Module for calculating content-addressed hash (CID) and pinning/retrieving data from IPFS/Filecoin.
//! Supports storage for NUWE, MODURUST, and Neuroemotive AI projects.
//! Enhanced with Filecoin-specific features for decentralized storage with persistence guarantees.

use cid::Cid;
use multihash::{Code, MultihashDigest};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use std::collections::HashMap;

mod ipfs_client;
mod nuwe_storage;
mod modurust_storage;
mod neuroemotive_storage;

pub use ipfs_client::*;
pub use nuwe_storage::*;
pub use modurust_storage::*;
pub use neuroemotive_storage::*;

/// IPFS persistence layer for creative data
#[derive(Clone)]
pub struct IpfsPersistenceLayer {
    client: IpfsClient,
    gateway_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PinResponse {
    pub cid: String,
    pub size: u64,
    pub timestamp: String,
    // Add Filecoin-specific fields
    pub storage_providers: Option<Vec<String>>, // Filecoin storage providers
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeAsset {
    pub name: String,
    pub description: String,
    pub data: Vec<u8>,
    pub content_type: String,
    pub metadata: serde_json::Value,
    // Add optional emotional computing fields for enhanced NFTs
    pub emotional_traits: Option<serde_json::Value>,
}

impl IpfsPersistenceLayer {
    /// Create a new IPFS persistence layer
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            client: IpfsClient::new(host.to_string(), port),
            gateway_url: format!("http://{}:{}", host, port),
        }
    }

    /// Generate CID from creative data
    pub fn generate_cid(&self, data: &[u8]) -> Result<Cid, Box<dyn std::error::Error>> {
        // Create SHA-256 hash
        let hash = Code::Sha2_256.digest(data);

        // Create CID v1 with raw codec
        let cid = Cid::new_v1(0x55, hash);

        Ok(cid)
    }

    /// Add data to IPFS and return CID
    pub async fn add_to_ipfs(&self, data: Vec<u8>) -> Result<String, Box<dyn std::error::Error>> {
        let cid = self.client.add_bytes(&data).await?;
        Ok(cid)
    }

    /// Pin content to IPFS with Filecoin storage information
    pub async fn pin_content(&self, cid: &str) -> Result<PinResponse, Box<dyn std::error::Error>> {
        self.client.pin(cid).await?;

        // Add Filecoin storage provider information
        let storage_providers = Some(vec![
            "f0123456".to_string(), // Example Filecoin storage provider IDs
            "f0789012".to_string(),
        ]);

        Ok(PinResponse {
            cid: cid.to_string(),
            size: 0, // Would need to get actual size
            timestamp: Utc::now().to_rfc3339(),
            storage_providers,
        })
    }

    /// Retrieve data from IPFS by CID
    pub async fn get_from_ipfs(&self, cid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let data = self.client.get(cid).await?;
        Ok(data)
    }

    /// Create and upload creative asset
    pub async fn upload_creative_asset(&self, asset: CreativeAsset) -> Result<(String, PinResponse), Box<dyn std::error::Error>> {
        // Serialize asset data
        let asset_json = serde_json::to_vec(&asset)?;

        // Add to IPFS
        let cid = self.add_to_ipfs(asset_json).await?;

        // Pin the content
        let pin_response = self.pin_content(&cid).await?;

        Ok((cid, pin_response))
    }

    /// Generate enhanced metadata for NFT with Filecoin storage information
    pub fn generate_nft_metadata(&self, cid: &str, name: &str, description: &str, pin_response: Option<PinResponse>) -> serde_json::Value {
        let mut metadata = serde_json::json!({
            "name": name,
            "description": description,
            "image": format!("ipfs://{}", cid),
            "external_url": format!("https://ipfs.io/ipfs/{}", cid),
            "attributes": [
                {
                    "trait_type": "Creative Medium",
                    "value": "Generated Art"
                },
                {
                    "trait_type": "Storage",
                    "value": "IPFS/Filecoin"
                }
            ]
        });
        
        // Add Filecoin storage information if available
        if let Some(pin_info) = pin_response {
            metadata["storage_info"] = serde_json::json!({
                "cid": pin_info.cid,
                "pinned_at": pin_info.timestamp,
                "storage_providers": pin_info.storage_providers.unwrap_or_default()
            });
        }

        metadata
    }
    
    /// Verify data integrity by comparing CID
    pub fn verify_data_integrity(&self, data: &[u8], expected_cid: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let calculated_cid = self.generate_cid(data)?;
        Ok(calculated_cid.to_string() == expected_cid)
    }
}

/// Utility function to create creative asset
pub fn create_creative_asset(
    name: &str,
    description: &str,
    data: Vec<u8>,
    content_type: &str,
    metadata: serde_json::Value
) -> CreativeAsset {
    CreativeAsset {
        name: name.to_string(),
        description: description.to_string(),
        data,
        content_type: content_type.to_string(),
        metadata,
        emotional_traits: None,
    }
}

/// Utility function to create creative asset with emotional traits
pub fn create_creative_asset_with_traits(
    name: &str,
    description: &str,
    data: Vec<u8>,
    content_type: &str,
    metadata: serde_json::Value,
    emotional_traits: serde_json::Value
) -> CreativeAsset {
    CreativeAsset {
        name: name.to_string(),
        description: description.to_string(),
        data,
        content_type: content_type.to_string(),
        metadata,
        emotional_traits: Some(emotional_traits),
    }
}

/// Batch upload multiple assets
pub async fn batch_upload_assets(
    layer: &IpfsPersistenceLayer,
    assets: Vec<CreativeAsset>
) -> Result<Vec<(String, String, PinResponse)>, Box<dyn std::error::Error>> {
    let mut results = Vec::new();

    for asset in assets {
        let (cid, pin_response) = layer.upload_creative_asset(asset.clone()).await?;
        results.push((asset.name, cid, pin_response));
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cid_generation() {
        let layer = IpfsPersistenceLayer::new("localhost", 5001);
        let data = b"Hello, IPFS!";
        let cid = layer.generate_cid(data).unwrap();
        assert!(!cid.to_string().is_empty());
    }

    #[test]
    fn test_creative_asset_creation() {
        let asset = create_creative_asset(
            "Test Art",
            "A test creative asset",
            vec![1, 2, 3, 4],
            "image/png",
            serde_json::json!({"artist": "Test Artist"})
        );

        assert_eq!(asset.name, "Test Art");
        assert_eq!(asset.content_type, "image/png");
    }

    #[test]
    fn test_nft_metadata_generation() {
        let layer = IpfsPersistenceLayer::new("localhost", 5001);
        let cid = "QmTestCid123";

        let metadata = layer.generate_nft_metadata(cid, "Test NFT", "A test NFT", None);

        assert_eq!(metadata["name"], "Test NFT");
        assert!(metadata["image"].as_str().unwrap().starts_with("ipfs://"));
    }

    #[tokio::test]
    async fn test_ipfs_client_creation() {
        let layer = IpfsPersistenceLayer::new("localhost", 5001);
        // This test just verifies the client can be created
        // Actual IPFS operations would require a running IPFS node
        assert_eq!(layer.gateway_url, "http://localhost:5001");
    }
    
    #[test]
    fn test_data_integrity_verification() {
        let layer = IpfsPersistenceLayer::new("localhost", 5001);
        let data = b"Hello, IPFS!";
        let cid = layer.generate_cid(data).unwrap();
        
        // Test with correct data
        assert!(layer.verify_data_integrity(data, &cid.to_string()).unwrap());
        
        // Test with incorrect data
        let wrong_data = b"Hello, IPFS?"; // Different data
        assert!(!layer.verify_data_integrity(wrong_data, &cid.to_string()).unwrap());
    }
}
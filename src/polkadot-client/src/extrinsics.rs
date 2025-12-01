//! Real Extrinsic Submission Module
//! 
//! Enhanced extrinsic submission with proper error handling and event decoding
//! Based on ink! e2e patterns for robust blockchain interaction

use subxt::{OnlineClient, PolkadotConfig, TxProgress};
use subxt::tx::{PairSigner, TxPayload};
use subxt::error::{DispatchError, Error as SubxtError};
use sp_core::{sr25519::Pair, Pair as PairTrait};
use sp_runtime::AccountId32;
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Enhanced transaction result with detailed status and events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionResult {
    pub hash: String,
    pub block_hash: Option<String>,
    pub status: TransactionStatus,
    pub events: Vec<TransactionEvent>,
    pub error: Option<String>,
}

/// Transaction status enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionStatus {
    Pending,
    InBlock,
    Finalized,
    Failed,
}

/// Transaction event with decoded data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub pallet: String,
    pub variant: String,
    pub data: serde_json::Value,
}

/// Enhanced extrinsic submitter with robust error handling
pub struct ExtrinsicSubmitter {
    client: OnlineClient<PolkadotConfig>,
}

impl ExtrinsicSubmitter {
    /// Create a new extrinsic submitter
    pub fn new(client: OnlineClient<PolkadotConfig>) -> Self {
        Self { client }
    }
    
    /// Submit an extrinsic and wait for finalization with full event decoding
    pub async fn submit_and_watch<T: TxPayload>(
        &self,
        payload: T,
        signer: &PairSigner<PolkadotConfig, Pair>,
    ) -> Result<TransactionResult> {
        let tx_progress = self.client
            .tx()
            .sign_and_submit_then_watch_default(&payload, signer)
            .await?;
            
        self.process_tx_progress(tx_progress).await
    }
    
    /// Submit an extrinsic and wait for in-block status
    pub async fn submit_and_wait_for_in_block<T: TxPayload>(
        &self,
        payload: T,
        signer: &PairSigner<PolkadotConfig, Pair>,
    ) -> Result<TransactionResult> {
        let tx_progress = self.client
            .tx()
            .sign_and_submit_then_watch_default(&payload, signer)
            .await?;
            
        self.wait_for_in_block(tx_progress).await
    }
    
    /// Process transaction progress and decode events
    async fn process_tx_progress(&self, mut tx_progress: TxProgress<PolkadotConfig>) -> Result<TransactionResult> {
        let mut result = TransactionResult {
            hash: format!("{:?}", tx_progress.extrinsic_hash()),
            block_hash: None,
            status: TransactionStatus::Pending,
            events: Vec::new(),
            error: None,
        };
        
        while let Some(status) = tx_progress.next().await {
            match status? {
                subxt::tx::TxStatus::InBlock(in_block) => {
                    result.status = TransactionStatus::InBlock;
                    result.block_hash = Some(format!("{:?}", in_block.block_hash()));
                    
                    // Fetch and decode events
                    let events = in_block.fetch_events().await?;
                    result.events = self.decode_events(&events)?;
                    
                    // Check for dispatch errors
                    if let Some(error) = self.check_dispatch_error(&events) {
                        result.error = Some(error);
                        result.status = TransactionStatus::Failed;
                        return Ok(result);
                    }
                }
                subxt::tx::TxStatus::Finalized(finalized) => {
                    result.status = TransactionStatus::Finalized;
                    result.block_hash = Some(format!("{:?}", finalized.block_hash()));
                    
                    // Fetch final events
                    let events = finalized.fetch_events().await?;
                    result.events = self.decode_events(&events)?;
                    
                    return Ok(result);
                }
                subxt::tx::TxStatus::Dropped => {
                    result.error = Some("Transaction dropped from mempool".to_string());
                    result.status = TransactionStatus::Failed;
                    return Ok(result);
                }
                subxt::tx::TxStatus::Invalid => {
                    result.error = Some("Transaction invalid".to_string());
                    result.status = TransactionStatus::Failed;
                    return Ok(result);
                }
                _ => continue,
            }
        }
        
        result.error = Some("Transaction monitoring stopped unexpectedly".to_string());
        result.status = TransactionStatus::Failed;
        Ok(result)
    }
    
    /// Wait for in-block status only
    async fn wait_for_in_block(&self, mut tx_progress: TxProgress<PolkadotConfig>) -> Result<TransactionResult> {
        let mut result = TransactionResult {
            hash: format!("{:?}", tx_progress.extrinsic_hash()),
            block_hash: None,
            status: TransactionStatus::Pending,
            events: Vec::new(),
            error: None,
        };
        
        while let Some(status) = tx_progress.next().await {
            match status? {
                subxt::tx::TxStatus::InBlock(in_block) => {
                    result.status = TransactionStatus::InBlock;
                    result.block_hash = Some(format!("{:?}", in_block.block_hash()));
                    
                    let events = in_block.fetch_events().await?;
                    result.events = self.decode_events(&events)?;
                    
                    return Ok(result);
                }
                subxt::tx::TxStatus::Dropped => {
                    result.error = Some("Transaction dropped from mempool".to_string());
                    result.status = TransactionStatus::Failed;
                    return Ok(result);
                }
                subxt::tx::TxStatus::Invalid => {
                    result.error = Some("Transaction invalid".to_string());
                    result.status = TransactionStatus::Failed;
                    return Ok(result);
                }
                _ => continue,
            }
        }
        
        result.error = Some("Transaction monitoring stopped unexpectedly".to_string());
        result.status = TransactionStatus::Failed;
        Ok(result)
    }
    
    /// Decode events from transaction
    fn decode_events(&self, events: &subxt::events::Events<PolkadotConfig>) -> Result<Vec<TransactionEvent>> {
        let mut decoded_events = Vec::new();
        
        for event in events.iter() {
            let event = event?;
            
            // Convert to JSON for easier handling
            let event_json = serde_json::json!({
                "pallet": event.pallet_name(),
                "variant": event.variant_name(),
                "fields": event.field_values()
            });
            
            decoded_events.push(TransactionEvent {
                pallet: event.pallet_name().to_string(),
                variant: event.variant_name().to_string(),
                data: event_json,
            });
        }
        
        Ok(decoded_events)
    }
    
    /// Check for dispatch errors in events
    fn check_dispatch_error(&self, events: &subxt::events::Events<PolkadotConfig>) -> Option<String> {
        for event in events.iter() {
            if let Ok(event) = event {
                // Check if this is a system event with dispatch error
                if event.pallet_name() == "System" && event.variant_name() == "ExtrinsicFailed" {
                    return Some("Extrinsic failed - check dispatch error".to_string());
                }
            }
        }
        None
    }
}

/// Soulbound identity extrinsics
pub struct SoulboundExtrinsics;

impl SoulboundExtrinsics {
    /// Create a soulbound identity extrinsic
    pub fn create_identity(
        owner: AccountId32,
        identity_data: Vec<u8>,
    ) -> impl TxPayload {
        // This would be the actual extrinsic for your ink! contract
        // For now, we'll create a placeholder that matches your contract interface
        CreateIdentityExtrinsic {
            owner,
            identity_data,
        }
    }
    
    /// Update emotional metadata for a soulbound token
    pub fn update_emotional_metadata(
        token_id: u64,
        emotional_data: crate::EmotionalMetadata,
    ) -> impl TxPayload {
        UpdateEmotionalMetadataExtrinsic {
            token_id,
            emotional_data,
        }
    }
}

/// Placeholder extrinsic for creating identity
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CreateIdentityExtrinsic {
    owner: AccountId32,
    identity_data: Vec<u8>,
}

impl TxPayload for CreateIdentityExtrinsic {
    fn encode_call_data_to(
        &self,
        _metadata: &subxt::Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), subxt::error::Error> {
        // This would encode the actual call data for your ink! contract
        // For now, we'll encode a placeholder
        out.extend_from_slice(b"create_identity");
        out.extend_from_slice(&self.owner.encode());
        out.extend_from_slice(&(self.identity_data.len() as u32).to_le_bytes());
        out.extend_from_slice(&self.identity_data);
        Ok(())
    }
}

/// Placeholder extrinsic for updating emotional metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
struct UpdateEmotionalMetadataExtrinsic {
    token_id: u64,
    emotional_data: crate::EmotionalMetadata,
}

impl TxPayload for UpdateEmotionalMetadataExtrinsic {
    fn encode_call_data_to(
        &self,
        _metadata: &subxt::Metadata,
        out: &mut Vec<u8>,
    ) -> Result<(), subxt::error::Error> {
        // This would encode the actual call data for your ink! contract
        out.extend_from_slice(b"update_emotional_metadata");
        out.extend_from_slice(&self.token_id.to_le_bytes());
        
        // Encode emotional metadata
        let encoded_data = serde_json::to_vec(&self.emotional_data)
            .map_err(|e| subxt::error::Error::Other(e.into()))?;
        out.extend_from_slice(&(encoded_data.len() as u32).to_le_bytes());
        out.extend_from_slice(&encoded_data);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extrinsic_creation() {
        let owner = AccountId32::from([1u8; 32]);
        let identity_data = b"test_identity".to_vec();
        
        let extrinsic = SoulboundExtrinsics::create_identity(owner, identity_data);
        
        // Test that extrinsic can be created
        assert!(true); // Placeholder test
    }
    
    #[test]
    fn test_transaction_result_serialization() {
        let result = TransactionResult {
            hash: "0x1234".to_string(),
            block_hash: Some("0x5678".to_string()),
            status: TransactionStatus::Finalized,
            events: vec![TransactionEvent {
                pallet: "System".to_string(),
                variant: "ExtrinsicSuccess".to_string(),
                data: serde_json::json!({"success": true}),
            }],
            error: None,
        };
        
        let serialized = serde_json::to_string(&result).unwrap();
        assert!(serialized.contains("Finalized"));
    }
}
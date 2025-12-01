//! NUWE Marketplace Integration
//! 
//! Marketplace features specific to NUWE/Fractal Studio sessions and VJ performances

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp};
use near_contract_standards::non_fungible_token::TokenId;

/// NUWE creative session NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NuweSessionNFT {
    pub token_id: TokenId,
    pub session_id: String,
    pub session_type: SessionType,
    pub creator: AccountId,
    pub created_at: Timestamp,
    pub session_duration: u64,
    pub ipfs_cid: String,
    pub performance_metrics: PerformanceMetrics,
    pub emotional_summary: EmotionalSummary,
    pub preview_url: String,
}

/// Type of NUWE session
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum SessionType {
    FractalStudio,
    WGSLStudio,
    VJPerformance,
    LiveCoding,
    ImmersiveVJ,
}

/// Performance metrics for session
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PerformanceMetrics {
    pub avg_fps: f32,
    pub peak_fps: f32,
    pub total_frames: u64,
    pub unique_parameters_modified: u32,
}

/// Summary of emotional data during session
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalSummary {
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub avg_dominance: f32,
    pub emotional_variance: f32,
}

/// VJ performance collection
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VJCollection {
    pub collection_id: String,
    pub artist: AccountId,
    pub collection_name: String,
    pub sessions: Vec<TokenId>,
    pub total_duration: u64,
    pub created_at: Timestamp,
}

/// Fractal artwork NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FractalArtwork {
    pub token_id: TokenId,
    pub fractal_type: String,
    pub parameters: FractalParameters,
    pub resolution: (u32, u32),
    pub render_time: u32,
    pub ipfs_cid: String,
}

/// Fractal parameters
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FractalParameters {
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub iterations: u32,
    pub color_palette: Vec<u32>,
}

impl NuweSessionNFT {
    /// Create a new NUWE session NFT
    pub fn new(
        token_id: TokenId,
        session_id: String,
        session_type: SessionType,
        session_duration: u64,
        ipfs_cid: String,
    ) -> Self {
        Self {
            token_id,
            session_id,
            session_type,
            creator: env::predecessor_account_id(),
            created_at: env::block_timestamp(),
            session_duration,
            ipfs_cid,
            performance_metrics: PerformanceMetrics {
                avg_fps: 0.0,
                peak_fps: 0.0,
                total_frames: 0,
                unique_parameters_modified: 0,
            },
            emotional_summary: EmotionalSummary {
                avg_valence: 0.0,
                avg_arousal: 0.0,
                avg_dominance: 0.0,
                emotional_variance: 0.0,
            },
            preview_url: String::new(),
        }
    }

    /// Get session value score (for pricing)
    pub fn value_score(&self) -> u32 {
        let mut score = 0u32;
        
        // Longer sessions are more valuable
        score += (self.session_duration / 60) as u32; // 1 point per minute
        
        // Higher FPS indicates quality
        score += (self.performance_metrics.avg_fps / 10.0) as u32;
        
        // More frames = more content
        score += (self.performance_metrics.total_frames / 100) as u32;
        
        // Parameter variety indicates creativity
        score += self.performance_metrics.unique_parameters_modified;
        
        score
    }
}

impl VJCollection {
    /// Create a new VJ collection
    pub fn new(collection_id: String, collection_name: String) -> Self {
        Self {
            collection_id,
            artist: env::predecessor_account_id(),
            collection_name,
            sessions: Vec::new(),
            total_duration: 0,
            created_at: env::block_timestamp(),
        }
    }

    /// Add session to collection
    pub fn add_session(&mut self, token_id: TokenId, duration: u64) {
        self.sessions.push(token_id);
        self.total_duration += duration;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuwe_session_nft_creation() {
        let nft = NuweSessionNFT::new(
            "token_001".to_string(),
            "session_123".to_string(),
            SessionType::FractalStudio,
            300,
            "QmXXXXX".to_string(),
        );

        assert_eq!(nft.session_id, "session_123");
        assert_eq!(nft.session_duration, 300);
    }

    #[test]
    fn test_value_score_calculation() {
        let mut nft = NuweSessionNFT::new(
            "token_001".to_string(),
            "session_123".to_string(),
            SessionType::VJPerformance,
            600, // 10 minutes
            "QmXXXXX".to_string(),
        );

        nft.performance_metrics.avg_fps = 60.0;
        nft.performance_metrics.total_frames = 36000;
        nft.performance_metrics.unique_parameters_modified = 50;

        let score = nft.value_score();
        assert!(score > 0);
    }
}

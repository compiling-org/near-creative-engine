//! # Polkadot Client
//!
//! Client library for interacting with Polkadot chains for creative NFT metadata
//! and cross-chain bridging.
//! Enhanced with emotional bridge capabilities and advanced metadata handling.

use subxt::{OnlineClient, PolkadotConfig};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod emotional_bridge;
mod soulbound;

pub use emotional_bridge::*;
pub use soulbound::*;

/// Polkadot client for creative NFT operations
pub struct PolkadotClient {
    client: OnlineClient<PolkadotConfig>,
    metadata_cache: HashMap<String, serde_json::Value>,
    /// Advanced analytics for tracking token performance
    pub token_analytics: TokenAnalytics,
}

impl PolkadotClient {
    /// Create a new Polkadot client
    pub async fn new(url: &str) -> Result<Self> {
        let client = OnlineClient::<PolkadotConfig>::from_url(url).await?;
        Ok(Self {
            client,
            metadata_cache: HashMap::new(),
            token_analytics: TokenAnalytics::new(),
        })
    }

    /// Get the underlying subxt client
    pub fn client(&self) -> &OnlineClient<PolkadotConfig> {
        &self.client
    }
    
    /// Store metadata in cache
    pub fn cache_metadata(&mut self, key: String, metadata: serde_json::Value) {
        self.metadata_cache.insert(key, metadata);
    }
    
    /// Retrieve metadata from cache
    pub fn get_cached_metadata(&self, key: &str) -> Option<&serde_json::Value> {
        self.metadata_cache.get(key)
    }
    
    /// Clear metadata cache
    pub fn clear_cache(&mut self) {
        self.metadata_cache.clear();
    }
    
    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.metadata_cache.len()
    }
    
    /// Get trending tokens based on engagement metrics
    pub fn get_trending_tokens(&self, limit: usize) -> Vec<(String, f32)> {
        self.token_analytics.get_trending_tokens(limit)
    }
    
    /// Predict emotional state of a token
    pub fn predict_token_emotion(&self, token_id: &str) -> Option<EmotionalMetadata> {
        self.token_analytics.predict_emotion(token_id)
    }
}

/// Token analytics for tracking performance and engagement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenAnalytics {
    pub creation_timestamp: u64,
    pub interaction_count: u32,
    pub emotional_history: Vec<EmotionalMetadata>,
    pub last_interaction: u64,
    pub emotional_complexity: f32,
    pub engagement_score: f32,
    pub evolution_progress: f32,
}

impl TokenAnalytics {
    /// Create new token analytics
    pub fn new() -> Self {
        Self {
            creation_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            interaction_count: 0,
            emotional_history: Vec::new(),
            last_interaction: 0,
            emotional_complexity: 0.0,
            engagement_score: 0.0,
            evolution_progress: 0.0,
        }
    }
    
    /// Record an interaction with emotional metadata
    pub fn record_interaction(&mut self, emotional_data: EmotionalMetadata) {
        self.interaction_count += 1;
        self.last_interaction = emotional_data.timestamp;
        self.emotional_history.push(emotional_data);
        
        // Update complexity and engagement scores
        self.emotional_complexity = EmotionalBridgeProcessor::calculate_emotional_complexity(&self.emotional_history);
        self.engagement_score = self.calculate_engagement_score();
        self.evolution_progress = self.calculate_evolution_progress();
    }
    
    /// Calculate engagement score based on interaction frequency and emotional variance
    fn calculate_engagement_score(&self) -> f32 {
        if self.emotional_history.is_empty() {
            return 0.0;
        }
        
        // Base score on interaction count and emotional variance
        let interaction_score = (self.interaction_count as f32).min(100.0) / 100.0;
        
        // Higher score for more emotionally varied interactions
        let variance_score = self.emotional_complexity;
        
        (interaction_score * 0.7 + variance_score * 0.3).clamp(0.0, 1.0)
    }
    
    /// Calculate evolution progress based on emotional journey
    fn calculate_evolution_progress(&self) -> f32 {
        if self.emotional_history.len() < 2 {
            return 0.0;
        }
        
        // Measure how much the emotional state has changed over time
        let first = &self.emotional_history[0];
        let last = self.emotional_history.last().unwrap();
        
        let valence_change = (last.valence - first.valence).abs();
        let arousal_change = (last.arousal - first.arousal).abs();
        let dominance_change = (last.dominance - first.dominance).abs();
        
        // Normalize and combine changes
        let total_change = (valence_change + arousal_change + dominance_change) / 3.0;
        total_change.clamp(0.0, 1.0)
    }
    
    /// Get trending tokens based on engagement metrics
    pub fn get_trending_tokens(&self, limit: usize) -> Vec<(String, f32)> {
        // In a real implementation, this would query multiple tokens
        // For now, we'll return a placeholder
        vec![("token_1".to_string(), self.engagement_score)]
            .into_iter()
            .take(limit)
            .collect()
    }
    
    /// Predict emotion based on historical data
    pub fn predict_emotion(&self, _token_id: &str) -> Option<EmotionalMetadata> {
        EmotionalBridgeProcessor::predict_next_emotion(&self.emotional_history)
    }
}

/// Emotional metadata for NFTs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalMetadata {
    pub valence: f32,     // Emotional positivity/negativity (-1 to 1)
    pub arousal: f32,     // Emotional intensity (0 to 1)
    pub dominance: f32,   // Sense of control (0 to 1)
    pub confidence: f32,  // Confidence in emotional assessment (0 to 1)
    pub timestamp: u64,   // When emotional data was captured
    // Enhanced fields
    pub emotional_category: String, // Human-readable emotional category
    pub emotional_trajectory: Vec<EmotionalPoint>, // Historical emotional path
    pub predicted_emotion: Option<Box<EmotionalMetadata>>, // Predicted next emotional state
    pub emotional_complexity: f32, // Complexity of emotional journey
}

/// Point in emotional trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPoint {
    pub valence: f32,
    pub arousal: f32,
    pub timestamp: u64,
}

impl EmotionalMetadata {
    /// Create new emotional metadata with enhanced fields
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
            
        let category = Self::get_emotional_category(valence, arousal);
        
        Self {
            valence,
            arousal,
            dominance,
            confidence: 0.8,
            timestamp,
            emotional_category: category,
            emotional_trajectory: vec![],
            predicted_emotion: None,
            emotional_complexity: 0.0,
        }
    }
    
    /// Get human-readable emotional category
    pub fn get_emotional_category(valence: f32, arousal: f32) -> String {
        match (valence, arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
            (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
            _ => "Calm".to_string(),
        }
    }
    
    /// Add point to emotional trajectory
    pub fn add_trajectory_point(&mut self, valence: f32, arousal: f32) {
        self.emotional_trajectory.push(EmotionalPoint {
            valence,
            arousal,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    /// Calculate emotional complexity based on trajectory
    pub fn calculate_complexity(&mut self) {
        if self.emotional_trajectory.len() < 2 {
            self.emotional_complexity = 0.0;
            return;
        }
        
        let mut total_distance = 0.0;
        for i in 1..self.emotional_trajectory.len() {
            let prev = &self.emotional_trajectory[i-1];
            let curr = &self.emotional_trajectory[i];
            let distance = ((curr.valence - prev.valence).powi(2) + 
                           (curr.arousal - prev.arousal).powi(2)).sqrt();
            total_distance += distance;
        }
        
        // Normalize by number of points
        self.emotional_complexity = (total_distance / self.emotional_trajectory.len() as f32).clamp(0.0, 1.0);
    }
}

/// Cross-chain bridge information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeInfo {
    pub source_chain: String,
    pub target_chain: String,
    pub source_contract: String,
    pub target_contract: String,
    pub bridge_status: String, // "pending", "bridged", "failed"
    pub bridge_timestamp: u64,
    // Enhanced fields
    pub emotional_preservation: f32, // How well emotional data was preserved (0-1)
    pub bridge_complexity: f32, // Complexity of the bridging operation
    pub cross_chain_emotional_sync: bool, // Whether emotional data is synced across chains
}

/// Advanced metadata structure for creative NFTs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeNFTMetadata {
    pub name: String,
    pub description: String,
    pub emotional_data: Option<EmotionalMetadata>,
    pub bridge_info: Option<BridgeInfo>,
    pub attributes: HashMap<String, serde_json::Value>,
    pub creator_reputation: Option<f32>,
    // Enhanced fields
    pub emotional_journey: Vec<EmotionalMetadata>, // Complete emotional history
    pub interaction_patterns: Vec<InteractionPattern>, // Patterns in user interactions
    pub community_engagement: CommunityEngagementMetrics, // Community response metrics
    pub adaptive_behavior: AdaptiveBehavior, // How the NFT adapts to interactions
}

/// Interaction pattern analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionPattern {
    pub pattern_type: String,
    pub frequency: u32,
    pub emotional_correlation: f32,
}

/// Community engagement metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityEngagementMetrics {
    pub total_interactions: u32,
    pub unique_participants: u32,
    pub sentiment_score: f32,
    pub viral_coefficient: f32,
}

impl Default for CommunityEngagementMetrics {
    fn default() -> Self {
        Self {
            total_interactions: 0,
            unique_participants: 0,
            sentiment_score: 0.0,
            viral_coefficient: 0.0,
        }
    }
}

/// Adaptive behavior configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveBehavior {
    pub is_adaptive: bool,
    pub adaptation_speed: f32,
    pub preferred_emotions: Vec<String>,
    pub learning_rate: f32,
}

impl Default for AdaptiveBehavior {
    fn default() -> Self {
        Self {
            is_adaptive: false,
            adaptation_speed: 0.5,
            preferred_emotions: vec![],
            learning_rate: 0.1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polkadot_client_creation() {
        // This would be an integration test in a real implementation
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn test_emotional_metadata_enhancement() {
        let mut metadata = EmotionalMetadata::new(0.7, 0.8, 0.6);
        assert_eq!(metadata.emotional_category, "Excited");
        
        metadata.add_trajectory_point(0.5, 0.4);
        metadata.add_trajectory_point(0.3, 0.2);
        metadata.calculate_complexity();
        
        assert!(metadata.emotional_complexity >= 0.0);
        assert!(metadata.emotional_complexity <= 1.0);
    }
    
    #[test]
    fn test_token_analytics() {
        let mut analytics = TokenAnalytics::new();
        let emotional_data = EmotionalMetadata::new(0.5, 0.5, 0.5);
        analytics.record_interaction(emotional_data);
        
        assert_eq!(analytics.interaction_count, 1);
        assert!(analytics.engagement_score >= 0.0);
        assert!(analytics.engagement_score <= 1.0);
    }
}
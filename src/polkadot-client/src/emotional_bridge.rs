//! Emotional Bridge Module
//!
//! Advanced cross-chain emotional computing capabilities for Polkadot integrations

use serde::{Deserialize, Serialize};
use crate::{EmotionalMetadata, BridgeInfo};

/// Emotional bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalBridgeConfig {
    pub source_chain: String,
    pub target_chain: String,
    pub emotional_sync_enabled: bool,
    pub sync_frequency: u64, // seconds
    pub confidence_threshold: f32,
}

/// Advanced emotional profile for creators
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreatorEmotionalProfile {
    pub creator_id: String,
    pub emotional_history: Vec<EmotionalMetadata>,
    pub emotional_trend: EmotionalTrend,
    pub predicted_next_emotion: Option<EmotionalMetadata>,
    pub emotional_complexity: f32,
    pub creativity_index: f32,
    pub engagement_score: f32,
}

/// Emotional trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionalTrend {
    Ascending,
    Descending,
    Stable,
    Volatile,
}

impl Default for EmotionalTrend {
    fn default() -> Self {
        EmotionalTrend::Stable
    }
}

/// Emotional bridge processor
pub struct EmotionalBridgeProcessor;

impl EmotionalBridgeProcessor {
    /// Process emotional metadata for cross-chain transfer
    pub fn process_emotional_bridge(
        config: &EmotionalBridgeConfig,
        metadata: &EmotionalMetadata,
    ) -> Option<BridgeInfo> {
        if !config.emotional_sync_enabled || metadata.confidence < config.confidence_threshold {
            return None;
        }

        Some(BridgeInfo {
            source_chain: config.source_chain.clone(),
            target_chain: config.target_chain.clone(),
            source_contract: String::new(),
            target_contract: String::new(),
            bridge_status: "pending".to_string(),
            bridge_timestamp: metadata.timestamp,
            emotional_preservation: 0.95, // Default high preservation
            bridge_complexity: 0.3, // Default complexity
            cross_chain_emotional_sync: config.emotional_sync_enabled,
        })
    }

    /// Analyze emotional trend from history
    pub fn analyze_emotional_trend(history: &[EmotionalMetadata]) -> EmotionalTrend {
        if history.len() < 2 {
            return EmotionalTrend::Stable;
        }

        let recent = history.iter().take(5.min(history.len())).collect::<Vec<_>>();
        let oldest = recent.first().unwrap();
        let newest = recent.last().unwrap();

        let valence_diff = newest.valence - oldest.valence;
        let arousal_diff = newest.arousal - oldest.arousal;

        match (valence_diff.abs(), arousal_diff.abs()) {
            (v, a) if v < 0.1 && a < 0.1 => EmotionalTrend::Stable,
            (v, a) if v > 0.3 || a > 0.3 => EmotionalTrend::Volatile,
            _ => {
                if valence_diff > 0.1 || arousal_diff > 0.1 {
                    EmotionalTrend::Ascending
                } else if valence_diff < -0.1 || arousal_diff < -0.1 {
                    EmotionalTrend::Descending
                } else {
                    EmotionalTrend::Stable
                }
            }
        }
    }

    /// Predict next emotional state
    pub fn predict_next_emotion(history: &[EmotionalMetadata]) -> Option<EmotionalMetadata> {
        if history.len() < 3 {
            return None;
        }

        let len = history.len();
        let latest = &history[len - 1];
        let previous = &history[len - 2];
        let older = &history[len - 3];

        // Simple linear extrapolation
        let valence_delta = (latest.valence - previous.valence) * 0.7 + (previous.valence - older.valence) * 0.3;
        let arousal_delta = (latest.arousal - previous.arousal) * 0.7 + (previous.arousal - older.arousal) * 0.3;
        let dominance_delta = (latest.dominance - previous.dominance) * 0.7 + (previous.dominance - older.dominance) * 0.3;
        let confidence_delta = (latest.confidence - previous.confidence) * 0.7 + (previous.confidence - older.confidence) * 0.3;

        Some(EmotionalMetadata {
            valence: (latest.valence + valence_delta).clamp(-1.0, 1.0),
            arousal: (latest.arousal + arousal_delta).clamp(0.0, 1.0),
            dominance: (latest.dominance + dominance_delta).clamp(0.0, 1.0),
            confidence: (latest.confidence + confidence_delta).clamp(0.0, 1.0),
            timestamp: latest.timestamp + 3600, // Predict 1 hour ahead
            emotional_category: EmotionalMetadata::get_emotional_category(latest.valence + valence_delta, latest.arousal + arousal_delta),
            emotional_trajectory: latest.emotional_trajectory.clone(),
            predicted_emotion: None, // Would need recursive handling in a real implementation
            emotional_complexity: latest.emotional_complexity,
        })
    }

    /// Calculate emotional complexity score
    pub fn calculate_emotional_complexity(history: &[EmotionalMetadata]) -> f32 {
        if history.is_empty() {
            return 0.0;
        }

        let len = history.len();

        // Calculate variance in emotional dimensions
        let avg_valence: f32 = history.iter().map(|e| e.valence).sum::<f32>() / len as f32;
        let avg_arousal: f32 = history.iter().map(|e| e.arousal).sum::<f32>() / len as f32;
        let avg_dominance: f32 = history.iter().map(|e| e.dominance).sum::<f32>() / len as f32;

        let valence_variance: f32 = history.iter().map(|e| (e.valence - avg_valence).powi(2)).sum::<f32>() / len as f32;
        let arousal_variance: f32 = history.iter().map(|e| (e.arousal - avg_arousal).powi(2)).sum::<f32>() / len as f32;
        let dominance_variance: f32 = history.iter().map(|e| (e.dominance - avg_dominance).powi(2)).sum::<f32>() / len as f32;

        // Complexity is higher when there's more variation
        let total_variance = (valence_variance + arousal_variance + dominance_variance).sqrt();

        // Normalize to 0-1 range
        total_variance.clamp(0.0, 1.0)
    }
}
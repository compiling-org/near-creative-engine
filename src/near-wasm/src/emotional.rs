//! Emotional data structures for interactive NFTs
//!
//! Enhanced with advanced emotional computing and prediction capabilities.

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalData {
    pub timestamp: Timestamp,
    pub valence: f32,      // pleasure vs displeasure (-1.0 to 1.0)
    pub arousal: f32,      // calm vs excited (0.0 to 1.0)
    pub dominance: f32,    // controlled vs in-control (0.0 to 1.0)
    pub confidence: f32,   // certainty of emotional state (0.0 to 1.0)
    pub raw_vector: Vec<f32>,
    pub emotional_vector: EmotionalVector,
    // Add advanced emotional metrics
    pub emotional_trajectory: Vec<EmotionalVector>,
    pub predicted_emotion: Option<EmotionalVector>,
    pub emotional_complexity: f32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub timestamp: Timestamp,
}

impl EmotionalData {
    pub fn new() -> Self {
        Self {
            timestamp: near_sdk::env::block_timestamp(),
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
            confidence: 0.8,
            raw_vector: vec![],
            emotional_vector: EmotionalVector {
                valence: 0.0,
                arousal: 0.5,
                dominance: 0.5,
                timestamp: near_sdk::env::block_timestamp(),
            },
            emotional_trajectory: vec![],
            predicted_emotion: None,
            emotional_complexity: 0.0,
        }
    }
    
    pub fn from_vector(raw_vector: Vec<f32>) -> Self {
        // Simple emotion detection from raw vector
        // In practice, this would use a more sophisticated model
        let valence = if raw_vector.len() > 0 { raw_vector[0].clamp(-1.0, 1.0) } else { 0.0 };
        let arousal = if raw_vector.len() > 1 { raw_vector[1].clamp(0.0, 1.0) } else { 0.5 };
        let dominance = if raw_vector.len() > 2 { raw_vector[2].clamp(0.0, 1.0) } else { 0.5 };
        
        Self {
            timestamp: near_sdk::env::block_timestamp(),
            valence,
            arousal,
            dominance,
            confidence: 0.8,
            raw_vector,
            emotional_vector: EmotionalVector {
                valence,
                arousal,
                dominance,
                timestamp: near_sdk::env::block_timestamp(),
            },
            emotional_trajectory: vec![],
            predicted_emotion: None,
            emotional_complexity: 0.0,
        }
    }
    
    /// Add a new emotional state to the trajectory
    pub fn add_to_trajectory(&mut self, emotion: EmotionalVector) {
        self.emotional_trajectory.push(emotion);
        // Keep only the last 10 emotional states
        if self.emotional_trajectory.len() > 10 {
            self.emotional_trajectory.remove(0);
        }
        // Update complexity based on trajectory variance
        self.update_emotional_complexity();
    }
    
    /// Predict next emotional state based on trajectory
    pub fn predict_next_emotion(&mut self) -> EmotionalVector {
        if self.emotional_trajectory.len() < 2 {
            return self.emotional_vector.clone();
        }
        
        // Simple linear regression prediction
        let len = self.emotional_trajectory.len();
        let last = &self.emotional_trajectory[len - 1];
        let prev = &self.emotional_trajectory[len - 2];
        
        let delta_valence = last.valence - prev.valence;
        let delta_arousal = last.arousal - prev.arousal;
        let delta_dominance = last.dominance - prev.dominance;
        
        let predicted = EmotionalVector {
            valence: (last.valence + delta_valence).clamp(-1.0, 1.0),
            arousal: (last.arousal + delta_arousal).clamp(0.0, 1.0),
            dominance: (last.dominance + delta_dominance).clamp(0.0, 1.0),
            timestamp: near_sdk::env::block_timestamp() + 1000000000, // 1 second in the future
        };
        
        self.predicted_emotion = Some(predicted.clone());
        predicted
    }
    
    /// Update emotional complexity based on trajectory variance
    fn update_emotional_complexity(&mut self) {
        if self.emotional_trajectory.len() < 3 {
            self.emotional_complexity = 0.0;
            return;
        }
        
        let mut valence_variance = 0.0;
        let mut arousal_variance = 0.0;
        let mut dominance_variance = 0.0;
        
        // Calculate mean
        let len = self.emotional_trajectory.len() as f32;
        let mean_valence: f32 = self.emotional_trajectory.iter().map(|e| e.valence).sum::<f32>() / len;
        let mean_arousal: f32 = self.emotional_trajectory.iter().map(|e| e.arousal).sum::<f32>() / len;
        let mean_dominance: f32 = self.emotional_trajectory.iter().map(|e| e.dominance).sum::<f32>() / len;
        
        // Calculate variance
        for emotion in &self.emotional_trajectory {
            valence_variance += (emotion.valence - mean_valence).powi(2);
            arousal_variance += (emotion.arousal - mean_arousal).powi(2);
            dominance_variance += (emotion.dominance - mean_dominance).powi(2);
        }
        
        valence_variance /= len;
        arousal_variance /= len;
        dominance_variance /= len;
        
        // Combine variances to get complexity score (0.0 to 1.0)
        self.emotional_complexity = ((valence_variance + arousal_variance + dominance_variance) / 3.0).min(1.0);
    }
    
    /// Get emotional category based on VAD values
    pub fn get_emotional_category(&self) -> String {
        match (self.valence, self.arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
            (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
            _ => "Calm".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_data_creation() {
        let emotion = EmotionalData::new();
        assert_eq!(emotion.valence, 0.0);
        assert_eq!(emotion.arousal, 0.5);
        assert_eq!(emotion.dominance, 0.5);
    }

    #[test]
    fn test_emotional_data_from_vector() {
        let raw_vector = vec![0.8, 0.9, 0.5];
        let emotion = EmotionalData::from_vector(raw_vector.clone());
        assert_eq!(emotion.raw_vector, raw_vector);
        assert_eq!(emotion.valence, 0.8);
        assert_eq!(emotion.arousal, 0.9);
        assert_eq!(emotion.dominance, 0.5);
    }

    #[test]
    fn test_emotional_vector_creation() {
        let timestamp = near_sdk::env::block_timestamp();
        let vector = EmotionalVector {
            valence: 0.5,
            arousal: 0.6,
            dominance: 0.7,
            timestamp,
        };
        assert_eq!(vector.valence, 0.5);
        assert_eq!(vector.arousal, 0.6);
        assert_eq!(vector.dominance, 0.7);
        assert_eq!(vector.timestamp, timestamp);
    }
    
    #[test]
    fn test_add_to_trajectory() {
        let mut emotion = EmotionalData::new();
        let vector1 = EmotionalVector {
            valence: 0.1,
            arousal: 0.2,
            dominance: 0.3,
            timestamp: near_sdk::env::block_timestamp(),
        };
        let vector2 = EmotionalVector {
            valence: 0.4,
            arousal: 0.5,
            dominance: 0.6,
            timestamp: near_sdk::env::block_timestamp() + 1000,
        };
        
        emotion.add_to_trajectory(vector1.clone());
        emotion.add_to_trajectory(vector2.clone());
        
        assert_eq!(emotion.emotional_trajectory.len(), 2);
        assert_eq!(emotion.emotional_trajectory[0].valence, 0.1);
        assert_eq!(emotion.emotional_trajectory[1].valence, 0.4);
    }
    
    #[test]
    fn test_predict_next_emotion() {
        let mut emotion = EmotionalData::new();
        let vector1 = EmotionalVector {
            valence: 0.1,
            arousal: 0.2,
            dominance: 0.3,
            timestamp: near_sdk::env::block_timestamp(),
        };
        let vector2 = EmotionalVector {
            valence: 0.2,
            arousal: 0.3,
            dominance: 0.4,
            timestamp: near_sdk::env::block_timestamp() + 1000,
        };
        
        emotion.add_to_trajectory(vector1);
        emotion.add_to_trajectory(vector2);
        
        let predicted = emotion.predict_next_emotion();
        assert_eq!(predicted.valence, 0.3);
        assert_eq!(predicted.arousal, 0.4);
        assert_eq!(predicted.dominance, 0.5);
    }
    
    #[test]
    fn test_get_emotional_category() {
        let mut emotion = EmotionalData::new();
        emotion.valence = 0.8;
        emotion.arousal = 0.9;
        assert_eq!(emotion.get_emotional_category(), "Excited");
        
        emotion.valence = 0.8;
        emotion.arousal = 0.3;
        assert_eq!(emotion.get_emotional_category(), "Happy");
        
        emotion.valence = -0.3;
        emotion.arousal = 0.8;
        assert_eq!(emotion.get_emotional_category(), "Anxious");
        
        emotion.valence = -0.3;
        emotion.arousal = 0.3;
        assert_eq!(emotion.get_emotional_category(), "Calm");
    }
}
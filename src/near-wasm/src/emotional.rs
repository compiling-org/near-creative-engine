//! Emotional data structures for interactive NFTs

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::Timestamp;
use sha2::{Sha256, Digest};

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
    /// Data integrity hash for verification
    pub data_hash: String,
    /// Source of emotional data
    pub source: EmotionalDataSource,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
}

/// Source of emotional data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum EmotionalDataSource {
    EEG,
    FacialRecognition,
    HeartRate,
    GSR,
    ManualInput,
    AIInference,
    Combined,
}

impl EmotionalData {
    pub fn new() -> Self {
        let emotional_vector = EmotionalVector {
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
        };
        
        let data_hash = Self::calculate_hash(&emotional_vector, &vec![], &EmotionalDataSource::ManualInput);
        
        Self {
            timestamp: near_sdk::env::block_timestamp(),
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
            confidence: 0.8,
            raw_vector: vec![],
            emotional_vector,
            data_hash,
            source: EmotionalDataSource::ManualInput,
        }
    }
    
    pub fn from_vector(raw_vector: Vec<f32>) -> Self {
        // Simple emotion detection from raw vector
        // In practice, this would use a more sophisticated model
        let valence = if raw_vector.len() > 0 { raw_vector[0].clamp(-1.0, 1.0) } else { 0.0 };
        let arousal = if raw_vector.len() > 1 { raw_vector[1].clamp(0.0, 1.0) } else { 0.5 };
        let dominance = if raw_vector.len() > 2 { raw_vector[2].clamp(0.0, 1.0) } else { 0.5 };
        
        let emotional_vector = EmotionalVector {
            valence,
            arousal,
            dominance,
        };
        
        let data_hash = Self::calculate_hash(&emotional_vector, &raw_vector, &EmotionalDataSource::AIInference);
        
        Self {
            timestamp: near_sdk::env::block_timestamp(),
            valence,
            arousal,
            dominance,
            confidence: 0.8,
            raw_vector,
            emotional_vector,
            data_hash,
            source: EmotionalDataSource::AIInference,
        }
    }
    
    pub fn from_eeg_data(eeg_data: Vec<f32>) -> Self {
        // Process EEG data to extract emotional state
        // This is a simplified implementation
        let valence = if eeg_data.len() > 0 { 
            // Frontal asymmetry as valence indicator
            let left_frontal = *eeg_data.get(0).unwrap_or(&0.0);
            let right_frontal = *eeg_data.get(1).unwrap_or(&0.0);
            ((left_frontal - right_frontal) / (left_frontal + right_frontal + 0.001)).clamp(-1.0, 1.0)
        } else { 0.0 };
        
        let arousal = if eeg_data.len() > 2 {
            // Beta/alpha ratio as arousal indicator
            let beta = *eeg_data.get(2).unwrap_or(&0.0);
            let alpha = *eeg_data.get(3).unwrap_or(&0.0);
            (beta / (alpha + 0.001)).clamp(0.0, 1.0)
        } else { 0.5 };
        
        let dominance = if eeg_data.len() > 4 {
            // Theta/beta ratio as dominance indicator
            let theta = *eeg_data.get(4).unwrap_or(&0.0);
            let beta = *eeg_data.get(2).unwrap_or(&0.0);
            (theta / (beta + 0.001)).clamp(0.0, 1.0)
        } else { 0.5 };
        
        let emotional_vector = EmotionalVector {
            valence,
            arousal,
            dominance,
        };
        
        let data_hash = Self::calculate_hash(&emotional_vector, &eeg_data, &EmotionalDataSource::EEG);
        
        Self {
            timestamp: near_sdk::env::block_timestamp(),
            valence,
            arousal,
            dominance,
            confidence: 0.9, // Higher confidence for EEG data
            raw_vector: eeg_data,
            emotional_vector,
            data_hash,
            source: EmotionalDataSource::EEG,
        }
    }
    
    /// Calculate hash for data integrity verification
    fn calculate_hash(emotional_vector: &EmotionalVector, raw_vector: &[f32], source: &EmotionalDataSource) -> String {
        let data_repr = format!("{:?}{:?}{:?}", emotional_vector, raw_vector, source);
        
        let mut hasher = Sha256::new();
        hasher.update(data_repr.as_bytes());
        let result = hasher.finalize();
        
        format!("{:x}", result)
    }
    
    /// Verify data integrity
    pub fn verify_integrity(&self) -> bool {
        let expected_hash = Self::calculate_hash(&self.emotional_vector, &self.raw_vector, &self.source);
        expected_hash == self.data_hash
    }
    
    /// Update confidence based on data quality
    pub fn update_confidence(&mut self, quality_score: f32) {
        self.confidence = quality_score.clamp(0.0, 1.0);
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
        assert!(!emotion.data_hash.is_empty());
        assert!(emotion.verify_integrity());
    }

    #[test]
    fn test_emotional_data_from_vector() {
        let raw_vector = vec![0.8, 0.9, 0.5];
        let emotion = EmotionalData::from_vector(raw_vector.clone());
        assert_eq!(emotion.raw_vector, raw_vector);
        assert_eq!(emotion.valence, 0.8);
        assert_eq!(emotion.arousal, 0.9);
        assert_eq!(emotion.dominance, 0.5);
        assert!(emotion.verify_integrity());
    }

    #[test]
    fn test_emotional_data_from_eeg() {
        let eeg_data = vec![0.7, 0.3, 0.8, 0.4, 0.6];
        let emotion = EmotionalData::from_eeg_data(eeg_data.clone());
        assert_eq!(emotion.raw_vector, eeg_data);
        assert!(emotion.valence >= -1.0 && emotion.valence <= 1.0);
        assert!(emotion.arousal >= 0.0 && emotion.arousal <= 1.0);
        assert!(emotion.dominance >= 0.0 && emotion.dominance <= 1.0);
        assert!(emotion.verify_integrity());
    }

    #[test]
    fn test_emotional_vector_creation() {
        let vector = EmotionalVector {
            valence: 0.5,
            arousal: 0.6,
            dominance: 0.7,
        };
        assert_eq!(vector.valence, 0.5);
        assert_eq!(vector.arousal, 0.6);
        assert_eq!(vector.dominance, 0.7);
    }
    
    #[test]
    fn test_data_integrity() {
        let mut emotion = EmotionalData::new();
        assert!(emotion.verify_integrity());
        
        // Tamper with data
        emotion.valence = 0.9;
        assert!(!emotion.verify_integrity());
    }
}
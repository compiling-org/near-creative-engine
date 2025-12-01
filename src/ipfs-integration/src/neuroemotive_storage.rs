//! Neuroemotive Storage - IPFS integration for emotional AI data
//!
//! Handles storage of emotional trajectories, diffusion generations, and creative sessions
//! Enhanced with advanced emotional computing capabilities

use crate::ipfs_client::IpfsClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

/// Emotional state vector (Valence-Arousal-Dominance model)
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct EmotionalVector {
    pub valence: f32,      // pleasure vs displeasure (-1.0 to 1.0)
    pub arousal: f32,      // calm vs excited (0.0 to 1.0)
    pub dominance: f32,    // controlled vs in-control (0.0 to 1.0)
    // Enhanced fields for advanced emotional computing
    pub emotional_category: String, // Emotional category based on VAD values
    pub emotional_complexity: f32,  // Complexity of emotional state (0.0-1.0)
}

/// Compressed emotional state for efficient storage
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CompressedEmotionalState {
    pub timestamp_offset: u32, // offset from start_time in milliseconds
    pub v: i8,                  // valence * 100 compressed to i8
    pub a: u8,                  // arousal * 100 compressed to u8
    pub d: u8,                  // dominance * 100 compressed to u8
}

/// Diffusion frame with emotional metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DiffusionFrame {
    pub frame_id: String,
    pub timestamp: u64,
    pub emotional_state: EmotionalVector,
    pub prompt_conditioning: String,
    pub image_cid: String,
    pub generation_parameters: HashMap<String, serde_json::Value>,
}

/// Emotional trajectory - sequence of emotional states
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmotionalTrajectory {
    pub trajectory_id: String,
    pub creator: String,
    pub start_time: u64,
    pub compressed_states: Vec<CompressedEmotionalState>,
    pub metadata: TrajectoryMetadata,
    // Enhanced fields
    pub predicted_next_state: Option<EmotionalVector>,
    pub prediction_confidence: f32,
    pub emotional_complexity: f32, // Overall complexity of the trajectory
}

/// Trajectory metadata
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TrajectoryMetadata {
    pub session_type: String,
    pub participant_count: u8,
    pub environment: String,
    pub tags: Vec<String>,
    // Enhanced fields
    pub dominant_emotions: Vec<String>,
    pub emotional_volatility: f32,
}

/// Neuroemotive creative session
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NeuroemotiveSession {
    pub session_id: String,
    pub creator: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub emotional_states: Vec<EmotionalVector>,
    pub frames: Vec<DiffusionFrame>,
    pub performance_data: PerformanceData,
    // Enhanced fields for advanced emotional computing
    pub emotional_complexity: f32,
    pub creativity_index: f32,
    pub engagement_score: f32,
    pub session_traits: HashMap<String, f32>,
}

/// Performance data for session
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PerformanceData {
    pub total_frames: u64,
    pub avg_generation_time_ms: f32,
    pub total_emotional_states: u64,
    pub storage_size_bytes: u64,
}

impl EmotionalVector {
    /// Create a new emotional vector
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        let emotional_category = Self::get_emotional_category(valence, arousal, dominance);
        let emotional_complexity = Self::calculate_emotional_complexity(valence, arousal, dominance);
        
        Self {
            valence,
            arousal,
            dominance,
            emotional_category,
            emotional_complexity,
        }
    }
    
    /// Get emotional category based on VAD values
    pub fn get_emotional_category(valence: f32, arousal: f32, dominance: f32) -> String {
        match (valence, arousal, dominance) {
            (v, a, d) if v > 0.5 && a > 0.5 && d > 0.5 => "Excited".to_string(),
            (v, a, d) if v > 0.5 && a > 0.5 && d <= 0.5 => "Happy".to_string(),
            (v, a, d) if v > 0.5 && a <= 0.5 && d > 0.5 => "Proud".to_string(),
            (v, a, d) if v > 0.5 && a <= 0.5 && d <= 0.5 => "Calm".to_string(),
            (v, a, d) if v <= 0.5 && a > 0.5 && d > 0.5 => "Angry".to_string(),
            (v, a, d) if v <= 0.5 && a > 0.5 && d <= 0.5 => "Anxious".to_string(),
            (v, a, d) if v <= 0.5 && a <= 0.5 && d > 0.5 => "Bored".to_string(),
            _ => "Sad".to_string(),
        }
    }
    
    /// Calculate emotional complexity based on VAD values
    pub fn calculate_emotional_complexity(valence: f32, arousal: f32, dominance: f32) -> f32 {
        // Calculate the distance from the emotional center (0.0, 0.5, 0.5)
        let center_distance = ((valence - 0.0).powi(2) + (arousal - 0.5).powi(2) + (dominance - 0.5).powi(2)).sqrt();
        
        // Normalize to 0.0-1.0 range
        (center_distance / 1.5).min(1.0)
    }
    
    /// Calculate emotional distance between two states
    pub fn distance(&self, other: &EmotionalVector) -> f32 {
        let dv = self.valence - other.valence;
        let da = self.arousal - other.arousal;
        let dd = self.dominance - other.dominance;
        (dv * dv + da * da + dd * dd).sqrt()
    }
    
    /// Check if emotional state is valid
    pub fn is_valid(&self) -> bool {
        self.valence >= -1.0 && self.valence <= 1.0
            && self.arousal >= 0.0 && self.arousal <= 1.0
            && self.dominance >= 0.0 && self.dominance <= 1.0
    }
    
    /// Compress to efficient storage format
    pub fn compress(&self, timestamp_offset: u32) -> CompressedEmotionalState {
        CompressedEmotionalState {
            timestamp_offset,
            v: (self.valence * 100.0) as i8,
            a: (self.arousal * 100.0) as u8,
            d: (self.dominance * 100.0) as u8,
        }
    }
}

impl CompressedEmotionalState {
    /// Decompress to full emotional state
    pub fn decompress(&self, base_timestamp: u64) -> (u64, EmotionalVector) {
        let timestamp = base_timestamp + (self.timestamp_offset as u64);
        let emotional_vector = EmotionalVector::new(
            (self.v as f32) / 100.0,
            (self.a as f32) / 100.0,
            (self.d as f32) / 100.0,
        );
        (timestamp, emotional_vector)
    }
}

impl EmotionalTrajectory {
    /// Create a new emotional trajectory
    pub fn new(trajectory_id: String, creator: String) -> Self {
        Self {
            trajectory_id,
            creator,
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            compressed_states: Vec::new(),
            metadata: TrajectoryMetadata::default(),
            predicted_next_state: None,
            prediction_confidence: 0.0,
            emotional_complexity: 0.0,
        }
    }
    
    /// Add compressed emotional state to trajectory
    pub fn add_state(&mut self, state: CompressedEmotionalState) {
        self.compressed_states.push(state);
        self.calculate_trajectory_metrics();
    }
    
    /// Calculate trajectory metrics
    pub fn calculate_trajectory_metrics(&mut self) {
        if self.compressed_states.is_empty() {
            return;
        }
        
        // Calculate emotional volatility
        if self.compressed_states.len() > 1 {
            let mut sum_variance = 0.0;
            for i in 1..self.compressed_states.len() {
                let prev = &self.compressed_states[i-1];
                let curr = &self.compressed_states[i];
                
                let dv = (curr.v - prev.v) as f32;
                let da = (curr.a as i16 - prev.a as i16) as f32;
                let dd = (curr.d as i16 - prev.d as i16) as f32;
                
                sum_variance += (dv * dv + da * da + dd * dd).sqrt();
            }
            
            self.metadata.emotional_volatility = sum_variance / (self.compressed_states.len() - 1) as f32;
        }
        
        // Identify dominant emotions
        let mut emotion_counts = HashMap::new();
        for state in &self.compressed_states {
            // Decompress to get emotional category
            let valence = (state.v as f32) / 100.0;
            let arousal = (state.a as f32) / 100.0;
            let dominance = (state.d as f32) / 100.0;
            
            let category = match (valence, arousal, dominance) {
                (v, a, _d) if v > 0.5 && a > 0.5 => "Excited".to_string(),
                (v, a, _d) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
                (v, a, _d) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
                _ => "Calm".to_string(),
            };
            
            *emotion_counts.entry(category).or_insert(0) += 1;
        }
        
        // Find most frequent emotions
        let mut sorted_emotions: Vec<(String, i32)> = emotion_counts.into_iter().collect();
        sorted_emotions.sort_by(|a, b| b.1.cmp(&a.1));
        
        self.metadata.dominant_emotions = sorted_emotions
            .into_iter()
            .take(3)
            .map(|(emotion, _)| emotion)
            .collect();
            
        // Calculate trajectory complexity
        self.emotional_complexity = self.calculate_trajectory_complexity();
    }
    
    /// Calculate emotional trajectory complexity
    pub fn calculate_trajectory_complexity(&self) -> f32 {
        if self.compressed_states.len() < 3 {
            return 0.0;
        }
        
        // Calculate the complexity based on direction changes and variance
        let mut direction_changes = 0;
        let mut total_distance = 0.0;
        
        for i in 1..self.compressed_states.len() {
            let prev = &self.compressed_states[i-1];
            let curr = &self.compressed_states[i];
            
            // Calculate distance between consecutive states
            let dv = (curr.v - prev.v) as f32;
            let da = (curr.a as i16 - prev.a as i16) as f32;
            let dd = (curr.d as i16 - prev.d as i16) as f32;
            
            let distance = (dv * dv + da * da + dd * dd).sqrt();
            total_distance += distance;
            
            // Check for significant direction changes in valence
            if i > 1 {
                let prev_prev = &self.compressed_states[i-2];
                let prev_dv = (prev.v - prev_prev.v) as f32;
                
                // If direction changed significantly
                if (dv * prev_dv) < -10.0 {
                    direction_changes += 1;
                }
            }
        }
        
        // Normalize complexity score (0-1)
        let avg_distance = total_distance / (self.compressed_states.len() - 1) as f32;
        let change_ratio = direction_changes as f32 / self.compressed_states.len() as f32;
        
        (avg_distance / 100.0 + change_ratio).min(1.0)
    }
    
    /// Predict next emotional state in the trajectory
    pub fn predict_next_state(&mut self) -> Option<EmotionalVector> {
        if self.compressed_states.len() < 3 {
            return None;
        }
        
        // Simple prediction based on last few states
        let len = self.compressed_states.len();
        let last_state = &self.compressed_states[len-1];
        let prev_state = &self.compressed_states[len-2];
        
        let delta_v = last_state.v as i16 - prev_state.v as i16;
        let delta_a = last_state.a as i16 - prev_state.a as i16;
        let delta_d = last_state.d as i16 - prev_state.d as i16;
        
        let predicted_v = last_state.v as i16 + delta_v;
        let predicted_a = last_state.a as i16 + delta_a;
        let predicted_d = last_state.d as i16 + delta_d;
        
        // Confidence decreases with prediction distance
        self.prediction_confidence = 0.8 - (len as f32 * 0.05).min(0.7);
        
        Some(EmotionalVector {
            valence: (predicted_v as f32) / 100.0,
            arousal: (predicted_a as f32) / 100.0,
            dominance: (predicted_d as f32) / 100.0,
            emotional_category: EmotionalVector::get_emotional_category(
                (predicted_v as f32) / 100.0,
                (predicted_a as f32) / 100.0,
                (predicted_d as f32) / 100.0,
            ),
            emotional_complexity: EmotionalVector::calculate_emotional_complexity(
                (predicted_v as f32) / 100.0,
                (predicted_a as f32) / 100.0,
                (predicted_d as f32) / 100.0,
            ),
        })
    }
}

impl NeuroemotiveSession {
    /// Create a new neuroemotive session
    pub fn new(session_id: String, creator: String) -> Self {
        Self {
            session_id,
            creator,
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            end_time: None,
            emotional_states: Vec::new(),
            frames: Vec::new(),
            performance_data: PerformanceData::default(),
            emotional_complexity: 0.0,
            creativity_index: 0.5,
            engagement_score: 0.0,
            session_traits: HashMap::new(),
        }
    }
    
    /// Add emotional state to session
    pub fn add_emotional_state(&mut self, state: EmotionalVector) {
        self.emotional_states.push(state);
        self.performance_data.total_emotional_states = self.emotional_states.len() as u64;
        
        // Update emotional complexity
        self.emotional_complexity = self.calculate_emotional_complexity();
        
        // Update creativity index based on emotional variance
        self.creativity_index = self.calculate_creativity_index();
    }
    
    /// Add frame to session
    pub fn add_frame(&mut self, frame: DiffusionFrame) {
        self.frames.push(frame);
        self.performance_data.total_frames = self.frames.len() as u64;
    }
    
    /// Finalize session
    pub fn finalize(&mut self) {
        self.end_time = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
    }
    
    /// Calculate average emotional state
    pub fn average_emotional_state(&self) -> EmotionalVector {
        if self.emotional_states.is_empty() {
            return EmotionalVector::default();
        }

        let sum = self.emotional_states.iter().fold(
            EmotionalVector::default(),
            |acc, state| EmotionalVector {
                valence: acc.valence + state.valence,
                arousal: acc.arousal + state.arousal,
                dominance: acc.dominance + state.dominance,
                emotional_category: acc.emotional_category.clone(),
                emotional_complexity: acc.emotional_complexity + state.emotional_complexity,
            },
        );

        let count = self.emotional_states.len() as f32;
        EmotionalVector {
            valence: sum.valence / count,
            arousal: sum.arousal / count,
            dominance: sum.dominance / count,
            emotional_category: EmotionalVector::get_emotional_category(
                sum.valence / count,
                sum.arousal / count,
                sum.dominance / count,
            ),
            emotional_complexity: sum.emotional_complexity / count,
        }
    }

    /// Get emotional variance (measure of emotional volatility)
    pub fn emotional_variance(&self) -> f32 {
        if self.emotional_states.len() < 2 {
            return 0.0;
        }

        let avg = self.average_emotional_state();
        let variance_sum: f32 = self.emotional_states.iter().map(|state| {
            let dv = state.valence - avg.valence;
            let da = state.arousal - avg.arousal;
            let dd = state.dominance - avg.dominance;
            dv * dv + da * da + dd * dd
        }).sum();

        variance_sum / self.emotional_states.len() as f32
    }
    
    /// Calculate emotional complexity based on variance and trajectory
    pub fn calculate_emotional_complexity(&self) -> f32 {
        if self.emotional_states.len() < 2 {
            return 0.0;
        }
        
        // Use variance as the base complexity measure
        let variance = self.emotional_variance();
        
        // Add trajectory complexity (direction changes)
        let mut direction_changes = 0;
        for i in 1..self.emotional_states.len() {
            let prev = &self.emotional_states[i-1];
            let curr = &self.emotional_states[i];
            
            // Check for significant direction changes in valence
            if (curr.valence - prev.valence).abs() > 0.3 {
                direction_changes += 1;
            }
        }
        
        let trajectory_complexity = direction_changes as f32 / self.emotional_states.len() as f32;
        
        // Combine variance and trajectory complexity (0-1 scale)
        (variance + trajectory_complexity).min(1.0)
    }
    
    /// Calculate creativity index based on emotional dynamics
    pub fn calculate_creativity_index(&self) -> f32 {
        if self.emotional_states.len() < 2 {
            return 0.5; // Default neutral creativity
        }
        
        // Creativity is higher with moderate variance and complexity
        let variance = self.emotional_variance();
        let complexity = self.emotional_complexity;
        
        // Optimal creativity is around variance = 0.3-0.6
        let variance_score = 1.0 - ((variance - 0.45).abs() / 0.45).min(1.0);
        
        // Combine with complexity (0-1 scale)
        (variance_score * 0.7 + complexity * 0.3).min(1.0)
    }
    
    /// Update engagement score based on interaction patterns
    pub fn update_engagement_score(&mut self, interaction_intensity: f32) {
        // Simple engagement calculation based on interaction intensity and frequency
        let frequency_factor = (self.performance_data.total_emotional_states as f32 / 100.0).min(1.0);
        self.engagement_score = (interaction_intensity * 0.7 + frequency_factor * 0.3).min(1.0);
    }
    
    /// Store session to IPFS
    pub async fn store_to_ipfs(&self, client: &IpfsClient) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        client.add_json(&json).await
    }
}
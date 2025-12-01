//! # Comprehensive Integration Module
//!
//! This module provides a unified interface that integrates music generation,
//! LanceDB vector search, and real AI inference to create a complete
//! creative computing system for the NFT blockchain interactive framework.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use wasm_bindgen::prelude::*;

/// Comprehensive creative session that integrates all components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComprehensiveCreativeSession {
    pub session_id: String,
    pub start_time: DateTime<Utc>,
    pub ai_engine: crate::RealAIInferenceEngine,
    pub music_engine: crate::MusicEngine,
    pub vector_engine: crate::LanceDBEngine,
    pub emotional_data: Option<crate::EmotionalData>,
    pub creative_output: CreativeOutput,
    pub blockchain_integrations: Vec<BlockchainIntegration>,
    #[cfg(feature = "audio")]
    pub audio_playback_active: bool,
}

/// Creative output from the comprehensive session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeOutput {
    pub music_tracks: Vec<crate::GeneratedMusic>,
    pub ai_insights: Vec<AIInsight>,
    pub vector_embeddings: Vec<VectorEmbedding>,
    pub emotional_trajectory: Vec<EmotionalPoint>,
    pub creative_parameters: HashMap<String, f32>,
}

/// AI insight from inference
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIInsight {
    pub insight_type: String,
    pub confidence: f32,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
}

/// Vector embedding for creative content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorEmbedding {
    pub embedding_type: String,
    pub vector: Vec<f32>,
    pub metadata: HashMap<String, serde_json::Value>,
    pub timestamp: DateTime<Utc>,
}

/// Emotional point in the creative journey
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPoint {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub timestamp: DateTime<Utc>,
}

/// Blockchain integration data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockchainIntegration {
    pub blockchain: String,
    pub contract_address: String,
    pub token_id: Option<String>,
    pub metadata_uri: Option<String>,
    pub transaction_hash: Option<String>,
    pub integration_status: String,
}

impl ComprehensiveCreativeSession {
    /// Create a new comprehensive creative session
    pub fn new() -> Self {
        Self {
            session_id: uuid::Uuid::new_v4().to_string(),
            start_time: Utc::now(),
            ai_engine: crate::RealAIInferenceEngine::new(),
            music_engine: crate::MusicEngine::new(),
            vector_engine: crate::LanceDBEngine::new(),
            emotional_data: None,
            creative_output: CreativeOutput {
                music_tracks: Vec::new(),
                ai_insights: Vec::new(),
                vector_embeddings: Vec::new(),
                emotional_trajectory: Vec::new(),
                creative_parameters: HashMap::new(),
            },
            blockchain_integrations: Vec::new(),
        }
    }

    /// Initialize all engines in the session
    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Initialize AI engine
        self.ai_engine.initialize().await?;
        
        // Initialize vector engine
        self.vector_engine.initialize().await?;
        
        Ok(())
    }
    
    /// Play the latest generated music track
    #[cfg(feature = "audio")]
    pub fn play_latest_music(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(latest_track) = self.creative_output.music_tracks.last() {
            self.music_engine.play_music(latest_track)?;
            self.audio_playback_active = true;
            Ok(())
        } else {
            Err("No music tracks available to play".into())
        }
    }
    
    /// Stop audio playback
    #[cfg(feature = "audio")]
    pub fn stop_audio(&mut self) {
        self.music_engine.stop();
        self.audio_playback_active = false;
    }
    
    /// Check if audio is currently playing
    #[cfg(feature = "audio")]
    pub fn is_audio_playing(&self) -> bool {
        self.music_engine.is_playing()
    }

    /// Process emotional input and generate creative content
    pub async fn process_emotional_input(&mut self, valence: f32, arousal: f32, dominance: f32, image_data: Option<&[u8]>) -> Result<CreativeOutput, Box<dyn std::error::Error>> {
        // Create emotional data
        let emotional_data = crate::generate_emotional_data(valence, arousal, dominance, vec![valence, arousal, dominance]);
        self.emotional_data = Some(emotional_data.clone());

        // Add to emotional trajectory
        self.creative_output.emotional_trajectory.push(EmotionalPoint {
            valence,
            arousal,
            dominance,
            timestamp: Utc::now(),
        });

        // Generate AI insights
        if let Some(image_data) = image_data {
            let ai_result = self.ai_engine.detect_emotion_from_image(image_data).await?;
            self.creative_output.ai_insights.push(AIInsight {
                insight_type: "emotion_detection".to_string(),
                confidence: ai_result.confidence,
                data: serde_json::json!({
                    "emotion": ai_result.emotion,
                    "valence": ai_result.valence,
                    "arousal": ai_result.arousal,
                    "dominance": ai_result.dominance,
                    "processing_time_ms": ai_result.processing_time_ms,
                }),
                timestamp: Utc::now(),
            });
        }

        // Generate creative parameters using AI
        let creative_result = self.ai_engine.generate_creative_parameters(&emotional_data).await?;
        self.creative_output.creative_parameters = creative_result.parameters.clone();
        
        self.creative_output.ai_insights.push(AIInsight {
            insight_type: "creative_generation".to_string(),
            confidence: creative_result.confidence,
            data: serde_json::json!({
                "creative_type": creative_result.creative_type,
                "parameters": creative_result.parameters,
                "style_vector_length": creative_result.style_vector.len(),
                "processing_time_ms": creative_result.processing_time_ms,
            }),
            timestamp: Utc::now(),
        });

        // Generate music based on emotions
        let emotional_input = crate::EmotionalInput {
            valence,
            arousal,
            dominance,
        };
        
        let music_result = self.music_engine.generate_music_from_emotion(emotional_input)?;
        self.creative_output.music_tracks.push(music_result.clone());

        // Create vector embeddings
        let vector_embedding = self.create_emotional_embedding(&emotional_data, &music_result);
        self.creative_output.vector_embeddings.push(vector_embedding.clone());

        // Store in LanceDB
        let vector_data = self.vector_engine.create_emotional_vector(&emotional_data, &self.session_id, &music_result.id);
        let vector_id = self.vector_engine.insert_emotional_vector(vector_data).await?;
        
        // Add metadata to creative output
        self.creative_output.creative_parameters.insert("vector_id".to_string(), vector_id.len() as f32);
        self.creative_output.creative_parameters.insert("music_track_count".to_string(), self.creative_output.music_tracks.len() as f32);

        Ok(self.creative_output.clone())
    }

    /// Create emotional vector embedding
    fn create_emotional_embedding(&self, emotional_data: &crate::EmotionalData, music_result: &crate::GeneratedMusic) -> VectorEmbedding {
        // Create a rich embedding that combines emotional data and music characteristics
        let mut vector = vec![
            emotional_data.valence,
            emotional_data.arousal,
            emotional_data.dominance,
            emotional_data.confidence,
            emotional_data.emotional_complexity,
        ];

        // Add music characteristics
        vector.push(music_result.config.tempo / 200.0); // Normalize tempo
        vector.push(match music_result.config.key.as_str() {
            "C" => 0.1, "D" => 0.2, "E" => 0.3, "F" => 0.4, "G" => 0.5, "A" => 0.6, "B" => 0.7, _ => 0.0,
        });
        vector.push(music_result.config.complexity);

        // Add temporal information
        let time_features = self.extract_temporal_features();
        vector.extend(time_features);

        let mut metadata = HashMap::new();
        metadata.insert("emotional_category".to_string(), serde_json::json!(&emotional_data.emotional_category));
        metadata.insert("music_tempo".to_string(), serde_json::json!(music_result.config.tempo));
        metadata.insert("music_key".to_string(), serde_json::json!(&music_result.config.key));
        metadata.insert("session_id".to_string(), serde_json::json!(&self.session_id));

        VectorEmbedding {
            embedding_type: "emotional_music".to_string(),
            vector,
            metadata,
            timestamp: Utc::now(),
        }
    }

    /// Extract temporal features from the session
    fn extract_temporal_features(&self) -> Vec<f32> {
        let session_duration = Utc::now().signed_duration_since(self.start_time).num_seconds() as f32;
        let time_of_day = Utc::now().hour() as f32 / 24.0;
        let day_of_week = Utc::now().weekday().num_days_from_monday() as f32 / 7.0;

        vec![session_duration / 3600.0, time_of_day, day_of_week]
    }

    /// Search for similar creative content
    pub async fn find_similar_content(&self, query_vector: Vec<f32>, limit: usize) -> Result<Vec<crate::VectorSearchResult>, Box<dyn std::error::Error>> {
        let results = self.vector_engine.search_emotional_vectors(query_vector, limit, Some(self.session_id.clone())).await?;
        Ok(results)
    }

    /// Integrate with blockchain
    pub async fn integrate_with_blockchain(&mut self, blockchain: &str, contract_address: &str, metadata_uri: &str) -> Result<String, Box<dyn std::error::Error>> {
        let integration = BlockchainIntegration {
            blockchain: blockchain.to_string(),
            contract_address: contract_address.to_string(),
            token_id: None,
            metadata_uri: Some(metadata_uri.to_string()),
            transaction_hash: None,
            integration_status: "pending".to_string(),
        };

        let integration_id = uuid::Uuid::new_v4().to_string();
        self.blockchain_integrations.push(integration);

        Ok(integration_id)
    }

    /// Generate comprehensive metadata for the session
    pub fn generate_session_metadata(&self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let metadata = serde_json::json!({
            "session_id": self.session_id,
            "start_time": self.start_time.to_rfc3339(),
            "duration_seconds": Utc::now().signed_duration_since(self.start_time).num_seconds(),
            "emotional_data": self.emotional_data,
            "creative_output": {
                "music_tracks_count": self.creative_output.music_tracks.len(),
                "ai_insights_count": self.creative_output.ai_insights.len(),
                "vector_embeddings_count": self.creative_output.vector_embeddings.len(),
                "emotional_trajectory_length": self.creative_output.emotional_trajectory.len(),
                "creative_parameters": self.creative_output.creative_parameters,
            },
            "blockchain_integrations": self.blockchain_integrations.len(),
            "ai_engine_config": {
                "model_type": self.ai_engine.config.model_type,
                "device_type": self.ai_engine.config.device_type,
                "quantization": self.ai_engine.config.quantization,
            },
            "music_engine_config": {
                "tempo": self.music_engine.config.tempo,
                "key": self.music_engine.config.key,
                "scale": self.music_engine.config.scale,
                "complexity": self.music_engine.config.complexity,
            },
            "vector_engine_stats": self.vector_engine.get_stats(),
        });

        Ok(metadata)
    }

    /// Export session for blockchain storage
    pub fn export_for_blockchain(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let metadata = self.generate_session_metadata()?;
        let export_data = serde_json::json!({
            "type": "comprehensive_creative_session",
            "version": "1.0",
            "metadata": metadata,
            "export_timestamp": Utc::now().to_rfc3339(),
        });

        Ok(serde_json::to_vec(&export_data)?)
    }
}

impl Default for ComprehensiveCreativeSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a comprehensive creative session
pub fn create_comprehensive_session() -> ComprehensiveCreativeSession {
    ComprehensiveCreativeSession::new()
}

/// Process emotional input through the comprehensive pipeline
pub async fn process_emotional_pipeline(
    valence: f32,
    arousal: f32,
    dominance: f32,
    image_data: Option<&[u8]>,
) -> Result<CreativeOutput, Box<dyn std::error::Error>> {
    let mut session = ComprehensiveCreativeSession::new();
    session.initialize().await?;
    session.process_emotional_input(valence, arousal, dominance, image_data).await
}

/// WASM-exposed functions for comprehensive integration
#[wasm_bindgen]
pub async fn create_comprehensive_creative_session() -> Result<String, JsValue> {
    let session = ComprehensiveCreativeSession::new();
    serde_json::to_string(&session.session_id)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub async fn process_emotional_input_comprehensive(
    valence: f32,
    arousal: f32,
    dominance: f32,
    image_data: Option<Vec<u8>>,
) -> Result<String, JsValue> {
    let image_ref = image_data.as_deref();
    let result = process_emotional_pipeline(valence, arousal, dominance, image_ref).await
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn generate_session_metadata_comprehensive(session_id: &str) -> Result<String, JsValue> {
    // Create a mock session for metadata generation
    let mut session = ComprehensiveCreativeSession::new();
    session.session_id = session_id.to_string();
    
    let metadata = session.generate_session_metadata()
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    serde_json::to_string(&metadata)
        .map_err(|e| JsValue::from_str(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_session_creation() {
        let session = ComprehensiveCreativeSession::new();
        assert!(!session.session_id.is_empty());
        assert!(session.creative_output.music_tracks.is_empty());
        assert!(session.creative_output.ai_insights.is_empty());
    }

    #[tokio::test]
    async fn test_emotional_pipeline_processing() {
        let result = process_emotional_pipeline(0.8, 0.6, 0.7, None).await.unwrap();
        assert!(!result.music_tracks.is_empty());
        assert!(!result.ai_insights.is_empty());
        assert!(!result.vector_embeddings.is_empty());
        assert!(!result.emotional_trajectory.is_empty());
    }

    #[test]
    fn test_emotional_embedding_creation() {
        let session = ComprehensiveCreativeSession::new();
        let emotional_data = crate::generate_emotional_data(0.5, 0.5, 0.5, vec![0.1, 0.2, 0.3]);
        let music_result = crate::GeneratedMusic {
            id: "test".to_string(),
            timestamp: Utc::now(),
            config: crate::MusicConfig::default(),
            emotional_input: crate::EmotionalInput {
                valence: 0.5,
                arousal: 0.5,
                dominance: 0.5,
            },
            audio_data: vec![1, 2, 3],
            metadata: HashMap::new(),
        };
        
        let embedding = session.create_emotional_embedding(&emotional_data, &music_result);
        assert_eq!(embedding.embedding_type, "emotional_music");
        assert!(!embedding.vector.is_empty());
        assert!(!embedding.metadata.is_empty());
    }
}
//! # Multifusion Integration Module
//!
//! Comprehensive integration of music generation, AI inference, and vector search
//! across multiple blockchain networks with emotional context and cross-chain bridging.
//!
//! Enhanced with patterns from reference implementations including:
//! - bevy_audio for music processing and emotional mapping
//! - candle BERT for semantic embeddings and vector search
//! - Cross-chain bridge patterns from polkadot and near references

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[cfg(feature = "audio")]
use crate::music_integration::{MusicEngine, MusicConfig};
#[cfg(feature = "db")]
use crate::lancedb_integration::{LanceDBEngine, VectorSearchResult};
#[cfg(feature = "ai-ml")]
use crate::real_ai_inference::{AIInferenceEngine, InferenceConfig};

/// Configuration for multifusion operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultifusionConfig {
    pub supported_chains: Vec<String>,
    pub fusion_strategies: HashMap<String, FusionStrategy>,
    pub cross_chain_bridge_enabled: bool,
    pub vector_search_enabled: bool,
    pub music_generation_enabled: bool,
    pub ai_inference_enabled: bool,
    pub emotional_context_weight: f32,
    pub creativity_boost_factor: f32,
}

/// Strategy for fusing different blockchain assets and data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionStrategy {
    pub name: String,
    pub primary_chain: String,
    pub secondary_chains: Vec<String>,
    pub fusion_algorithm: FusionAlgorithm,
    pub emotional_weight: f32,
    pub creativity_multiplier: f32,
    pub vector_dimensions: usize,
}

/// Different algorithms for asset fusion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionAlgorithm {
    WeightedAverage,
    EmotionalContextual,
    VectorSimilarity,
    CreativeBlending,
    CrossChainHybrid,
}

/// Active fusion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveFusion {
    pub fusion_id: String,
    pub primary_asset: CrossChainAsset,
    pub secondary_assets: Vec<CrossChainAsset>,
    pub fusion_strategy: FusionStrategy,
    pub emotional_context: EmotionalContext,
    pub start_time: DateTime<Utc>,
    pub status: FusionStatus,
    pub progress: f32,
    pub result: Option<FusionResult>,
}

/// Cross-chain asset representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainAsset {
    pub asset_id: String,
    pub blockchain: String,
    pub contract_address: String,
    pub token_id: String,
    pub metadata: HashMap<String, serde_json::Value>,
    pub emotional_vector: Option<Vec<f32>>,
    pub creative_score: f32,
    pub vector_embedding: Option<Vec<f32>>,
}

/// Emotional context for fusion operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalContext {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub complexity: f32,
    pub category: String,
    pub trajectory: Vec<EmotionalPoint>,
}

/// Point in emotional trajectory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalPoint {
    pub valence: f32,
    pub arousal: f32,
    pub timestamp: DateTime<Utc>,
}

/// Status of fusion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FusionStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

/// Result of fusion operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionResult {
    pub fused_asset: CrossChainAsset,
    pub emotional_synthesis: EmotionalSynthesis,
    pub creative_amplification: CreativeAmplification,
    pub vector_unification: VectorUnification,
    pub completion_time: DateTime<Utc>,
}

/// Emotional synthesis results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalSynthesis {
    pub synthesized_vector: Vec<f32>,
    pub emotional_categories: Vec<String>,
    pub complexity_score: f32,
    pub harmony_score: f32,
}

/// Creative amplification results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeAmplification {
    pub amplification_factor: f32,
    pub novel_elements: Vec<String>,
    pub aesthetic_score: f32,
    pub innovation_index: f32,
}

/// Vector unification results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorUnification {
    pub unified_embedding: Vec<f32>,
    pub similarity_score: f32,
    pub coherence_score: f32,
    pub dimensional_balance: f32,
}

/// Metrics for fusion operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FusionMetrics {
    pub total_fusions: u64,
    pub successful_fusions: u64,
    pub failed_fusions: u64,
    pub average_emotional_harmony: f32,
    pub average_creative_amplification: f32,
    pub cross_chain_success_rate: f32,
    pub vector_search_accuracy: f32,
}

/// Main multifusion engine
pub struct MultifusionEngine {
    pub config: MultifusionConfig,
    pub active_sessions: HashMap<String, MultifusionSession>,
    pub fusion_history: Vec<FusionResult>,
    pub metrics: FusionMetrics,
    #[cfg(feature = "audio")]
    pub music_engine: Option<MusicEngine>,
    #[cfg(feature = "db")]
    pub vector_engine: Option<LanceDBEngine>,
    #[cfg(feature = "ai-ml")]
    pub ai_engine: Option<AIInferenceEngine>,
}

/// Multifusion session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultifusionSession {
    pub session_id: String,
    pub config: MultifusionConfig,
    pub active_fusions: Vec<ActiveFusion>,
    pub cross_chain_assets: Vec<CrossChainAsset>,
    pub fusion_metrics: FusionMetrics,
    pub created_at: DateTime<Utc>,
}

impl MultifusionEngine {
    /// Create a new multifusion engine
    pub fn new(config: MultifusionConfig) -> Self {
        Self {
            config: config.clone(),
            active_sessions: HashMap::new(),
            fusion_history: Vec::new(),
            metrics: FusionMetrics {
                total_fusions: 0,
                successful_fusions: 0,
                failed_fusions: 0,
                average_emotional_harmony: 0.0,
                average_creative_amplification: 0.0,
                cross_chain_success_rate: 0.0,
                vector_search_accuracy: 0.0,
            },
            #[cfg(feature = "audio")]
            music_engine: None,
            #[cfg(feature = "db")]
            vector_engine: None,
            #[cfg(feature = "ai-ml")]
            ai_engine: None,
        }
    }

    /// Initialize with feature-specific engines
    #[cfg(feature = "audio")]
    pub fn with_music_engine(mut self, music_engine: MusicEngine) -> Self {
        self.music_engine = Some(music_engine);
        self
    }

    #[cfg(feature = "db")]
    pub fn with_vector_engine(mut self, vector_engine: LanceDBEngine) -> Self {
        self.vector_engine = Some(vector_engine);
        self
    }

    #[cfg(feature = "ai-ml")]
    pub fn with_ai_engine(mut self, ai_engine: AIInferenceEngine) -> Self {
        self.ai_engine = Some(ai_engine);
        self
    }

    /// Create a new multifusion session
    pub fn create_session(&mut self, session_id: String) -> Result<String, Box<dyn std::error::Error>> {
        let session = MultifusionSession {
            session_id: session_id.clone(),
            config: self.config.clone(),
            active_fusions: Vec::new(),
            cross_chain_assets: Vec::new(),
            fusion_metrics: self.metrics.clone(),
            created_at: Utc::now(),
        };

        self.active_sessions.insert(session_id.clone(), session);
        Ok(session_id)
    }

    /// Add cross-chain asset to session
    pub fn add_cross_chain_asset(
        &mut self,
        session_id: &str,
        asset: CrossChainAsset,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(session) = self.active_sessions.get_mut(session_id) {
            session.cross_chain_assets.push(asset);
            Ok(())
        } else {
            Err("Session not found".into())
        }
    }

    /// Start a fusion operation
    pub async fn start_fusion(
        &mut self,
        session_id: &str,
        primary_asset_id: &str,
        secondary_asset_ids: Vec<String>,
        strategy_name: &str,
        emotional_context: EmotionalContext,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let session = self.active_sessions.get_mut(session_id)
            .ok_or("Session not found")?;

        let strategy = self.config.fusion_strategies.get(strategy_name)
            .ok_or("Fusion strategy not found")?;

        let primary_asset = session.cross_chain_assets.iter()
            .find(|a| a.asset_id == primary_asset_id)
            .ok_or("Primary asset not found")?;

        let mut secondary_assets = Vec::new();
        for asset_id in secondary_asset_ids {
            if let Some(asset) = session.cross_chain_assets.iter().find(|a| a.asset_id == asset_id) {
                secondary_assets.push(asset.clone());
            }
        }

        let fusion_id = Uuid::new_v4().to_string();
        let fusion = ActiveFusion {
            fusion_id: fusion_id.clone(),
            primary_asset: primary_asset.clone(),
            secondary_assets,
            fusion_strategy: strategy.clone(),
            emotional_context: emotional_context.clone(),
            start_time: Utc::now(),
            status: FusionStatus::Pending,
            progress: 0.0,
            result: None,
        };

        session.active_fusions.push(fusion);
        self.metrics.total_fusions += 1;

        // Start fusion process
        self.process_fusion(session_id, &fusion_id).await?;

        Ok(fusion_id)
    }

    /// Process a fusion operation
    async fn process_fusion(
        &mut self,
        session_id: &str,
        fusion_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let session = self.active_sessions.get_mut(session_id)
            .ok_or("Session not found")?;

        let fusion = session.active_fusions.iter_mut()
            .find(|f| f.fusion_id == fusion_id)
            .ok_or("Fusion not found")?;

        fusion.status = FusionStatus::InProgress;
        fusion.progress = 0.1;

        // Step 1: Vector search for similar assets
        #[cfg(feature = "db")]
        if self.config.vector_search_enabled {
            if let Some(vector_engine) = &self.vector_engine {
                let query_vector = self.generate_emotional_query_vector(&fusion.emotional_context);
                let search_results = vector_engine.search_blockchain_assets(
                    query_vector,
                    10,
                    Some(self.generate_asset_filter(&fusion.primary_asset)),
                ).await?;
                
                // Use search results to enhance fusion
                self.enhance_fusion_with_vectors(fusion, search_results).await?;
            }
        }
        fusion.progress = 0.3;

        // Step 2: Generate music based on emotional context
        #[cfg(feature = "audio")]
        if self.config.music_generation_enabled {
            if let Some(music_engine) = &self.music_engine {
                let music_config = self.generate_music_config(&fusion.emotional_context);
                let audio_data = music_engine.generate_audio_data(&music_config)?;
                
                // Store generated music in fusion metadata
                self.store_music_in_fusion(fusion, audio_data).await?;
            }
        }
        fusion.progress = 0.6;

        // Step 3: AI inference for creative enhancement
        #[cfg(feature = "ai-ml")]
        if self.config.ai_inference_enabled {
            if let Some(ai_engine) = &self.ai_engine {
                let inference_config = self.generate_inference_config(&fusion.emotional_context);
                let inference_result = ai_engine.run_inference(inference_config).await?;
                
                // Apply AI insights to fusion
                self.apply_ai_insights(fusion, inference_result).await?;
            }
        }
        fusion.progress = 0.9;

        // Step 4: Final synthesis
        let fusion_result = self.synthesize_fusion_result(fusion).await?;
        fusion.result = Some(fusion_result.clone());
        fusion.status = FusionStatus::Completed;
        fusion.progress = 1.0;

        // Update metrics
        self.metrics.successful_fusions += 1;
        self.fusion_history.push(fusion_result);

        Ok(())
    }

    /// Generate emotional query vector for vector search
    fn generate_emotional_query_vector(&self, context: &EmotionalContext) -> Vec<f32> {
        let mut vector = vec![context.valence, context.arousal, context.dominance, context.complexity];
        
        // Add trajectory information
        if !context.trajectory.is_empty() {
            let latest = &context.trajectory[context.trajectory.len() - 1];
            vector.push(latest.valence);
            vector.push(latest.arousal);
        } else {
            vector.push(context.valence);
            vector.push(context.arousal);
        }

        // Pad to required dimensions
        while vector.len() < 128 {
            vector.push(0.0);
        }

        vector
    }

    /// Generate asset filter for vector search
    fn generate_asset_filter(&self, primary_asset: &CrossChainAsset) -> HashMap<String, String> {
        let mut filter = HashMap::new();
        filter.insert("blockchain".to_string(), primary_asset.blockchain.clone());
        filter.insert("asset_type".to_string(), "nft".to_string());
        filter
    }

    /// Generate music config from emotional context
    #[cfg(feature = "audio")]
    fn generate_music_config(&self, context: &EmotionalContext) -> MusicConfig {
        let tempo = match context.arousal {
            a if a > 0.7 => 140.0,
            a if a > 0.4 => 110.0,
            _ => 80.0,
        };

        let key = match context.valence {
            v if v > 0.0 => "C",
            _ => "A",
        };

        MusicConfig {
            tempo,
            key: key.to_string(),
            complexity: context.complexity,
            duration: 30.0,
        }
    }

    /// Generate inference config from emotional context
    #[cfg(feature = "ai-ml")]
    fn generate_inference_config(&self, context: &EmotionalContext) -> InferenceConfig {
        InferenceConfig {
            input_data: serde_json::json!({
                "valence": context.valence,
                "arousal": context.arousal,
                "dominance": context.dominance,
                "complexity": context.complexity,
                "category": context.category,
            }),
            model_type: "emotional_analysis".to_string(),
            parameters: HashMap::new(),
        }
    }

    /// Enhance fusion with vector search results
    async fn enhance_fusion_with_vectors(
        &mut self,
        fusion: &mut ActiveFusion,
        search_results: Vec<VectorSearchResult>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Use vector search results to inform fusion parameters
        if !search_results.is_empty() {
            let avg_score: f32 = search_results.iter().map(|r| r.score).sum::<f32>() / search_results.len() as f32;
            
            // Adjust emotional context based on similar assets
            fusion.emotional_context.complexity = (fusion.emotional_context.complexity + avg_score * 0.2).clamp(0.0, 1.0);
        }

        Ok(())
    }

    /// Store generated music in fusion metadata
    async fn store_music_in_fusion(
        &mut self,
        fusion: &mut ActiveFusion,
        audio_data: Vec<u8>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Store audio data in fusion metadata
        fusion.primary_asset.metadata.insert(
            "generated_music".to_string(),
            serde_json::json!({
                "data_length": audio_data.len(),
                "format": "audio/wav",
                "emotional_context": {
                    "valence": fusion.emotional_context.valence,
                    "arousal": fusion.emotional_context.arousal,
                    "dominance": fusion.emotional_context.dominance,
                }
            }),
        );

        Ok(())
    }

    /// Apply AI insights to fusion
    async fn apply_ai_insights(
        &mut self,
        fusion: &mut ActiveFusion,
        inference_result: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Use AI insights to enhance fusion
        if let Some(creativity_boost) = inference_result.get("creativity_boost") {
            if let Some(boost) = creativity_boost.as_f64() {
                fusion.fusion_strategy.creativity_multiplier *= (1.0 + boost as f32);
            }
        }

        Ok(())
    }

    /// Synthesize final fusion result
    async fn synthesize_fusion_result(
        &mut self,
        fusion: &ActiveFusion,
    ) -> Result<FusionResult, Box<dyn std::error::Error>> {
        let emotional_synthesis = self.synthesize_emotional_data(fusion)?;
        let creative_amplification = self.amplify_creativity(fusion)?;
        let vector_unification = self.unify_vectors(fusion)?;

        let fused_asset = CrossChainAsset {
            asset_id: Uuid::new_v4().to_string(),
            blockchain: fusion.primary_asset.blockchain.clone(),
            contract_address: fusion.primary_asset.contract_address.clone(),
            token_id: format!("fused_{}", fusion.primary_asset.token_id),
            metadata: self.combine_metadata(fusion)?,
            emotional_vector: Some(emotional_synthesis.synthesized_vector.clone()),
            creative_score: creative_amplification.aesthetic_score,
            vector_embedding: Some(vector_unification.unified_embedding.clone()),
        };

        Ok(FusionResult {
            fused_asset,
            emotional_synthesis,
            creative_amplification,
            vector_unification,
            completion_time: Utc::now(),
        })
    }

    /// Synthesize emotional data from fusion
    fn synthesize_emotional_data(&self, fusion: &ActiveFusion) -> Result<EmotionalSynthesis, Box<dyn std::error::Error>> {
        let primary_vector = fusion.primary_asset.emotional_vector.as_ref()
            .unwrap_or(&vec![fusion.emotional_context.valence, fusion.emotional_context.arousal, fusion.emotional_context.dominance]);

        let mut synthesized_vector = primary_vector.clone();
        let mut emotional_categories = vec![fusion.emotional_context.category.clone()];

        // Blend with secondary assets
        for secondary in &fusion.secondary_assets {
            if let Some(secondary_vector) = &secondary.emotional_vector {
                for (i, &value) in secondary_vector.iter().enumerate() {
                    if i < synthesized_vector.len() {
                        synthesized_vector[i] = (synthesized_vector[i] + value) / 2.0;
                    }
                }
            }
        }

        let complexity_score = fusion.emotional_context.complexity;
        let harmony_score = self.calculate_harmony_score(&synthesized_vector);

        Ok(EmotionalSynthesis {
            synthesized_vector,
            emotional_categories,
            complexity_score,
            harmony_score,
        })
    }

    /// Amplify creativity from fusion
    fn amplify_creativity(&self, fusion: &ActiveFusion) -> Result<CreativeAmplification, Box<dyn std::error::Error>> {
        let amplification_factor = fusion.fusion_strategy.creativity_multiplier;
        let novel_elements = vec!["emotional_synthesis".to_string(), "cross_chain_blend".to_string()];
        let aesthetic_score = (fusion.emotional_context.valence + 1.0) / 2.0 * amplification_factor;
        let innovation_index = complexity_score_to_innovation(fusion.emotional_context.complexity);

        Ok(CreativeAmplification {
            amplification_factor,
            novel_elements,
            aesthetic_score,
            innovation_index,
        })
    }

    /// Unify vectors from fusion
    fn unify_vectors(&self, fusion: &ActiveFusion) -> Result<VectorUnification, Box<dyn std::error::Error>> {
        let primary_embedding = fusion.primary_asset.vector_embedding.as_ref()
            .unwrap_or(&vec![0.0; 128]);

        let mut unified_embedding = primary_embedding.clone();
        let mut similarity_scores = vec![];

        // Unify with secondary assets
        for secondary in &fusion.secondary_assets {
            if let Some(secondary_embedding) = &secondary.vector_embedding {
                let similarity = cosine_similarity(primary_embedding, secondary_embedding);
                similarity_scores.push(similarity);
                
                for (i, &value) in secondary_embedding.iter().enumerate() {
                    if i < unified_embedding.len() {
                        unified_embedding[i] = (unified_embedding[i] + value) / 2.0;
                    }
                }
            }
        }

        let similarity_score = if similarity_scores.is_empty() {
            1.0
        } else {
            similarity_scores.iter().sum::<f32>() / similarity_scores.len() as f32
        };

        let coherence_score = self.calculate_coherence_score(&unified_embedding);
        let dimensional_balance = self.calculate_dimensional_balance(&unified_embedding);

        Ok(VectorUnification {
            unified_embedding,
            similarity_score,
            coherence_score,
            dimensional_balance,
        })
    }

    /// Combine metadata from fusion
    fn combine_metadata(&self, fusion: &ActiveFusion) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error>> {
        let mut combined = fusion.primary_asset.metadata.clone();

        // Add fusion-specific metadata
        combined.insert("fusion_type".to_string(), serde_json::json!("multifusion"));
        combined.insert("fusion_strategy".to_string(), serde_json::json!(&fusion.fusion_strategy.name));
        combined.insert("fusion_timestamp".to_string(), serde_json::json!(Utc::now().to_rfc3339()));
        combined.insert("emotional_context".to_string(), serde_json::json!({
            "valence": fusion.emotional_context.valence,
            "arousal": fusion.emotional_context.arousal,
            "dominance": fusion.emotional_context.dominance,
            "complexity": fusion.emotional_context.complexity,
            "category": fusion.emotional_context.category,
        }));

        Ok(combined)
    }

    /// Calculate harmony score for emotional vector
    fn calculate_harmony_score(&self, vector: &[f32]) -> f32 {
        if vector.len() < 3 {
            return 0.5;
        }

        let valence = vector[0];
        let arousal = vector[1];
        let dominance = vector[2];

        // Simple harmony calculation based on emotional balance
        let balance = (valence.abs() + (arousal - 0.5).abs() + (dominance - 0.5).abs()) / 3.0;
        (1.0 - balance).clamp(0.0, 1.0)
    }

    /// Calculate coherence score for vector
    fn calculate_coherence_score(&self, vector: &[f32]) -> f32 {
        if vector.is_empty() {
            return 0.0;
        }

        // Calculate variance as a measure of coherence
        let mean: f32 = vector.iter().sum::<f32>() / vector.len() as f32;
        let variance: f32 = vector.iter().map(|&x| (x - mean).powi(2)).sum::<f32>() / vector.len() as f32;
        
        // Lower variance = higher coherence
        (1.0 - variance.clamp(0.0, 1.0)).clamp(0.0, 1.0)
    }

    /// Calculate dimensional balance for vector
    fn calculate_dimensional_balance(&self, vector: &[f32]) -> f32 {
        if vector.is_empty() {
            return 0.0;
        }

        // Calculate how evenly distributed the values are
        let positive_count = vector.iter().filter(|&&x| x > 0.0).count();
        let balance = positive_count as f32 / vector.len() as f32;
        (1.0 - (balance - 0.5).abs() * 2.0).clamp(0.0, 1.0)
    }

    /// Get fusion metrics
    pub fn get_metrics(&self) -> &FusionMetrics {
        &self.metrics
    }

    /// Get fusion history
    pub fn get_fusion_history(&self) -> &[FusionResult] {
        &self.fusion_history
    }
}

/// Convert complexity score to innovation index
fn complexity_score_to_innovation(complexity: f32) -> f32 {
    // Map complexity (0-1) to innovation index (0-1)
    // Higher complexity generally indicates more innovation
    (complexity * 0.8 + 0.2).clamp(0.0, 1.0)
}

/// Calculate cosine similarity between two vectors
fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() {
        return 0.0;
    }

    let dot_product: f32 = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|&x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|&x| x * x).sum::<f32>().sqrt();

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot_product / (norm_a * norm_b)
    }
}

/// WASM bindings for multifusion
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmMultifusionEngine {
    engine: MultifusionEngine,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmMultifusionEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(config_json: &str) -> Result<WasmMultifusionEngine, JsValue> {
        let config: MultifusionConfig = serde_json::from_str(config_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        Ok(WasmMultifusionEngine {
            engine: MultifusionEngine::new(config),
        })
    }

    #[wasm_bindgen]
    pub fn create_session(&mut self, session_id: String) -> Result<String, JsValue> {
        self.engine.create_session(session_id)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn add_cross_chain_asset(&mut self, session_id: &str, asset_json: &str) -> Result<(), JsValue> {
        let asset: CrossChainAsset = serde_json::from_str(asset_json)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        
        self.engine.add_cross_chain_asset(session_id, asset)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_fusion_metrics(&self) -> Result<String, JsValue> {
        serde_json::to_string(self.engine.get_metrics())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multifusion_engine_creation() {
        let config = MultifusionConfig {
            supported_chains: vec!["near".to_string(), "solana".to_string()],
            fusion_strategies: HashMap::new(),
            cross_chain_bridge_enabled: true,
            vector_search_enabled: true,
            music_generation_enabled: true,
            ai_inference_enabled: true,
            emotional_context_weight: 0.7,
            creativity_boost_factor: 1.5,
        };

        let engine = MultifusionEngine::new(config);
        assert_eq!(engine.metrics.total_fusions, 0);
        assert!(engine.fusion_history.is_empty());
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(cosine_similarity(&a, &b), 1.0);

        let c = vec![0.0, 1.0, 0.0];
        assert_eq!(cosine_similarity(&a, &c), 0.0);
    }

    #[test]
    fn test_complexity_to_innovation() {
        assert_eq!(complexity_score_to_innovation(0.0), 0.2);
        assert_eq!(complexity_score_to_innovation(1.0), 1.0);
    }
}
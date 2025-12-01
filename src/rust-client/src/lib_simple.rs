//! # NFT Rust Client - Simplified Version
//!
//! Core Rust library for generating and formatting audiovisual/emotional metadata
//! Simplified version for WASM compilation testing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// WASM bindings
use wasm_bindgen::prelude::*;

// Simplified modules
pub mod simple_webgpu;
pub mod simple_blockchain;

// Re-export simplified functionality
pub use simple_webgpu::*;
pub use simple_blockchain::*;

/// Core metadata structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeMetadata {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub metadata_type: String,
    pub data: HashMap<String, serde_json::Value>,
}

#[wasm_bindgen]
pub struct MetadataGenerator {
    metadata: HashMap<String, CreativeMetadata>,
}

#[wasm_bindgen]
impl MetadataGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        MetadataGenerator {
            metadata: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn generate_fractal_metadata(&mut self, fractal_type: String, zoom: f32, iterations: u32) -> String {
        let metadata = serde_json::json!({
            "type": "fractal",
            "fractal_type": fractal_type,
            "zoom": zoom,
            "iterations": iterations,
        });
        
        serde_json::to_string_pretty(&metadata).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn generate_audio_metadata(&mut self, frequency: f32, amplitude: f32, duration: f32) -> String {
        let metadata = serde_json::json!({
            "type": "audio",
            "frequency": frequency,
            "amplitude": amplitude,
            "duration": duration,
        });
        
        serde_json::to_string_pretty(&metadata).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn generate_emotional_metadata(&mut self, valence: f32, arousal: f32, dominance: f32) -> String {
        let metadata = serde_json::json!({
            "type": "emotional",
            "valence": valence.clamp(0.0, 1.0),
            "arousal": arousal.clamp(0.0, 1.0),
            "dominance": dominance.clamp(0.0, 1.0),
        });
        
        serde_json::to_string_pretty(&metadata).unwrap_or_default()
    }
}

// WASM initialization
#[wasm_bindgen(start)]
pub fn wasm_init() {
    #[cfg(target_arch = "wasm32")]
    {
        // Simplified initialization without problematic imports
        // console_error_panic_hook::set_once();
        // web_sys::console::log_1(&"Rust WASM module initialized!".into());
    }
}
//! Minimal WASM-compatible Rust library for NFT blockchain interactive

use wasm_bindgen::prelude::*;
use uuid::Uuid;
use std::collections::HashMap;

/// Simple metadata generator
#[wasm_bindgen]
pub struct WasmClient {
    metadata: HashMap<String, String>,
}

#[wasm_bindgen]
impl WasmClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        web_sys::console::log_1(&"WASM Client initialized!".into());
        
        WasmClient {
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
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "id": Uuid::new_v4().to_string(),
        });
        
        let result = serde_json::to_string_pretty(&metadata).unwrap_or_default();
        self.metadata.insert("fractal".to_string(), result.clone());
        result
    }

    #[wasm_bindgen]
    pub fn generate_audio_metadata(&mut self, frequency: f32, amplitude: f32, duration: f32) -> String {
        let metadata = serde_json::json!({
            "type": "audio",
            "frequency": frequency,
            "amplitude": amplitude,
            "duration": duration,
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "id": Uuid::new_v4().to_string(),
        });
        
        let result = serde_json::to_string_pretty(&metadata).unwrap_or_default();
        self.metadata.insert("audio".to_string(), result.clone());
        result
    }

    #[wasm_bindgen]
    pub fn generate_emotional_metadata(&mut self, valence: f32, arousal: f32, dominance: f32) -> String {
        let metadata = serde_json::json!({
            "type": "emotional",
            "valence": valence.clamp(0.0, 1.0),
            "arousal": arousal.clamp(0.0, 1.0),
            "dominance": dominance.clamp(0.0, 1.0),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "id": Uuid::new_v4().to_string(),
        });
        
        let result = serde_json::to_string_pretty(&metadata).unwrap_or_default();
        self.metadata.insert("emotional".to_string(), result.clone());
        result
    }

    #[wasm_bindgen]
    pub fn get_metadata(&self, key: &str) -> Option<String> {
        self.metadata.get(key).cloned()
    }

    #[wasm_bindgen]
    pub fn list_metadata_keys(&self) -> Vec<String> {
        self.metadata.keys().cloned().collect()
    }

    #[wasm_bindgen]
    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    #[wasm_bindgen]
    pub fn get_version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}
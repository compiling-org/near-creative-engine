//! Simplified WebGPU engine for WASM compilation testing

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Simplified shader engine
#[wasm_bindgen]
pub struct SimpleShaderEngine {
    programs: HashMap<String, String>,
}

#[wasm_bindgen]
impl SimpleShaderEngine {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SimpleShaderEngine {
            programs: HashMap::new(),
        }
    }

    #[wasm_bindgen]
    pub fn add_program(&mut self, name: String, code: String) {
        self.programs.insert(name, code);
    }

    #[wasm_bindgen]
    pub fn get_program(&self, name: &str) -> Option<String> {
        self.programs.get(name).cloned()
    }

    #[wasm_bindgen]
    pub fn list_programs(&self) -> Vec<String> {
        self.programs.keys().cloned().collect()
    }
}
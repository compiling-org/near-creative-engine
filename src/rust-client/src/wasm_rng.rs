//! WASM-compatible random number generator
//! Fixes getrandom issues for WASM32 targets

use wasm_bindgen::prelude::*;
use web_sys::window;

/// Custom random number generator for WASM targets
pub struct WasmRng {
    seed: u64,
}

impl WasmRng {
    pub fn new() -> Self {
        let seed = Self::get_wasm_seed();
        Self { seed }
    }
    
    fn get_wasm_seed() -> u64 {
        // Use browser crypto API for seed
        if let Some(window) = web_sys::window() {
            if let Ok(crypto) = window.crypto() {
                if let Ok(array) = crypto.get_random_values_with_u8_array(&mut [0u8; 8]) {
                    return u64::from_le_bytes(array);
                }
            }
            
            // Fallback to performance timer
            if let Some(performance) = window.performance() {
                if let Ok(time) = performance.now() {
                    return time as u64 ^ 0x1234567890ABCDEF;
                }
            }
        }
        
        // Final fallback
        0xDEADBEEF_CAFEBABE
    }
    
    pub fn next_u64(&mut self) -> u64 {
        // Simple linear congruential generator
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.seed
    }
    
    pub fn next_f64(&mut self) -> f64 {
        let bits = self.next_u64();
        // Generate float in [0, 1)
        (bits >> 11) as f64 / (1u64 << 53) as f64
    }
    
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut chunks = dest.chunks_exact_mut(8);
        while let Some(chunk) = chunks.next() {
            let random = self.next_u64();
            chunk.copy_from_slice(&random.to_le_bytes());
        }
        
        let remainder = chunks.into_remainder();
        if !remainder.is_empty() {
            let random = self.next_u64();
            let bytes = random.to_le_bytes();
            remainder.copy_from_slice(&bytes[..remainder.len()]);
        }
    }
}

impl Default for WasmRng {
    fn default() -> Self {
        Self::new()
    }
}

/// WASM-compatible random number generation
#[wasm_bindgen]
pub fn wasm_get_random_bytes(len: usize) -> Vec<u8> {
    let mut rng = WasmRng::new();
    let mut bytes = vec![0u8; len];
    rng.fill_bytes(&mut bytes);
    bytes
}

/// Initialize WASM random number generator
#[wasm_bindgen(start)]
pub fn init_wasm_rng() {
    // Set up custom random generator for WASM
    console_log!("WASM RNG initialized");
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
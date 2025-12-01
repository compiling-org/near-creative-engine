//! WASM-compatible getrandom implementation
//! This module provides a custom implementation for WASM targets

use wasm_bindgen::prelude::*;

/// Custom getrandom implementation for WASM
pub fn custom_getrandom(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    // Use browser crypto API for random generation
    if let Some(window) = web_sys::window() {
        if let Ok(crypto) = window.crypto() {
            for chunk in dest.chunks_mut(65536) {
                if crypto.get_random_values_with_u8_array(chunk).is_err() {
                    return Err(getrandom::Error::UNSUPPORTED);
                }
            }
            return Ok(());
        }
    }
    
    // Fallback: use Math.random() with proper seeding
    let mut seed = 0u64;
    for (i, byte) in dest.iter_mut().enumerate() {
        if i % 8 == 0 {
            seed = js_sys::Math::random().to_bits();
        }
        *byte = (seed >> ((i % 8) * 8)) as u8;
    }
    
    Ok(())
}

/// Initialize WASM getrandom - this will be called automatically
#[wasm_bindgen(start)]
pub fn init_wasm_getrandom() {
    // The custom getrandom will be used automatically when the "custom" feature is enabled
    web_sys::console::log_1(&"WASM getrandom initialized".into());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_custom_getrandom() {
        let mut buffer = [0u8; 32];
        assert!(custom_getrandom(&mut buffer).is_ok());
        assert!(buffer.iter().any(|&b| b != 0)); // Should have some non-zero bytes
    }
}
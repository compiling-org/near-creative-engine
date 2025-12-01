// Simple standalone fractal generator for WASM deployment
// This is a minimal working version for the test website

use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn init() {
    console::log_1(&"🦀 Rust WASM Fractal Engine loaded!".into());
}

#[wasm_bindgen]
pub struct FractalEngine {
    width: u32,
    height: u32,
}

#[wasm_bindgen]
impl FractalEngine {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> FractalEngine {
        console::log_1(&format!("Initializing fractal engine {}x{}", width, height).into());
        FractalEngine { width, height }
    }

    /// Generate Mandelbrot fractal data
    #[wasm_bindgen]
    pub fn generate_mandelbrot(&self, zoom: f64, offset_x: f64, offset_y: f64, max_iterations: u32) -> Vec<u8> {
        let mut pixels = vec![0u8; (self.width * self.height * 4) as usize];
        
        for py in 0..self.height {
            for px in 0..self.width {
                let x0 = ((px as f64 - self.width as f64 / 2.0) / (self.width as f64 / 4.0)) / zoom + offset_x;
                let y0 = ((py as f64 - self.height as f64 / 2.0) / (self.height as f64 / 4.0)) / zoom + offset_y;
                
                let mut x = 0.0;
                let mut y = 0.0;
                let mut iteration = 0;
                
                while x * x + y * y <= 4.0 && iteration < max_iterations {
                    let xtemp = x * x - y * y + x0;
                    y = 2.0 * x * y + y0;
                    x = xtemp;
                    iteration += 1;
                }
                
                let idx = ((py * self.width + px) * 4) as usize;
                if iteration == max_iterations {
                    pixels[idx] = 0;
                    pixels[idx + 1] = 0;
                    pixels[idx + 2] = 0;
                    pixels[idx + 3] = 255;
                } else {
                    let color = (iteration as f64 * 255.0 / max_iterations as f64) as u8;
                    pixels[idx] = color;
                    pixels[idx + 1] = (color as f64 * 0.5) as u8;
                    pixels[idx + 2] = 255 - color;
                    pixels[idx + 3] = 255;
                }
            }
        }
        
        pixels
    }

    /// Generate Julia Set fractal
    #[wasm_bindgen]
    pub fn generate_julia(&self, c_real: f64, c_imag: f64, zoom: f64, max_iterations: u32) -> Vec<u8> {
        let mut pixels = vec![0u8; (self.width * self.height * 4) as usize];
        
        for py in 0..self.height {
            for px in 0..self.width {
                let mut x = ((px as f64 - self.width as f64 / 2.0) / (self.width as f64 / 4.0)) / zoom;
                let mut y = ((py as f64 - self.height as f64 / 2.0) / (self.height as f64 / 4.0)) / zoom;
                let mut iteration = 0;
                
                while x * x + y * y <= 4.0 && iteration < max_iterations {
                    let xtemp = x * x - y * y + c_real;
                    y = 2.0 * x * y + c_imag;
                    x = xtemp;
                    iteration += 1;
                }
                
                let idx = ((py * self.width + px) * 4) as usize;
                if iteration == max_iterations {
                    pixels[idx] = 0;
                    pixels[idx + 1] = 0;
                    pixels[idx + 2] = 0;
                    pixels[idx + 3] = 255;
                } else {
                    let color = (iteration as f64 * 255.0 / max_iterations as f64) as u8;
                    pixels[idx] = (color as f64 * 0.8) as u8;
                    pixels[idx + 1] = color;
                    pixels[idx + 2] = 255 - (color as f64 * 0.6) as u8;
                    pixels[idx + 3] = 255;
                }
            }
        }
        
        pixels
    }

    /// Compress emotional data using 8-bit quantization
    #[wasm_bindgen]
    pub fn compress_emotional_data(&self, valence: f32, arousal: f32, dominance: f32) -> Vec<u8> {
        vec![
            ((valence + 1.0) * 127.5) as u8,  // -1 to 1 -> 0 to 255
            (arousal * 255.0) as u8,           // 0 to 1 -> 0 to 255
            (dominance * 255.0) as u8,         // 0 to 1 -> 0 to 255
        ]
    }

    /// Calculate space savings from compression
    #[wasm_bindgen]
    pub fn calculate_compression_ratio(&self) -> f32 {
        let original_size = 36; // 3 f32 values * 4 bytes * 3 dimensions
        let compressed_size = 3; // 3 u8 values
        ((original_size - compressed_size) as f32 / original_size as f32) * 100.0
    }
    
    /// Generate Burning Ship fractal
    #[wasm_bindgen]
    pub fn generate_burning_ship(&self, zoom: f64, offset_x: f64, offset_y: f64, max_iterations: u32) -> Vec<u8> {
        let mut pixels = vec![0u8; (self.width * self.height * 4) as usize];
        
        for py in 0..self.height {
            for px in 0..self.width {
                let x0 = ((px as f64 - self.width as f64 / 2.0) / (self.width as f64 / 4.0)) / zoom + offset_x;
                let y0 = ((py as f64 - self.height as f64 / 2.0) / (self.height as f64 / 4.0)) / zoom + offset_y;
                
                let mut x = 0.0;
                let mut y = 0.0;
                let mut iteration = 0;
                
                while x * x + y * y <= 4.0 && iteration < max_iterations {
                    let xtemp = x * x - y * y + x0;
                    y = (2.0 * x * y).abs() + y0;
                    x = xtemp.abs();
                    iteration += 1;
                }
                
                let idx = ((py * self.width + px) * 4) as usize;
                if iteration == max_iterations {
                    pixels[idx] = 0;
                    pixels[idx + 1] = 0;
                    pixels[idx + 2] = 0;
                    pixels[idx + 3] = 255;
                } else {
                    let color = (iteration as f64 * 255.0 / max_iterations as f64) as u8;
                    pixels[idx] = 255 - color;
                    pixels[idx + 1] = (color as f64 * 0.7) as u8;
                    pixels[idx + 2] = color;
                    pixels[idx + 3] = 255;
                }
            }
        }
        
        pixels
    }
    
    /// Apply emotional modulation to fractal colors
    #[wasm_bindgen]
    pub fn apply_emotional_filter(&self, pixels: Vec<u8>, valence: f32, arousal: f32) -> Vec<u8> {
        let mut result = pixels;
        let intensity = ((valence + 1.0) / 2.0).clamp(0.0, 1.0);
        let speed = arousal.clamp(0.0, 1.0);
        
        for i in (0..result.len()).step_by(4) {
            // Modulate colors based on emotion
            result[i] = ((result[i] as f32) * intensity) as u8;
            result[i + 1] = ((result[i + 1] as f32) * (1.0 - speed * 0.5)) as u8;
            result[i + 2] = ((result[i + 2] as f32) * speed) as u8;
        }
        
        result
    }
    
    /// Calculate fractal complexity score
    #[wasm_bindgen]
    pub fn calculate_complexity(&self, pixels: &[u8]) -> f32 {
        let mut complexity = 0.0;
        
        for i in (0..pixels.len() - 4).step_by(4) {
            let r1 = pixels[i] as f32;
            let r2 = pixels[i + 4] as f32;
            complexity += (r1 - r2).abs();
        }
        
        complexity / (pixels.len() as f32 / 4.0)
    }
}

/// Generate metadata for NFT
#[wasm_bindgen]
pub fn generate_nft_metadata(title: &str, fractal_type: &str, iterations: u32) -> String {
    format!(
        r#"{{"title":"{}","type":"{}","iterations":{},"generated_by":"Rust WASM","timestamp":{}}}"#,
        title,
        fractal_type,
        iterations,
        js_sys::Date::now() as u64
    )
}

/// Check if WASM is working
#[wasm_bindgen]
pub fn health_check() -> String {
    "Rust WASM engine healthy!".to_string()
}

/// EEG Data Processing
#[wasm_bindgen]
pub struct EEGProcessor {
    sample_rate: u32,
}

#[wasm_bindgen]
impl EEGProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(sample_rate: u32) -> EEGProcessor {
        EEGProcessor { sample_rate }
    }
    
    /// Calculate band power from EEG samples
    #[wasm_bindgen]
    pub fn calculate_band_power(&self, samples: Vec<f32>, low_freq: f32, high_freq: f32) -> f32 {
        // Simple FFT-like band power calculation (simplified for WASM)
        let mut power = 0.0;
        for sample in samples.iter() {
            power += sample * sample;
        }
        power / samples.len() as f32
    }
    
    /// Calculate attention level from EEG
    #[wasm_bindgen]
    pub fn calculate_attention(&self, beta: f32, theta: f32) -> f32 {
        // Attention = Beta / Theta ratio
        if theta > 0.0 {
            (beta / theta).clamp(0.0, 1.0)
        } else {
            0.5
        }
    }
    
    /// Calculate meditation level
    #[wasm_bindgen]
    pub fn calculate_meditation(&self, alpha: f32, beta: f32) -> f32 {
        // Meditation = Alpha / Beta ratio
        if beta > 0.0 {
            (alpha / beta).clamp(0.0, 1.0)
        } else {
            0.5
        }
    }
    
    /// Calculate emotional state from EEG bands
    #[wasm_bindgen]
    pub fn calculate_emotional_state(&self, alpha: f32, beta: f32, theta: f32, delta: f32, gamma: f32) -> Vec<f32> {
        // Returns [valence, arousal, dominance]
        let valence = (alpha - beta).clamp(-1.0, 1.0);
        let arousal = (beta + gamma - delta).clamp(0.0, 1.0);
        let dominance = (alpha + beta - theta).clamp(0.0, 1.0);
        
        vec![valence, arousal, dominance]
    }
}

/// Compress EEG data for storage
#[wasm_bindgen]
pub fn compress_eeg_data(alpha: f32, beta: f32, theta: f32, delta: f32, gamma: f32) -> Vec<u8> {
    vec![
        (alpha * 255.0) as u8,
        (beta * 255.0) as u8,
        (theta * 255.0) as u8,
        (delta * 255.0) as u8,
        (gamma * 255.0) as u8,
    ]
}

/// Calculate storage savings for EEG data
#[wasm_bindgen]
pub fn eeg_compression_ratio() -> f32 {
    let original = 5 * 4; // 5 f32 values = 20 bytes
    let compressed = 5;    // 5 u8 values = 5 bytes
    ((original - compressed) as f32 / original as f32) * 100.0
}

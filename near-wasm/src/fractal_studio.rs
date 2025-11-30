//! Fractal Studio - Core fractal generation engine for NEAR BOS
//! 
//! Implements fractal algorithms from NUWE/Immersive VJ System

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env};

/// Fractal types supported by the studio
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum FractalType {
    Mandelbrot,
    Julia,
    BurningShip,
    Newton,
    Phoenix,
    Custom(String),
}

/// Fractal rendering parameters
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FractalParams {
    pub fractal_type: FractalType,
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub max_iterations: u32,
    pub color_palette: Vec<u32>,
    pub julia_c_real: Option<f64>,
    pub julia_c_imag: Option<f64>,
    pub time_offset: f64,
}

/// Fractal session for live performance tracking
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FractalSession {
    pub session_id: String,
    pub creator: near_sdk::AccountId,
    pub start_time: u64,
    pub params: FractalParams,
    pub keyframes: Vec<FractalKeyframe>,
    pub performance_data: Vec<PerformanceSnapshot>,
}

/// Keyframe for fractal animation
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FractalKeyframe {
    pub timestamp: u64,
    pub params: FractalParams,
    pub emotional_state: Option<EmotionalVector>,
}

/// Performance snapshot for VJ sessions
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PerformanceSnapshot {
    pub timestamp: u64,
    pub fps: f32,
    pub zoom_velocity: f64,
    pub parameter_changes: Vec<String>,
}

/// Emotional vector for creative expression
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
}

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            fractal_type: FractalType::Mandelbrot,
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            max_iterations: 100,
            color_palette: vec![0x000000, 0xFFFFFF],
            julia_c_real: None,
            julia_c_imag: None,
            time_offset: 0.0,
        }
    }
}

impl FractalParams {
    /// Create Mandelbrot parameters
    pub fn mandelbrot() -> Self {
        Self {
            fractal_type: FractalType::Mandelbrot,
            ..Default::default()
        }
    }

    /// Create Julia set parameters
    pub fn julia(c_real: f64, c_imag: f64) -> Self {
        Self {
            fractal_type: FractalType::Julia,
            julia_c_real: Some(c_real),
            julia_c_imag: Some(c_imag),
            center_x: 0.0,
            center_y: 0.0,
            ..Default::default()
        }
    }

    /// Create Burning Ship parameters
    pub fn burning_ship() -> Self {
        Self {
            fractal_type: FractalType::BurningShip,
            center_x: -0.5,
            center_y: -0.5,
            zoom: 0.5,
            ..Default::default()
        }
    }

    /// Apply emotional modulation to fractal parameters
    pub fn apply_emotional_modulation(&mut self, emotion: &EmotionalVector) {
        // Valence affects color intensity
        let _color_intensity = ((emotion.valence + 1.0) / 2.0).clamp(0.0, 1.0);
        
        // Arousal affects iteration count (more arousal = more detail)
        self.max_iterations = (100.0 + emotion.arousal * 200.0) as u32;
        
        // Dominance affects zoom (more dominance = more zoom)
        self.zoom *= 1.0 + (emotion.dominance * 0.1) as f64;
    }
}

impl FractalSession {
    /// Create a new fractal session
    pub fn new(session_id: String, params: FractalParams) -> Self {
        Self {
            session_id,
            creator: env::predecessor_account_id(),
            start_time: env::block_timestamp(),
            params,
            keyframes: Vec::new(),
            performance_data: Vec::new(),
        }
    }

    /// Add a keyframe to the session
    pub fn add_keyframe(&mut self, params: FractalParams, emotional_state: Option<EmotionalVector>) {
        self.keyframes.push(FractalKeyframe {
            timestamp: env::block_timestamp(),
            params,
            emotional_state,
        });
    }

    /// Record performance metrics
    pub fn record_performance(&mut self, fps: f32, zoom_velocity: f64, changes: Vec<String>) {
        self.performance_data.push(PerformanceSnapshot {
            timestamp: env::block_timestamp(),
            fps,
            zoom_velocity,
            parameter_changes: changes,
        });
    }

    /// Get session duration in nanoseconds
    pub fn duration(&self) -> u64 {
        env::block_timestamp() - self.start_time
    }
}

/// Fractal computation functions for WASM
impl FractalParams {
    /// Generate shader code for WebGL/WebGPU
    pub fn generate_shader_code(&self) -> String {
        match &self.fractal_type {
            FractalType::Mandelbrot => self.mandelbrot_shader(),
            FractalType::Julia => self.julia_shader(),
            FractalType::BurningShip => self.burning_ship_shader(),
            FractalType::Newton => self.newton_shader(),
            FractalType::Phoenix => self.phoenix_shader(),
            FractalType::Custom(code) => code.clone(),
        }
    }

    fn mandelbrot_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform vec2 u_center;
            uniform int u_max_iter;
            
            void main() {{
                vec2 c = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom + u_center;
                vec2 z = vec2(0.0);
                int iter = 0;
                
                for (int i = 0; i < {}; i++) {{
                    if (length(z) > 2.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                    iter = i;
                }}
                
                float color = float(iter) / float(u_max_iter);
                gl_FragColor = vec4(vec3(color), 1.0);
            }}
            "#,
            self.max_iterations
        )
    }

    fn julia_shader(&self) -> String {
        let c_real = self.julia_c_real.unwrap_or(-0.7);
        let c_imag = self.julia_c_imag.unwrap_or(0.27015);
        
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform vec2 u_center;
            
            void main() {{
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom + u_center;
                vec2 c = vec2({}, {});
                int iter = 0;
                
                for (int i = 0; i < {}; i++) {{
                    if (length(z) > 2.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                    iter = i;
                }}
                
                float color = float(iter) / float({});
                gl_FragColor = vec4(vec3(color), 1.0);
            }}
            "#,
            c_real, c_imag, self.max_iterations, self.max_iterations
        )
    }

    fn burning_ship_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform vec2 u_center;
            
            void main() {{
                vec2 c = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom + u_center;
                vec2 z = vec2(0.0);
                int iter = 0;
                
                for (int i = 0; i < {}; i++) {{
                    if (length(z) > 2.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * abs(z.x) * abs(z.y)) + c;
                    iter = i;
                }}
                
                float color = float(iter) / float({});
                gl_FragColor = vec4(vec3(color), 1.0);
            }}
            "#,
            self.max_iterations, self.max_iterations
        )
    }

    fn newton_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            
            void main() {{
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom;
                
                for (int i = 0; i < {}; i++) {{
                    // Newton's method for z^3 - 1
                    vec2 z2 = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
                    vec2 z3 = vec2(z2.x * z2.x - z2.y * z2.y, z2.x * z.y + z2.y * z.x);
                    vec2 dz = 3.0 * z2;
                    z = z - vec2(z3.x / dz.x, z3.y / dz.y);
                }}
                
                gl_FragColor = vec4(z.x, z.y, 0.5, 1.0);
            }}
            "#,
            self.max_iterations
        )
    }

    fn phoenix_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            
            void main() {{
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom;
                vec2 p = vec2(0.0);
                
                for (int i = 0; i < {}; i++) {{
                    if (length(z) > 4.0) break;
                    vec2 zn = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + vec2(0.56667, -0.5) + p * 0.5;
                    p = z;
                    z = zn;
                }}
                
                float color = length(z) / 4.0;
                gl_FragColor = vec4(vec3(color), 1.0);
            }}
            "#,
            self.max_iterations
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_params_creation() {
        let mandel = FractalParams::mandelbrot();
        assert_eq!(mandel.max_iterations, 100);
        
        let julia = FractalParams::julia(-0.7, 0.27);
        assert!(julia.julia_c_real.is_some());
    }

    #[test]
    fn test_emotional_modulation() {
        let mut params = FractalParams::default();
        let emotion = EmotionalVector {
            valence: 0.5,
            arousal: 0.8,
            dominance: 0.3,
        };
        
        let original_zoom = params.zoom;
        params.apply_emotional_modulation(&emotion);
        
        assert!(params.zoom > original_zoom);
        assert!(params.max_iterations > 100);
    }

    #[test]
    fn test_shader_generation() {
        let params = FractalParams::mandelbrot();
        let shader = params.generate_shader_code();
        assert!(shader.contains("mandelbrot") || shader.contains("vec2 z"));
    }
}

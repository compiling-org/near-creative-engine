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
    /// Enhanced fractal types with emotional modulation
    EmotionalMandelbrot,
    EmotionalJulia,
    EmotionalBurningShip,
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
    /// Emotional modulation parameters
    pub emotional_modulation: EmotionalModulation,
    /// Advanced rendering parameters
    pub anti_aliasing: bool,
    pub supersampling: u32,
    pub color_smoothing: bool,
}

/// Emotional modulation parameters for fractals
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalModulation {
    pub valence_influence: f32,     // How much valence affects color
    pub arousal_influence: f32,     // How much arousal affects iteration count
    pub dominance_influence: f32,   // How much dominance affects zoom
    pub engagement_boost: f32,      // Overall intensity boost
    pub stress_factor: f32,         // Complexity reduction factor
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
    /// Emotional journey tracking
    pub emotional_journey: Vec<EmotionalSnapshot>,
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
    /// Rendering quality metrics
    pub quality_score: f32,
    pub render_time_ms: f32,
}

/// Emotional vector for creative expression
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalVector {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
}

/// Emotional snapshot for tracking emotional journey
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalSnapshot {
    pub timestamp: u64,
    pub emotional_state: EmotionalVector,
    pub fractal_complexity: f32,
    pub color_variance: f32,
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
            emotional_modulation: EmotionalModulation {
                valence_influence: 0.5,
                arousal_influence: 0.3,
                dominance_influence: 0.2,
                engagement_boost: 1.0,
                stress_factor: 0.0,
            },
            anti_aliasing: false,
            supersampling: 1,
            color_smoothing: true,
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

    /// Create enhanced emotional Mandelbrot
    pub fn emotional_mandelbrot() -> Self {
        Self {
            fractal_type: FractalType::EmotionalMandelbrot,
            emotional_modulation: EmotionalModulation {
                valence_influence: 0.8,
                arousal_influence: 0.5,
                dominance_influence: 0.3,
                engagement_boost: 1.2,
                stress_factor: 0.1,
            },
            max_iterations: 150,
            ..Default::default()
        }
    }

    /// Apply emotional modulation to fractal parameters
    pub fn apply_emotional_modulation(&mut self, emotion: &EmotionalVector) {
        // Valence affects color intensity and palette
        let color_intensity = ((emotion.valence + 1.0) / 2.0).clamp(0.0, 1.0);
        
        // Arousal affects iteration count (more arousal = more detail)
        self.max_iterations = (100.0 + emotion.arousal * 200.0) as u32;
        
        // Dominance affects zoom (more dominance = more zoom)
        self.zoom *= 1.0 + (emotion.dominance * 0.1) as f64;
        
        // Update emotional modulation parameters based on current emotional state
        self.emotional_modulation.valence_influence = (emotion.valence.abs() * 0.5 + 0.3).clamp(0.3, 1.0);
        self.emotional_modulation.arousal_influence = emotion.arousal.clamp(0.1, 1.0);
        self.emotional_modulation.dominance_influence = (emotion.dominance * 0.5 + 0.2).clamp(0.2, 0.8);
        
        // Stress factor reduces complexity
        self.emotional_modulation.stress_factor = (1.0 - emotion.arousal).clamp(0.0, 0.5);
        
        // Engagement boost increases overall intensity
        self.emotional_modulation.engagement_boost = (emotion.arousal * 0.5 + 0.8).clamp(0.8, 1.5);
    }

    /// Get complexity score based on current parameters
    pub fn complexity_score(&self) -> f32 {
        let base_complexity = self.max_iterations as f32 / 100.0;
        let zoom_factor = self.zoom as f32 / 10.0;
        let anti_aliasing_factor = if self.anti_aliasing { 1.5 } else { 1.0 };
        let supersampling_factor = self.supersampling as f32;
        
        base_complexity * zoom_factor * anti_aliasing_factor * supersampling_factor
    }

    /// Generate shader code for WebGL/WebGPU with enhanced emotional modulation
    pub fn generate_shader_code(&self) -> String {
        match self.fractal_type {
            FractalType::Mandelbrot => self.mandelbrot_shader(),
            FractalType::Julia => self.julia_shader(),
            FractalType::BurningShip => self.burning_ship_shader(),
            FractalType::Newton => self.newton_shader(),
            FractalType::Phoenix => self.phoenix_shader(),
            FractalType::Custom(ref code) => code.clone(),
            FractalType::EmotionalMandelbrot => self.emotional_mandelbrot_shader(),
            FractalType::EmotionalJulia => self.emotional_julia_shader(),
            FractalType::EmotionalBurningShip => self.emotional_burning_ship_shader(),
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
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            
            void main() {{
                // Emotional modulation of parameters
                float emotional_zoom = u_zoom * (1.0 + u_valence * 0.2);
                int emotional_iter = int(float(u_max_iter) * (1.0 + u_arousal * 0.5));
                vec2 emotional_center = u_center + vec2(u_dominance * 0.1, u_valence * 0.1);
                
                vec2 c = (gl_FragCoord.xy / u_resolution - 0.5) * emotional_zoom + emotional_center;
                vec2 z = vec2(0.0);
                int iter = 0;
                
                for (int i = 0; i < {}; i++) {{
                    if (i >= emotional_iter) break;
                    if (length(z) > 2.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                    iter = i;
                }}
                
                float t = float(iter) / float(emotional_iter);
                
                // Enhanced color mapping with emotional influence
                float r = t * (1.0 + u_valence * 0.5);
                float g = t * (1.0 + u_arousal * 0.3);
                float b = t * (1.0 + u_dominance * 0.2);
                
                // Add time-based animation
                float wave = sin(u_time * 0.5 + t * 3.14159);
                r += wave * 0.1 * u_arousal;
                g += wave * 0.05 * abs(u_valence);
                b += wave * 0.05 * u_dominance;
                
                gl_FragColor = vec4(r, g, b, 1.0);
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
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            
            void main() {{
                // Emotional modulation
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom + u_center;
                vec2 c = vec2({}, {}) + vec2(u_valence * 0.1, u_arousal * 0.1);
                int emotional_iter = int(float({}) * (1.0 + u_dominance * 0.3));
                
                int iter = 0;
                
                for (int i = 0; i < {}; i++) {{
                    if (i >= emotional_iter) break;
                    if (length(z) > 4.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                    iter = i;
                }}
                
                float t = float(iter) / float(emotional_iter);
                
                // Enhanced emotional color mapping with engagement boost
                float engagement = (abs(u_valence) + u_arousal + u_dominance) / 3.0;
                float r = t * (1.0 + u_valence * engagement * 0.5);
                float g = t * (1.0 + u_arousal * engagement * 0.3);
                float b = t * (1.0 + u_dominance * engagement * 0.2);
                
                // Add time-based effects
                float pulse = sin(u_time * 2.0 + t * 6.28);
                r += pulse * 0.1 * u_arousal;
                g += pulse * 0.05 * abs(u_valence);
                b += pulse * 0.05 * u_dominance;
                
                gl_FragColor = vec4(r, g, b, 1.0);
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
            uniform int u_max_iter;
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            
            void main() {{
                // Emotional modulation
                float emotional_zoom = u_zoom * (1.0 + u_valence * 0.15);
                vec2 emotional_center = u_center + vec2(u_dominance * 0.05, u_arousal * 0.05);
                
                vec2 c = (gl_FragCoord.xy / u_resolution - 0.5) * emotional_zoom + emotional_center;
                vec2 z = vec2(0.0);
                int iter = 0;
                
                // Stress factor reduces complexity
                int max_iter = int(float(u_max_iter) * (1.0 - u_arousal * 0.3));
                
                for (int i = 0; i < {}; i++) {{
                    if (i >= max_iter) break;
                    if (length(z) > 2.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * abs(z.x) * abs(z.y)) + c;
                    iter = i;
                }}
                
                float t = float(iter) / float(max_iter);
                
                // Color with emotional influence
                float r = t * (1.5 + u_valence * 0.4);
                float g = t * (1.0 + u_arousal * 0.2);
                float b = t * (0.5 + u_dominance * 0.3);
                
                // Add time-based animation
                float wave = cos(u_time * 1.5 + t * 3.14159);
                r += wave * 0.1 * abs(u_valence);
                g += wave * 0.05 * u_arousal;
                b += wave * 0.05 * u_dominance;
                
                gl_FragColor = vec4(r, g, b, 1.0);
            }}
            "#,
            self.max_iterations
        )
    }

    fn newton_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            
            void main() {{
                // Emotional modulation
                float emotional_zoom = u_zoom * (1.0 + u_valence * 0.1);
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * emotional_zoom;
                
                // Add emotional offset
                z.x += u_dominance * 0.02;
                z.y += u_arousal * 0.01;
                
                vec2 root1 = vec2(1.0, 0.0);
                vec2 root2 = vec2(-0.5, 0.866025);
                vec2 root3 = vec2(-0.5, -0.866025);
                
                for (int i = 0; i < {}; i++) {{
                    // Newton's method for z^3 - 1
                    vec2 z2 = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
                    vec2 z3 = vec2(z2.x * z2.x - z2.y * z2.y, z2.x * z.y + z2.y * z.x);
                    vec2 dz = 3.0 * z2;
                    
                    // Add emotional perturbation
                    vec2 perturbation = vec2(u_valence * 0.001, u_arousal * 0.001);
                    z = z - vec2(z3.x / dz.x, z3.y / dz.y) + perturbation;
                }}
                
                // Distance to roots with emotional influence
                float d1 = distance(z, root1);
                float d2 = distance(z, root2);
                float d3 = distance(z, root3);
                
                // Color based on closest root with emotional modulation
                float r = (d1 < d2 && d1 < d3) ? 1.0 + u_valence * 0.3 : 0.0;
                float g = (d2 < d1 && d2 < d3) ? 1.0 + u_arousal * 0.3 : 0.0;
                float b = (d3 < d1 && d3 < d2) ? 1.0 + u_dominance * 0.3 : 0.0;
                
                // Add time-based animation
                float pulse = sin(u_time * 0.8);
                r += pulse * 0.1 * abs(u_valence);
                g += pulse * 0.1 * u_arousal;
                b += pulse * 0.1 * u_dominance;
                
                gl_FragColor = vec4(r, g, b, 1.0);
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
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            
            void main() {{
                // Emotional modulation
                float emotional_zoom = u_zoom * (1.0 + u_valence * 0.2);
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * emotional_zoom;
                vec2 p = vec2(0.0);
                
                // Emotional parameters for Phoenix fractal
                float param_a = 0.56667 + u_arousal * 0.1;
                float param_b = -0.5 + u_dominance * 0.1;
                
                for (int i = 0; i < {}; i++) {{
                    if (length(z) > 4.0) break;
                    vec2 zn = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + vec2(param_a, param_b) + p * (0.5 + u_valence * 0.2);
                    p = z;
                    z = zn;
                }}
                
                float color = length(z) / 4.0;
                
                // Enhanced color with emotional modulation
                float r = color * (1.0 + u_valence * 0.5);
                float g = color * (1.0 + u_arousal * 0.3);
                float b = color * (1.0 + u_dominance * 0.2);
                
                // Add time-based effects
                float wave = sin(u_time * 1.2 + color * 6.28);
                r += wave * 0.1 * abs(u_valence);
                g += wave * 0.05 * u_arousal;
                b += wave * 0.05 * u_dominance;
                
                gl_FragColor = vec4(r, g, b, 1.0);
            }}
            "#,
            self.max_iterations
        )
    }

    /// Enhanced emotional Mandelbrot shader with advanced emotional modulation
    fn emotional_mandelbrot_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform vec2 u_center;
            uniform int u_max_iter;
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            uniform float u_engagement;
            uniform float u_stress;
            
            void main() {{
                // Advanced emotional modulation of parameters
                float emotional_zoom = u_zoom * (1.0 + u_valence * 0.3) * (1.0 - u_stress * 0.2);
                int emotional_iter = int(float(u_max_iter) * (1.0 + u_arousal * 0.7) * (1.0 - u_stress * 0.3));
                vec2 emotional_center = u_center + vec2(u_dominance * 0.15, u_valence * 0.12);
                
                vec2 c = (gl_FragCoord.xy / u_resolution - 0.5) * emotional_zoom + emotional_center;
                vec2 z = vec2(0.0);
                int iter = 0;
                
                // Adaptive iteration limit based on engagement
                int adaptive_iter = int(float(emotional_iter) * (0.8 + u_engagement * 0.4));
                
                for (int i = 0; i < {}; i++) {{
                    if (i >= adaptive_iter) break;
                    if (length(z) > 2.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                    iter = i;
                }}
                
                float t = float(iter) / float(adaptive_iter);
                
                // Advanced emotional color mapping
                float engagement_factor = 0.5 + u_engagement * 0.5;
                float stress_factor = 1.0 - u_stress * 0.5;
                
                float r = t * (1.0 + u_valence * 0.6 * engagement_factor) * stress_factor;
                float g = t * (1.0 + u_arousal * 0.4 * engagement_factor) * stress_factor;
                float b = t * (1.0 + u_dominance * 0.3 * engagement_factor) * stress_factor;
                
                // Add time-based animation with emotional influence
                float wave1 = sin(u_time * (1.0 + u_arousal * 0.5) + t * 3.14159);
                float wave2 = cos(u_time * (0.7 + u_dominance * 0.3) + t * 6.28318);
                
                r += wave1 * 0.15 * u_arousal * stress_factor;
                g += wave2 * 0.1 * abs(u_valence) * engagement_factor;
                b += (wave1 + wave2) * 0.05 * u_dominance * stress_factor;
                
                gl_FragColor = vec4(r, g, b, 1.0);
            }}
            "#,
            self.max_iterations
        )
    }

    /// Generate emotional Julia shader with enhanced NUWE integration
    fn emotional_julia_shader(&self) -> String {
        let c_real = self.julia_c_real.unwrap_or(-0.7);
        let c_imag = self.julia_c_imag.unwrap_or(0.27015);
        
        // Use the variable to avoid unused warning
        let _color_intensity = ((c_real + 1.0) / 2.0).clamp(0.0, 1.0);
        
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform vec2 u_center;
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            uniform float u_engagement;
            uniform float u_stress;
            
            void main() {{
                // Emotional modulation
                vec2 z = (gl_FragCoord.xy / u_resolution - 0.5) * u_zoom + u_center;
                vec2 c = vec2({}, {}) + vec2(u_valence * 0.12, u_arousal * 0.08);
                int base_iter = {};
                int emotional_iter = int(float(base_iter) * (1.0 + u_dominance * 0.4) * (1.0 - u_stress * 0.25));
                
                int iter = 0;
                
                // Adaptive iteration with engagement
                int adaptive_iter = int(float(emotional_iter) * (0.7 + u_engagement * 0.6));
                
                for (int i = 0; i < {}; i++) {{
                    if (i >= adaptive_iter) break;
                    if (length(z) > 4.0) break;
                    z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
                    iter = i;
                }}
                
                float t = float(iter) / float(adaptive_iter);
                
                // Advanced emotional color mapping with engagement boost
                float engagement_boost = 0.6 + u_engagement * 0.7;
                float stress_reduction = 1.0 - u_stress * 0.4;
                
                float r = t * (1.0 + u_valence * engagement_boost * 0.6) * stress_reduction;
                float g = t * (1.0 + u_arousal * engagement_boost * 0.4) * stress_reduction;
                float b = t * (1.0 + u_dominance * engagement_boost * 0.3) * stress_reduction;
                
                // Add complex time-based effects
                float pulse1 = sin(u_time * (1.5 + u_arousal * 0.8) + t * 6.28);
                float pulse2 = cos(u_time * (1.2 + u_dominance * 0.6) + t * 3.14);
                float wave3 = sin(u_time * 0.7 + t * 9.42);
                
                r += pulse1 * 0.12 * u_arousal * stress_reduction;
                g += pulse2 * 0.08 * abs(u_valence) * engagement_boost;
                b += (pulse1 * 0.05 + wave3 * 0.07) * u_dominance * stress_reduction;
                
                gl_FragColor = vec4(r, g, b, 1.0);
            }}
            "#,
            c_real, c_imag, self.max_iterations, self.max_iterations
        )
    }

    /// Enhanced emotional Burning Ship shader
    fn emotional_burning_ship_shader(&self) -> String {
        format!(
            r#"
            precision highp float;
            uniform vec2 u_resolution;
            uniform float u_zoom;
            uniform vec2 u_center;
            uniform int u_max_iter;
            uniform float u_time;
            uniform float u_valence;
            uniform float u_arousal;
            uniform float u_dominance;
            uniform float u_engagement;
            uniform float u_stress;
            
            void main() {{
                // Advanced emotional modulation
                float emotional_zoom = u_zoom * (1.0 + u_valence * 0.2) * (1.0 - u_stress * 0.15);
                vec2 emotional_center = u_center + vec2(u_dominance * 0.08, u_arousal * 0.06);
                
                vec2 c = (gl_FragCoord.xy / u_resolution - 0.5) * emotional_zoom + emotional_center;
                vec2 z = vec2(0.0);
                int iter = 0;
                
                // Stress and engagement factors affect complexity
                float complexity_factor = (1.0 - u_stress * 0.4) * (0.8 + u_engagement * 0.4);
                int max_iter = int(float(u_max_iter) * complexity_factor);
                
                for (int i = 0; i < {}; i++) {{
                    if (i >= max_iter) break;
                    if (length(z) > 2.0) break;
                    // Burning Ship formula with emotional perturbation
                    float x = abs(z.x * z.x - z.y * z.y) + c.x + u_valence * 0.005;
                    float y = abs(2.0 * z.x * z.y) + c.y + u_arousal * 0.003;
                    z = vec2(x, y);
                    iter = i;
                }}
                
                float t = float(iter) / float(max_iter);
                
                // Color with advanced emotional influence
                float engagement_factor = 0.7 + u_engagement * 0.6;
                float stress_factor = 1.0 - u_stress * 0.3;
                
                float r = t * (1.8 + u_valence * 0.5 * engagement_factor) * stress_factor;
                float g = t * (1.2 + u_arousal * 0.3 * engagement_factor) * stress_factor;
                float b = t * (0.7 + u_dominance * 0.4 * engagement_factor) * stress_factor;
                
                // Add time-based animation
                float wave1 = sin(u_time * (1.0 + u_arousal * 0.6) + t * 4.71);
                float wave2 = cos(u_time * (0.8 + u_dominance * 0.4) + t * 3.14);
                
                r += wave1 * 0.1 * abs(u_valence) * stress_factor;
                g += wave2 * 0.08 * u_arousal * engagement_factor;
                b += (wave1 * 0.05 + wave2 * 0.03) * u_dominance * stress_factor;
                
                gl_FragColor = vec4(r, g, b, 1.0);
            }}
            "#,
            self.max_iterations
        )
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
            emotional_journey: Vec::new(),
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
    pub fn record_performance(&mut self, fps: f32, zoom_velocity: f64, changes: Vec<String>, quality_score: f32, render_time: f32) {
        self.performance_data.push(PerformanceSnapshot {
            timestamp: env::block_timestamp(),
            fps,
            zoom_velocity,
            parameter_changes: changes,
            quality_score,
            render_time_ms: render_time,
        });
    }

    /// Record emotional snapshot
    pub fn record_emotional_snapshot(&mut self, emotional_state: EmotionalVector) {
        let snapshot = EmotionalSnapshot {
            timestamp: env::block_timestamp(),
            emotional_state: emotional_state.clone(),
            fractal_complexity: self.params.complexity_score(),
            color_variance: self.calculate_color_variance(&emotional_state),
        };
        
        self.emotional_journey.push(snapshot);
        
        // Apply emotional modulation to parameters
        self.params.apply_emotional_modulation(&emotional_state);
    }

    /// Calculate color variance based on emotional state
    fn calculate_color_variance(&self, emotion: &EmotionalVector) -> f32 {
        // Simple calculation based on emotional vector magnitude
        let valence_contrib = emotion.valence.abs() * 0.4;
        let arousal_contrib = emotion.arousal * 0.4;
        let dominance_contrib = emotion.dominance * 0.2;
        
        (valence_contrib + arousal_contrib + dominance_contrib).clamp(0.0, 1.0)
    }

    /// Get session duration in nanoseconds
    pub fn duration(&self) -> u64 {
        env::block_timestamp() - self.start_time
    }

    /// Get emotional journey summary
    pub fn emotional_journey_summary(&self) -> (EmotionalVector, EmotionalVector, f32) {
        if self.emotional_journey.is_empty() {
            return (
                EmotionalVector { valence: 0.0, arousal: 0.0, dominance: 0.0 },
                EmotionalVector { valence: 0.0, arousal: 0.0, dominance: 0.0 },
                0.0
            );
        }

        let first = &self.emotional_journey[0].emotional_state;
        let last = &self.emotional_journey[self.emotional_journey.len() - 1].emotional_state;
        
        // Calculate average emotional state
        let mut avg_valence = 0.0;
        let mut avg_arousal = 0.0;
        let mut avg_dominance = 0.0;
        
        for snapshot in &self.emotional_journey {
            avg_valence += snapshot.emotional_state.valence;
            avg_arousal += snapshot.emotional_state.arousal;
            avg_dominance += snapshot.emotional_state.dominance;
        }
        
        let count = self.emotional_journey.len() as f32;
        avg_valence /= count;
        avg_arousal /= count;
        avg_dominance /= count;
        
        // Use the average value
        let _average = EmotionalVector {
            valence: avg_valence,
            arousal: avg_arousal,
            dominance: avg_dominance,
        };
        
        (first.clone(), last.clone(), count)
    }
}

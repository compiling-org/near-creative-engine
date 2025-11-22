//! WGSL Studio - WebGPU Shader Language studio for NEAR BOS
//! 
//! Real-time shader editing and performance tools from NUWE

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env};

/// WGSL shader program
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WGSLShader {
    pub shader_id: String,
    pub name: String,
    pub vertex_code: String,
    pub fragment_code: String,
    pub compute_code: Option<String>,
    pub created_at: u64,
    pub creator: near_sdk::AccountId,
}

/// Shader parameters for live control
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShaderParams {
    pub time: f32,
    pub resolution: (f32, f32),
    pub mouse: (f32, f32),
    pub custom_uniforms: Vec<UniformParam>,
}

/// Custom uniform parameter
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UniformParam {
    pub name: String,
    pub value_type: UniformType,
    pub value: Vec<f32>,
}

/// Uniform value types
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum UniformType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat4,
}

/// Live coding session for WGSL shaders
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct WGSLSession {
    pub session_id: String,
    pub shader: WGSLShader,
    pub params: ShaderParams,
    pub edit_history: Vec<ShaderEdit>,
    pub performance_metrics: PerformanceMetrics,
}

/// Shader edit for version tracking
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShaderEdit {
    pub timestamp: u64,
    pub fragment_code: String,
    pub description: String,
}

/// Performance metrics for shader
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PerformanceMetrics {
    pub avg_fps: f32,
    pub compile_time_ms: f32,
    pub gpu_memory_mb: f32,
}

impl WGSLShader {
    /// Create a new WGSL shader
    pub fn new(shader_id: String, name: String) -> Self {
        Self {
            shader_id,
            name,
            vertex_code: Self::default_vertex_shader(),
            fragment_code: Self::default_fragment_shader(),
            compute_code: None,
            created_at: env::block_timestamp(),
            creator: env::predecessor_account_id(),
        }
    }

    /// Default vertex shader for fullscreen quad
    fn default_vertex_shader() -> String {
        r#"
@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0)
    );
    
    let pos = positions[vertex_index];
    return vec4<f32>(pos, 0.0, 1.0);
}
        "#.to_string()
    }

    /// Default fragment shader
    fn default_fragment_shader() -> String {
        r#"
@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> resolution: vec2<f32>;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / resolution;
    let color = vec3<f32>(uv.x, uv.y, 0.5 + 0.5 * sin(time));
    return vec4<f32>(color, 1.0);
}
        "#.to_string()
    }

    /// Create a fractal shader template
    pub fn fractal_template() -> String {
        r#"
@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> resolution: vec2<f32>;
@group(0) @binding(2) var<uniform> zoom: f32;
@group(0) @binding(3) var<uniform> center: vec2<f32>;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    var uv = (pos.xy / resolution - 0.5) * zoom + center;
    var z = vec2<f32>(0.0, 0.0);
    var iterations = 0;
    
    for (var i = 0; i < 100; i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + uv;
        iterations = i;
    }
    
    let color = f32(iterations) / 100.0;
    return vec4<f32>(vec3<f32>(color), 1.0);
}
        "#.to_string()
    }

    /// Create audio-reactive shader template
    pub fn audio_reactive_template() -> String {
        r#"
@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> resolution: vec2<f32>;
@group(0) @binding(2) var<uniform> audio_bass: f32;
@group(0) @binding(3) var<uniform> audio_mid: f32;
@group(0) @binding(4) var<uniform> audio_high: f32;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / resolution;
    
    // Audio-reactive visualization
    let bass_pulse = audio_bass * sin(time * 2.0 + uv.y * 10.0);
    let mid_wave = audio_mid * cos(time + uv.x * 15.0);
    let high_sparkle = audio_high * sin(time * 4.0);
    
    let color = vec3<f32>(
        0.5 + 0.5 * bass_pulse,
        0.5 + 0.5 * mid_wave,
        0.5 + 0.5 * high_sparkle
    );
    
    return vec4<f32>(color, 1.0);
}
        "#.to_string()
    }
}

impl WGSLSession {
    /// Create a new WGSL live coding session
    pub fn new(session_id: String, shader: WGSLShader) -> Self {
        Self {
            session_id,
            shader,
            params: ShaderParams::default(),
            edit_history: Vec::new(),
            performance_metrics: PerformanceMetrics::default(),
        }
    }

    /// Record a shader edit
    pub fn record_edit(&mut self, fragment_code: String, description: String) {
        self.edit_history.push(ShaderEdit {
            timestamp: env::block_timestamp(),
            fragment_code: fragment_code.clone(),
            description,
        });
        self.shader.fragment_code = fragment_code;
    }

    /// Update performance metrics
    pub fn update_metrics(&mut self, fps: f32, compile_time: f32, gpu_memory: f32) {
        self.performance_metrics = PerformanceMetrics {
            avg_fps: fps,
            compile_time_ms: compile_time,
            gpu_memory_mb: gpu_memory,
        };
    }
}

impl Default for ShaderParams {
    fn default() -> Self {
        Self {
            time: 0.0,
            resolution: (1920.0, 1080.0),
            mouse: (0.0, 0.0),
            custom_uniforms: Vec::new(),
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_fps: 60.0,
            compile_time_ms: 0.0,
            gpu_memory_mb: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wgsl_shader_creation() {
        let shader = WGSLShader::new("test_shader".to_string(), "Test Shader".to_string());
        assert_eq!(shader.shader_id, "test_shader");
        assert!(!shader.vertex_code.is_empty());
        assert!(!shader.fragment_code.is_empty());
    }

    #[test]
    fn test_shader_templates() {
        let fractal = WGSLShader::fractal_template();
        assert!(fractal.contains("@fragment"));
        assert!(fractal.contains("zoom"));
        
        let audio = WGSLShader::audio_reactive_template();
        assert!(audio.contains("audio_bass"));
    }
}

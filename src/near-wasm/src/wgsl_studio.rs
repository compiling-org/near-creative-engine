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
    /// Shader metadata for validation and reflection
    pub metadata: ShaderMetadata,
}

/// Shader metadata for validation and reflection
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShaderMetadata {
    pub entry_points: Vec<String>,
    pub uniforms: Vec<UniformInfo>,
    pub storage_buffers: Vec<StorageBufferInfo>,
    pub textures: Vec<TextureInfo>,
    pub samplers: Vec<SamplerInfo>,
    /// Validation status
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    /// Performance metrics
    pub estimated_complexity: f32,
}

/// Information about a uniform variable
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct UniformInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub ty: String, // Type as string representation
    pub size: u32,
}

/// Information about a storage buffer
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct StorageBufferInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub size: u32,
    pub access_mode: String, // "read", "write", "read_write"
}

/// Information about a texture
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TextureInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub ty: String, // "texture_2d", "texture_cube", etc.
    pub format: String, // "f32", "u32", etc.
}

/// Information about a sampler
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SamplerInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub ty: String, // "sampler", "comparison_sampler"
}

/// Shader parameters for live control
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShaderParams {
    pub time: f32,
    pub resolution: (f32, f32),
    pub mouse: (f32, f32),
    pub custom_uniforms: Vec<UniformParam>,
    /// Emotional parameters for modulation
    pub emotional_params: EmotionalParams,
}

/// Emotional parameters for shader modulation
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalParams {
    pub valence: f32,      // -1.0 to 1.0
    pub arousal: f32,      // 0.0 to 1.0
    pub dominance: f32,    // 0.0 to 1.0
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
    /// Emotional resonance tracking
    pub emotional_resonance: EmotionalResonance,
}

/// Emotional resonance tracking for shader sessions
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalResonance {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub engagement: f32,
    pub last_updated: u64,
}

/// Shader edit for version tracking
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShaderEdit {
    pub timestamp: u64,
    pub fragment_code: String,
    pub description: String,
    pub emotional_state: Option<EmotionalParams>,
}

/// Performance metrics for shader
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PerformanceMetrics {
    pub avg_fps: f32,
    pub compile_time_ms: f32,
    pub gpu_memory_mb: f32,
    /// Complexity metrics
    pub instruction_count: u32,
    pub texture_samples: u32,
    pub arithmetic_operations: u32,
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
            metadata: ShaderMetadata {
                entry_points: vec!["vs_main".to_string(), "fs_main".to_string()],
                uniforms: vec![],
                storage_buffers: vec![],
                textures: vec![],
                samplers: vec![],
                is_valid: true,
                validation_errors: vec![],
                estimated_complexity: 0.1,
            },
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
@group(0) @binding(2) var<uniform> valence: f32;
@group(0) @binding(3) var<uniform> arousal: f32;
@group(0) @binding(4) var<uniform> dominance: f32;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / resolution;
    
    // Emotional modulation
    let color_shift = valence * 0.5;
    let intensity = arousal;
    let complexity = dominance * 2.0;
    
    let color = vec3<f32>(
        0.5 + uv.x * (1.0 + color_shift),
        0.5 + uv.y * intensity,
        0.5 + 0.5 * sin(time * complexity)
    );
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
@group(0) @binding(4) var<uniform> valence: f32;
@group(0) @binding(5) var<uniform> arousal: f32;
@group(0) @binding(6) var<uniform> dominance: f32;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    var uv = (pos.xy / resolution - 0.5) * zoom + center;
    
    // Emotional modulation of fractal parameters
    let emotional_zoom = zoom * (1.0 + valence * 0.2);
    let emotional_iterations = 100 + (arousal * 100.0) as u32;
    let emotional_offset = center + vec2<f32>(dominance * 0.1, valence * 0.1);
    
    var z = vec2<f32>(0.0, 0.0);
    var iterations = 0;
    
    for (var i = 0; i < emotional_iterations; i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + uv;
        iterations = i;
    }
    
    let color = f32(iterations) / f32(emotional_iterations);
    
    // Color modulation based on emotional state
    let r = color * (1.0 + valence);
    let g = color * (1.0 + arousal);
    let b = color * (1.0 + dominance);
    
    return vec4<f32>(vec3<f32>(r, g, b), 1.0);
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
@group(0) @binding(5) var<uniform> valence: f32;
@group(0) @binding(6) var<uniform> arousal: f32;
@group(0) @binding(7) var<uniform> dominance: f32;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy / resolution;
    
    // Audio-reactive visualization with emotional modulation
    let bass_pulse = audio_bass * sin(time * 2.0 + uv.y * 10.0 + valence * 3.0);
    let mid_wave = audio_mid * cos(time + uv.x * 15.0 + arousal * 2.0);
    let high_sparkle = audio_high * sin(time * 4.0 + dominance * 5.0);
    
    let color = vec3<f32>(
        0.5 + 0.5 * bass_pulse,
        0.5 + 0.5 * mid_wave,
        0.5 + 0.5 * high_sparkle
    );
    
    return vec4<f32>(color, 1.0);
}
        "#.to_string()
    }

    /// Validate shader code and extract metadata using improved reflection
    pub fn validate_and_analyze(&mut self) -> bool {
        // In a real implementation, this would use wgsl_reflect or similar libraries
        // to parse and analyze the shader code
        
        // For now, we'll do a more comprehensive check and set metadata
        self.metadata.is_valid = !self.fragment_code.is_empty() && !self.vertex_code.is_empty();
        
        if !self.metadata.is_valid {
            self.metadata.validation_errors.push("Shader code is empty".to_string());
            return false;
        }
        
        // Extract information about uniforms, storage buffers, textures, and samplers
        self.metadata.uniforms.clear();
        self.metadata.storage_buffers.clear();
        self.metadata.textures.clear();
        self.metadata.samplers.clear();
        
        // Clone the code to avoid borrowing conflicts
        let vertex_code = self.vertex_code.clone();
        let fragment_code = self.fragment_code.clone();
        
        // Parse uniforms from both vertex and fragment shaders
        self.parse_uniforms(&vertex_code, 0);
        self.parse_uniforms(&fragment_code, 0);
        
        // Parse storage buffers
        self.parse_storage_buffers(&vertex_code, 0);
        self.parse_storage_buffers(&fragment_code, 0);
        
        // Parse textures
        self.parse_textures(&vertex_code, 0);
        self.parse_textures(&fragment_code, 0);
        
        // Parse samplers
        self.parse_samplers(&vertex_code, 0);
        self.parse_samplers(&fragment_code, 0);
        
        // Extract entry points
        self.metadata.entry_points.clear();
        if self.vertex_code.contains("vs_main") {
            self.metadata.entry_points.push("vs_main".to_string());
        }
        if self.fragment_code.contains("fs_main") {
            self.metadata.entry_points.push("fs_main".to_string());
        }
        if let Some(ref compute_code) = self.compute_code {
            if compute_code.contains("cs_main") {
                self.metadata.entry_points.push("cs_main".to_string());
            }
        }
        
        // Estimate complexity based on code length and features
        let code_length = self.vertex_code.len() + self.fragment_code.len() + 
            self.compute_code.as_ref().map_or(0, |c| c.len());
        self.metadata.estimated_complexity = (code_length as f32) / 1000.0;
        
        // Add emotional computing validation
        self.validate_emotional_parameters();
        
        self.metadata.is_valid
    }
    
    /// Parse uniforms from shader code
    fn parse_uniforms(&mut self, code: &str, group: u32) {
        // Look for uniform declarations: @group(@binding) var<uniform> name: type;
        let lines: Vec<&str> = code.lines().collect();
        let mut binding_counter = 0;
        
        for line in lines {
            if line.contains("var<uniform>") {
                // Extract name and type
                if let Some(name_start) = line.find("var<uniform>") {
                    let name_part = &line[name_start + 12..];
                    if let Some(name_end) = name_part.find(':') {
                        let name = name_part[..name_end].trim();
                        let type_part = name_part[name_end + 1..].trim();
                        if let Some(type_end) = type_part.find(';') {
                            let ty = type_part[..type_end].trim();
                            
                            // Estimate size based on type
                            let size = match ty {
                                "f32" => 4,
                                "i32" => 4,
                                "u32" => 4,
                                "vec2<f32>" | "vec2<i32>" | "vec2<u32>" => 8,
                                "vec3<f32>" | "vec3<i32>" | "vec3<u32>" => 12,
                                "vec4<f32>" | "vec4<i32>" | "vec4<u32>" => 16,
                                "mat2x2<f32>" => 16,
                                "mat3x3<f32>" => 36,
                                "mat4x4<f32>" => 64,
                                _ => 16, // default size
                            };
                            
                            self.metadata.uniforms.push(UniformInfo {
                                name: name.to_string(),
                                binding: binding_counter,
                                group,
                                ty: ty.to_string(),
                                size,
                            });
                            
                            binding_counter += 1;
                        }
                    }
                }
            }
        }
    }
    
    /// Parse storage buffers from shader code
    fn parse_storage_buffers(&mut self, code: &str, group: u32) {
        // Look for storage buffer declarations: @group(@binding) var<storage, access_mode> name: type;
        let lines: Vec<&str> = code.lines().collect();
        let mut binding_counter = 0;
        
        for line in lines {
            if line.contains("var<storage") {
                // Extract name, type, and access mode
                if let Some(storage_start) = line.find("var<storage") {
                    let storage_part = &line[storage_start + 11..];
                    if let Some(storage_end) = storage_part.find('>') {
                        let access_part = &storage_part[..storage_end];
                        let access_mode = if access_part.contains("read_write") {
                            "read_write"
                        } else if access_part.contains("write") {
                            "write"
                        } else {
                            "read"
                        };
                        
                        let name_part = &storage_part[storage_end + 1..];
                        if let Some(name_start) = name_part.find(|c: char| c.is_alphabetic()) {
                            let name_and_type = &name_part[name_start..];
                            if let Some(name_end) = name_and_type.find(':') {
                                let name = name_and_type[..name_end].trim();
                                let type_part = name_and_type[name_end + 1..].trim();
                                if let Some(type_end) = type_part.find(';') {
                                    let _ty = type_part[..type_end].trim(); // Use _ty to avoid warning
                                    
                                    self.metadata.storage_buffers.push(StorageBufferInfo {
                                        name: name.to_string(),
                                        binding: binding_counter,
                                        group,
                                        size: 0, // Would need to parse the struct to determine size
                                        access_mode: access_mode.to_string(),
                                    });
                                    
                                    binding_counter += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Parse textures from shader code
    fn parse_textures(&mut self, code: &str, group: u32) {
        // Look for texture declarations: @group(@binding) var name: texture_type;
        let lines: Vec<&str> = code.lines().collect();
        let mut binding_counter = 0;
        
        for line in lines {
            if line.contains("var") && (line.contains("texture_") || line.contains("Texture")) {
                // Extract name and type
                if let Some(var_start) = line.find("var") {
                    let var_part = &line[var_start + 3..];
                    if let Some(name_end) = var_part.find(':') {
                        let name = var_part[..name_end].trim();
                        let type_part = var_part[name_end + 1..].trim();
                        if let Some(type_end) = type_part.find(';') {
                            let ty = type_part[..type_end].trim();
                            
                            // Extract format from type if available
                            let format = if ty.contains("f32") {
                                "f32"
                            } else if ty.contains("i32") {
                                "i32"
                            } else if ty.contains("u32") {
                                "u32"
                            } else {
                                "unknown"
                            };
                            
                            self.metadata.textures.push(TextureInfo {
                                name: name.to_string(),
                                binding: binding_counter,
                                group,
                                ty: ty.to_string(),
                                format: format.to_string(),
                            });
                            
                            binding_counter += 1;
                        }
                    }
                }
            }
        }
    }
    
    /// Parse samplers from shader code
    fn parse_samplers(&mut self, code: &str, group: u32) {
        // Look for sampler declarations: @group(@binding) var name: sampler_type;
        let lines: Vec<&str> = code.lines().collect();
        let mut binding_counter = 0;
        
        for line in lines {
            if line.contains("var") && (line.contains("sampler") || line.contains("Sampler")) {
                // Extract name and type
                if let Some(var_start) = line.find("var") {
                    let var_part = &line[var_start + 3..];
                    if let Some(name_end) = var_part.find(':') {
                        let name = var_part[..name_end].trim();
                        let type_part = var_part[name_end + 1..].trim();
                        if let Some(type_end) = type_part.find(';') {
                            let ty = type_part[..type_end].trim();
                            
                            self.metadata.samplers.push(SamplerInfo {
                                name: name.to_string(),
                                binding: binding_counter,
                                group,
                                ty: ty.to_string(),
                            });
                            
                            binding_counter += 1;
                        }
                    }
                }
            }
        }
    }
    
    /// Validate emotional parameters in shader
    fn validate_emotional_parameters(&mut self) {
        // Check if required emotional parameters are present
        let required_params = ["valence", "arousal", "dominance"];
        let mut missing_params = Vec::new();
        
        for param in &required_params {
            let found = self.metadata.uniforms.iter().any(|u| u.name == *param);
            if !found {
                missing_params.push(param.to_string());
            }
        }
        
        if !missing_params.is_empty() {
            self.metadata.validation_errors.push(
                format!("Missing emotional parameters: {}", missing_params.join(", "))
            );
        }
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
            emotional_resonance: EmotionalResonance {
                valence: 0.0,
                arousal: 0.5,
                dominance: 0.5,
                engagement: 0.0,
                last_updated: env::block_timestamp(),
            },
        }
    }

    /// Record a shader edit
    pub fn record_edit(&mut self, fragment_code: String, description: String, emotional_state: Option<EmotionalParams>) {
        self.edit_history.push(ShaderEdit {
            timestamp: env::block_timestamp(),
            fragment_code: fragment_code.clone(),
            description,
            emotional_state,
        });
        self.shader.fragment_code = fragment_code;
        
        // Revalidate shader after edit
        self.shader.validate_and_analyze();
    }

    /// Update performance metrics
    pub fn update_metrics(&mut self, fps: f32, compile_time: f32, gpu_memory: f32) {
        self.performance_metrics = PerformanceMetrics {
            avg_fps: fps,
            compile_time_ms: compile_time,
            gpu_memory_mb: gpu_memory,
            instruction_count: 0,
            texture_samples: 0,
            arithmetic_operations: 0,
        };
    }

    /// Update emotional resonance based on user interactions
    pub fn update_emotional_resonance(&mut self, emotional_params: EmotionalParams) {
        self.emotional_resonance.valence = emotional_params.valence;
        self.emotional_resonance.arousal = emotional_params.arousal;
        self.emotional_resonance.dominance = emotional_params.dominance;
        self.emotional_resonance.last_updated = env::block_timestamp();
        
        // Update engagement based on emotional variance
        let variance = emotional_params.valence.abs() + emotional_params.arousal.abs() + emotional_params.dominance.abs();
        self.emotional_resonance.engagement = variance / 3.0;
    }
}

impl Default for ShaderParams {
    fn default() -> Self {
        Self {
            time: 0.0,
            resolution: (1920.0, 1080.0),
            mouse: (0.0, 0.0),
            custom_uniforms: Vec::new(),
            emotional_params: EmotionalParams {
                valence: 0.0,
                arousal: 0.5,
                dominance: 0.5,
            },
        }
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_fps: 60.0,
            compile_time_ms: 0.0,
            gpu_memory_mb: 0.0,
            instruction_count: 0,
            texture_samples: 0,
            arithmetic_operations: 0,
        }
    }
}
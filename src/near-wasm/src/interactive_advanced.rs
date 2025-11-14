//! Advanced Interactive NFT with EEG/BMI Integration
//! 
//! Revolutionary NFT system that responds to real-time emotional states
//! from brain-computer interfaces and biometric sensors

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near, AccountId, Timestamp};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use md5::{Md5, Digest};

/// Interactive NFT with biometric integration
#[derive(BorshDeserialize, BorshSerialize)]
pub struct BiometricNFT {
    /// Token ID
    pub token_id: String,
    
    /// Current owner
    pub owner: AccountId,
    
    /// Visual state that changes based on emotions
    pub visual_state: VisualState,
    
    /// Emotional interaction history
    pub interaction_history: Vector<EmotionalInteraction>,
    
    /// Biometric profiles authorized to interact
    pub authorized_profiles: LookupMap<AccountId, BiometricProfile>,
    
    /// Current emotional resonance
    pub emotional_resonance: EmotionalResonance,
    
    /// NFT metadata
    pub metadata: InteractiveMetadata,
    
    /// Privacy settings
    pub privacy: PrivacySettings,
    
    /// Shader and fractal parameters for NUWE integration
    pub shader_params: ShaderParams,
    pub fractal_params: FractalParams,
    
    /// Data integrity verification
    pub integrity_hash: String,
}

/// Visual state of the NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VisualState {
    /// Base fractal parameters
    pub fractal_type: String,
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    
    /// Color scheme modulated by emotions
    pub color_palette: Vec<ColorRGB>,
    pub color_intensity: f32,
    
    /// Animation parameters
    pub animation_speed: f32,
    pub morphing_rate: f32,
    
    /// Complexity level (affected by arousal)
    pub detail_level: u32,
    
    /// Shader parameters for WebGL/WebGPU
    pub shader_uniforms: Vec<ShaderUniform>,
    
    /// Emotional modulation parameters
    pub emotional_modulation: EmotionalModulation,
}

/// Emotional modulation parameters
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalModulation {
    pub valence_influence: f32,
    pub arousal_influence: f32,
    pub dominance_influence: f32,
    pub engagement_boost: f32,
    pub stress_factor: f32,
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

/// Fractal parameters for NUWE integration
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct FractalParams {
    pub fractal_type: String, // Matches FractalType enum
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub max_iterations: u32,
    pub color_palette: Vec<u32>,
    pub julia_c_real: Option<f64>,
    pub julia_c_imag: Option<f64>,
    pub time_offset: f64,
    /// Emotional modulation parameters
    pub emotional_modulation: EmotionalModulationParams,
}

/// Emotional modulation parameters for fractals
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalModulationParams {
    pub valence_influence: f32,
    pub arousal_influence: f32,
    pub dominance_influence: f32,
    pub engagement_boost: f32,
    pub stress_factor: f32,
}

/// RGB color
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Copy)]
#[serde(crate = "near_sdk::serde")]
pub struct ColorRGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// Shader uniform parameter
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ShaderUniform {
    pub name: String,
    pub value: f32,
}

/// Emotional interaction event
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalInteraction {
    pub timestamp: Timestamp,
    pub user: AccountId,
    pub emotional_state: DetailedEmotionalState,
    pub biometric_data: BiometricSnapshot,
    pub interaction_type: InteractionType,
    pub state_before: VisualStateSnapshot,
    pub state_after: VisualStateSnapshot,
    /// Shader and fractal parameters at time of interaction
    pub shader_params: ShaderParams,
    pub fractal_params: FractalParams,
}

/// Detailed emotional state with multiple dimensions
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DetailedEmotionalState {
    /// Core VAD model
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    
    /// Extended emotional dimensions
    pub engagement: f32,
    pub focus: f32,
    pub stress: f32,
    pub relaxation: f32,
    
    /// Confidence in measurement
    pub confidence: f32,
    
    /// Primary emotion detected
    pub primary_emotion: String,
    
    /// Emotion intensity
    pub intensity: f32,
    
    /// Data integrity hash for verification
    pub data_hash: String,
}

/// Biometric snapshot from sensors
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BiometricSnapshot {
    /// EEG data
    pub eeg_data: Option<EEGData>,
    
    /// Heart rate data
    pub heart_rate: Option<HeartRateData>,
    
    /// Galvanic skin response
    pub gsr: Option<GSRData>,
    
    /// Facial expression analysis
    pub facial_data: Option<FacialData>,
    
    /// Data quality score
    pub quality_score: f32,
    
    /// IPFS hash of full biometric recording
    pub data_cid: String,
    
    /// Timestamp for verification
    pub timestamp: Timestamp,
}

/// EEG (Electroencephalography) data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EEGData {
    /// Alpha waves (8-13 Hz) - relaxation
    pub alpha: f32,
    
    /// Beta waves (13-30 Hz) - alertness, concentration
    pub beta: f32,
    
    /// Theta waves (4-8 Hz) - creativity, meditation
    pub theta: f32,
    
    /// Delta waves (0.5-4 Hz) - deep sleep
    pub delta: f32,
    
    /// Gamma waves (30-100 Hz) - peak concentration
    pub gamma: f32,
    
    /// Asymmetry (left-right brain)
    pub frontal_asymmetry: f32,
    
    /// Attention level
    pub attention: f32,
    
    /// Meditation level
    pub meditation: f32,
    
    /// Data integrity verification
    pub checksum: String,
}

/// Heart rate variability data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct HeartRateData {
    pub bpm: u32,
    pub hrv: f32, // Heart rate variability
    pub stress_index: f32,
    pub timestamp: Timestamp,
}

/// Galvanic Skin Response data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GSRData {
    pub conductance: f32,
    pub arousal_level: f32,
    pub timestamp: Timestamp,
}

/// Facial expression analysis
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FacialData {
    pub happiness: f32,
    pub sadness: f32,
    pub anger: f32,
    pub surprise: f32,
    pub fear: f32,
    pub disgust: f32,
    pub neutral: f32,
    pub timestamp: Timestamp,
}

/// Type of interaction
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum InteractionType {
    View,
    Meditation,
    CreativeSession,
    EmotionalSync,
    BiofeedbackLoop,
    ShaderEdit,
    FractalExplore,
}

/// Snapshot of visual state
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VisualStateSnapshot {
    pub zoom: f64,
    pub color_intensity: f32,
    pub animation_speed: f32,
    pub detail_level: u32,
    pub fractal_type: String,
    pub timestamp: Timestamp,
}

/// Biometric profile for a user
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BiometricProfile {
    pub account: AccountId,
    pub baseline_state: DetailedEmotionalState,
    pub interaction_count: u32,
    pub total_interaction_time: u64,
    pub favorite_states: Vec<VisualStateSnapshot>,
    pub authorized_at: Timestamp,
    pub permissions: Vec<String>,
}

/// Emotional resonance of the NFT
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalResonance {
    /// Accumulated emotional energy
    pub resonance_level: f32,
    
    /// Dominant emotion influencing the NFT
    pub dominant_emotion: String,
    
    /// Number of unique viewers
    pub unique_viewers: u32,
    
    /// Total interaction time
    pub total_engagement_seconds: u64,
    
    /// Average emotional intensity of interactions
    pub avg_intensity: f32,
    
    /// Emotional journey tracking
    pub emotional_journey: Vec<EmotionalJourneyPoint>,
    
    /// Last updated timestamp
    pub last_updated: Timestamp,
}

/// Emotional journey point for tracking evolution
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalJourneyPoint {
    pub timestamp: Timestamp,
    pub emotional_state: DetailedEmotionalState,
    pub interaction_type: InteractionType,
    pub visual_complexity: f32,
}

/// Interactive metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InteractiveMetadata {
    pub title: String,
    pub description: String,
    pub artist: AccountId,
    pub created_at: Timestamp,
    pub base_ipfs_cid: String,
    pub interaction_rules: InteractionRules,
    /// Data integrity information
    pub integrity_info: DataIntegrityInfo,
}

/// Data integrity information
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DataIntegrityInfo {
    pub content_hash: String,
    pub creation_timestamp: Timestamp,
    pub last_verified: Timestamp,
    pub verification_count: u32,
    pub is_verified: bool,
}

/// Rules for how NFT responds to emotions
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InteractionRules {
    /// Valence affects color warmth
    pub valence_affects_color: bool,
    
    /// Arousal affects animation speed
    pub arousal_affects_speed: bool,
    
    /// Dominance affects detail level
    pub dominance_affects_detail: bool,
    
    /// Meditation state affects fractal morphing
    pub meditation_affects_morphing: bool,
    
    /// Stress level affects complexity
    pub stress_affects_complexity: bool,
    
    /// Sensitivity multiplier
    pub sensitivity: f32,
    
    /// Shader customization allowed
    pub allow_shader_customization: bool,
    
    /// Fractal exploration allowed
    pub allow_fractal_exploration: bool,
}

/// Privacy settings
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PrivacySettings {
    pub store_biometric_data: bool,
    pub share_interaction_history: bool,
    pub allow_emotional_analytics: bool,
    pub anonymize_data: bool,
    pub allow_data_export: bool,
    pub data_retention_days: u32,
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

impl BiometricNFT {
    /// Create a new biometric NFT with enhanced data integrity
    pub fn new(
        token_id: String,
        owner: AccountId,
        metadata: InteractiveMetadata,
    ) -> Self {
        let visual_state = VisualState::default();
        let shader_params = ShaderParams::default();
        let fractal_params = FractalParams::default();
        
        // Calculate initial integrity hash with enhanced security
        let integrity_hash = Self::calculate_integrity_hash(&visual_state, &shader_params, &fractal_params);
        
        Self {
            token_id,
            owner,
            visual_state,
            interaction_history: Vector::new(b"h"),
            authorized_profiles: LookupMap::new(b"a"),
            emotional_resonance: EmotionalResonance::default(),
            metadata,
            privacy: PrivacySettings {
                store_biometric_data: true,
                share_interaction_history: false,
                allow_emotional_analytics: true,
                anonymize_data: false,
                allow_data_export: false,
                data_retention_days: 365,
            },
            shader_params,
            fractal_params,
            integrity_hash,
        }
    }

    /// Calculate integrity hash for data verification with enhanced security
    fn calculate_integrity_hash(visual_state: &VisualState, shader_params: &ShaderParams, fractal_params: &FractalParams) -> String {
        // Create a more comprehensive representation of the NFT state
        let visual_repr = format!("{:?}{:?}{:?}{:?}{:?}{:?}", 
            visual_state.fractal_type, 
            visual_state.zoom, 
            visual_state.color_intensity,
            visual_state.animation_speed,
            visual_state.morphing_rate,
            visual_state.detail_level);
        
        let shader_repr = format!("{:?}{:?}{:?}{:?}{:?}{:?}", 
            shader_params.time, 
            shader_params.resolution,
            shader_params.emotional_params.valence,
            shader_params.emotional_params.arousal,
            shader_params.emotional_params.dominance,
            shader_params.custom_uniforms.len());
        
        let fractal_repr = format!("{:?}{:?}{:?}{:?}{:?}{:?}", 
            fractal_params.fractal_type, 
            fractal_params.zoom, 
            fractal_params.max_iterations,
            fractal_params.emotional_modulation.valence_influence,
            fractal_params.emotional_modulation.arousal_influence,
            fractal_params.color_palette.len());
        
        // Include timestamp and account information for additional security
        let security_repr = format!("{}{}{}", 
            env::block_timestamp(),
            env::predecessor_account_id(),
            env::current_account_id());
        
        // Combine all representations
        let combined = format!("{}{}{}{}{}", 
            visual_repr, 
            shader_repr, 
            fractal_repr,
            security_repr,
            // Add a salt for additional security
            "NUWE_INTEGRITY_SALT_2025");
        
        // Use SHA-256 for better security than MD5
        let mut hasher = sha2::Sha256::new();
        hasher.update(combined.as_bytes());
        let result = hasher.finalize();
        
        // Convert to hex string
        format!("{:x}", result)
    }

    /// Verify data integrity with enhanced validation
    pub fn verify_integrity(&self) -> (bool, String) {
        let current_hash = Self::calculate_integrity_hash(&self.visual_state, &self.shader_params, &self.fractal_params);
        
        if current_hash == self.integrity_hash {
            (true, "Data integrity verified".to_string())
        } else {
            (false, "Data integrity check failed".to_string())
        }
    }

    /// Update integrity hash after state changes
    fn update_integrity_hash(&mut self) {
        self.integrity_hash = Self::calculate_integrity_hash(&self.visual_state, &self.shader_params, &self.fractal_params);
    }

    /// Interact with NFT using real-time biometric data with enhanced data integrity
    pub fn interact_with_biometrics(
        &mut self,
        emotional_state: DetailedEmotionalState,
        biometric_data: BiometricSnapshot,
        interaction_type: InteractionType,
    ) {
        let user = env::predecessor_account_id();
        
        // Verify integrity before interaction
        let (is_valid, message) = self.verify_integrity();
        if !is_valid {
            env::log_str(&format!("Integrity check failed before interaction: {}", message));
        }
        
        // Capture state before interaction
        let state_before = self.capture_state_snapshot();
        
        // Apply emotional modulation to visual state with enhanced biometric integration
        self.apply_emotional_modulation(&emotional_state, &biometric_data);
        
        // Update shader and fractal parameters based on emotional state
        self.update_shader_params(&emotional_state);
        self.update_fractal_params(&emotional_state);
        
        // Apply biometric-specific enhancements
        self.apply_biometric_enhancements(&biometric_data);
        
        // Capture state after interaction
        let state_after = self.capture_state_snapshot();
        
        // Create emotional state hash for verification
        let emotional_hash = self.calculate_emotional_state_hash(&emotional_state);
        
        // Record interaction with enhanced biometric data and integrity verification
        let interaction = EmotionalInteraction {
            timestamp: env::block_timestamp(),
            user: user.clone(),
            emotional_state: DetailedEmotionalState {
                data_hash: emotional_hash,
                ..emotional_state
            },
            biometric_data: BiometricSnapshot {
                data_cid: biometric_data.data_cid.clone(),
                checksum: self.calculate_biometric_checksum(&biometric_data),
                ..biometric_data
            },
            interaction_type,
            state_before,
            state_after,
            shader_params: self.shader_params.clone(),
            fractal_params: self.fractal_params.clone(),
        };
        
        self.interaction_history.push(&interaction);
        
        // Update emotional resonance
        self.update_resonance(&interaction);
        
        // Update integrity hash
        self.update_integrity_hash();
        
        // Log successful interaction
        env::log_str(&format!("Successfully recorded interaction for NFT {} by user {}", self.token_id, user));
    }

    /// Apply biometric-specific enhancements to visual parameters
    fn apply_biometric_enhancements(&mut self, biometric: &BiometricSnapshot) {
        // Apply EEG-specific enhancements
        if let Some(ref eeg) = biometric.eeg_data {
            // Alpha waves (8-13 Hz) - relaxation
            if eeg.alpha > 0.6 {
                // High alpha indicates relaxation, smooth animations
                self.visual_state.animation_speed *= 0.8;
                self.visual_state.morphing_rate *= 1.2;
            }
            
            // Beta waves (13-30 Hz) - alertness, concentration
            if eeg.beta > 0.7 {
                // High beta indicates focus, increase detail
                self.visual_state.detail_level = (self.visual_state.detail_level as f32 * 1.3) as u32;
            }
            
            // Theta waves (4-8 Hz) - creativity, meditation
            if eeg.theta > 0.5 {
                // High theta indicates creativity, add artistic effects
                self.visual_state.color_palette.push(ColorRGB { r: 255, g: 215, b: 0 }); // Gold for creativity
            }
            
            // Gamma waves (30-100 Hz) - peak concentration
            if eeg.gamma > 0.6 {
                // High gamma indicates peak concentration, enhance visual complexity
                self.visual_state.detail_level = (self.visual_state.detail_level as f32 * 1.5) as u32;
            }
            
            // Frontal asymmetry - emotional valence indicator
            if eeg.frontal_asymmetry > 0.3 {
                // Left frontal dominance indicates positive emotions
                self.apply_nuwe_color_shift(0.5); // Shift toward warm colors
            } else if eeg.frontal_asymmetry < -0.3 {
                // Right frontal dominance indicates negative emotions
                self.apply_nuwe_color_shift(-0.5); // Shift toward cool colors
            }
        }
        
        // Apply heart rate variability enhancements
        if let Some(ref hrv) = biometric.heart_rate {
            // HRV is an indicator of stress and emotional regulation
            if hrv.hrv > 50.0 {
                // High HRV indicates good emotional regulation, smooth transitions
                self.visual_state.morphing_rate *= 1.3;
                self.visual_state.smoothness = 0.8;
            } else if hrv.hrv < 20.0 {
                // Low HRV indicates stress, add stress visualization
                self.visual_state.shader_uniforms.push(ShaderUniform {
                    name: "stress_indicator".to_string(),
                    value: 0.7,
                });
            }
            
            // Heart rate affects pulse effects
            self.visual_state.pulse_rate = hrv.bpm as f32 / 60.0;
        }
        
        // Apply GSR enhancements
        if let Some(ref gsr) = biometric.gsr {
            // GSR indicates arousal level
            if gsr.arousal_level > 0.7 {
                // High arousal, increase animation intensity
                self.visual_state.animation_speed *= 1.5;
                self.visual_state.color_intensity *= 1.3;
            } else if gsr.arousal_level < 0.3 {
                // Low arousal, create calming effects
                self.visual_state.animation_speed *= 0.7;
                self.visual_state.color_intensity *= 0.8;
            }
        }
        
        // Apply facial expression analysis
        if let Some(ref facial) = biometric.facial_data {
            // Detect primary emotion from facial expressions
            let primary_emotion = self.detect_primary_emotion(facial);
            
            // Apply emotion-specific visual effects
            match primary_emotion.as_str() {
                "happiness" => {
                    // Bright, warm colors for happiness
                    self.visual_state.color_palette.clear();
                    self.visual_state.color_palette.push(ColorRGB { r: 255, g: 255, b: 0 }); // Yellow
                    self.visual_state.color_palette.push(ColorRGB { r: 255, g: 165, b: 0 }); // Orange
                    self.visual_state.animation_speed *= 1.2;
                }
                "sadness" => {
                    // Cool, muted colors for sadness
                    self.visual_state.color_palette.clear();
                    self.visual_state.color_palette.push(ColorRGB { r: 0, g: 0, b: 139 }); // Dark blue
                    self.visual_state.color_palette.push(ColorRGB { r: 70, g: 130, b: 180 }); // Steel blue
                    self.visual_state.animation_speed *= 0.8;
                }
                "anger" => {
                    // Red, intense colors for anger
                    self.visual_state.color_palette.clear();
                    self.visual_state.color_palette.push(ColorRGB { r: 255, g: 0, b: 0 }); // Red
                    self.visual_state.color_palette.push(ColorRGB { r: 139, g: 0, b: 0 }); // Dark red
                    self.visual_state.animation_speed *= 1.5;
                }
                "surprise" => {
                    // Bright, contrasting colors for surprise
                    self.visual_state.color_palette.clear();
                    self.visual_state.color_palette.push(ColorRGB { r: 255, g: 255, b: 255 }); // White
                    self.visual_state.color_palette.push(ColorRGB { r: 255, g: 215, b: 0 }); // Gold
                    self.visual_state.animation_speed *= 2.0;
                }
                _ => {}
            }
        }
    }

    /// Detect primary emotion from facial expression data
    fn detect_primary_emotion(&self, facial: &FacialData) -> String {
        let emotions = [
            ("happiness", facial.happiness),
            ("sadness", facial.sadness),
            ("anger", facial.anger),
            ("surprise", facial.surprise),
            ("fear", facial.fear),
            ("disgust", facial.disgust),
            ("neutral", facial.neutral),
        ];
        
        // Find emotion with highest confidence
        let mut max_confidence = 0.0;
        let mut primary_emotion = "neutral";
        
        for (emotion, confidence) in emotions {
            if confidence > max_confidence {
                max_confidence = confidence;
                primary_emotion = emotion;
            }
        }
        
        primary_emotion.to_string()
    }

    /// Apply emotional modulation to visual parameters with enhanced NUWE integration
    fn apply_emotional_modulation(
        &mut self,
        emotion: &DetailedEmotionalState,
        biometric: &BiometricSnapshot,
    ) {
        let rules = &self.metadata.interaction_rules;
        let sensitivity = rules.sensitivity;
        
        // Valence affects color warmth with NUWE color theory
        if rules.valence_affects_color {
            self.visual_state.color_intensity = 
                0.5 + (emotion.valence * sensitivity * 0.5);
            
            // Apply NUWE color palette shifting based on emotional valence
            self.apply_nuwe_color_shift(emotion.valence);
        }
        
        // Arousal affects animation speed with NUWE motion dynamics
        if rules.arousal_affects_speed {
            self.visual_state.animation_speed = 
                0.5 + (emotion.arousal * sensitivity);
            
            // Apply NUWE motion enhancement
            self.apply_nuwe_motion_effects(emotion.arousal);
        }
        
        // Dominance affects detail level with NUWE complexity scaling
        if rules.dominance_affects_detail {
            self.visual_state.detail_level = 
                (50.0 + emotion.dominance * 150.0) as u32;
            
            // Apply NUWE fractal complexity enhancement
            self.apply_nuwe_complexity_scaling(emotion.dominance);
        }
        
        // Meditation (from EEG) affects morphing with NUWE transformation dynamics
        if rules.meditation_affects_morphing {
            if let Some(ref eeg) = biometric.eeg_data {
                self.visual_state.morphing_rate = eeg.meditation * sensitivity;
                
                // Apply NUWE morphing enhancement
                self.apply_nuwe_morphing_effects(eeg.meditation);
            }
        }
        
        // Stress affects complexity with NUWE stress response
        if rules.stress_affects_complexity {
            let complexity_reduction = emotion.stress * 0.5;
            self.visual_state.detail_level = 
                (self.visual_state.detail_level as f32 * (1.0 - complexity_reduction)) as u32;
            
            // Apply NUWE stress visualization
            self.apply_nuwe_stress_effects(emotion.stress);
        }
        
        // Engagement affects color palette with NUWE engagement theory
        if emotion.engagement > 0.7 {
            // Expand color palette for high engagement
            self.visual_state.color_palette.push(ColorRGB { r: 255, g: 0, b: 0 }); // Add red for high engagement
            self.visual_state.color_palette.push(ColorRGB { r: 0, g: 255, b: 0 }); // Add green for high engagement
        }
        
        // Update emotional modulation parameters with enhanced NUWE integration
        self.visual_state.emotional_modulation.valence_influence = (emotion.valence.abs() * 0.5 + 0.3).clamp(0.3, 1.0);
        self.visual_state.emotional_modulation.arousal_influence = emotion.arousal.clamp(0.1, 1.0);
        self.visual_state.emotional_modulation.dominance_influence = (emotion.dominance * 0.5 + 0.2).clamp(0.2, 0.8);
        self.visual_state.emotional_modulation.stress_factor = (1.0 - emotion.arousal).clamp(0.0, 0.5);
        self.visual_state.emotional_modulation.engagement_boost = (emotion.arousal * 0.5 + 0.8).clamp(0.8, 1.5);
    }

    /// Apply NUWE color shifting based on emotional valence
    fn apply_nuwe_color_shift(&mut self, valence: f32) {
        // NUWE color theory: positive valence shifts toward warm colors, negative toward cool
        for color in &mut self.visual_state.color_palette {
            if valence > 0.0 {
                // Enhance reds and yellows for positive emotions
                color.r = (color.r as f32 * (1.0 + valence * 0.4)).min(255.0) as u8;
                color.g = (color.g as f32 * (1.0 + valence * 0.2)).min(255.0) as u8;
            } else {
                // Enhance blues and purples for negative emotions
                color.b = (color.b as f32 * (1.0 - valence * 0.4)).min(255.0) as u8;
                // Reduce reds for negative emotions
                color.r = (color.r as f32 * (1.0 + valence * 0.3)).max(0.0) as u8;
            }
        }
    }

    /// Apply NUWE motion effects based on arousal
    fn apply_nuwe_motion_effects(&mut self, arousal: f32) {
        // NUWE motion dynamics: higher arousal increases motion complexity
        self.visual_state.animation_speed *= 1.0 + arousal * 0.5;
        
        // Add secondary motion effects for high arousal
        if arousal > 0.7 {
            // Add subtle secondary animation
            self.visual_state.shader_uniforms.push(ShaderUniform {
                name: "secondary_motion".to_string(),
                value: arousal * 0.3,
            });
        }
    }

    /// Apply NUWE complexity scaling based on dominance
    fn apply_nuwe_complexity_scaling(&mut self, dominance: f32) {
        // NUWE complexity theory: dominance increases structural complexity
        self.visual_state.detail_level = (self.visual_state.detail_level as f32 * (1.0 + dominance * 0.3)) as u32;
        
        // Add complexity uniform for shaders
        self.visual_state.shader_uniforms.push(ShaderUniform {
            name: "structural_complexity".to_string(),
            value: dominance * 0.5,
        });
    }

    /// Apply NUWE morphing effects based on meditation
    fn apply_nuwe_morphing_effects(&mut self, meditation: f32) {
        // NUWE transformation dynamics: meditation creates smooth transitions
        self.visual_state.morphing_rate *= 1.0 + meditation * 0.7;
        
        // Add morphing uniform for shaders
        self.visual_state.shader_uniforms.push(ShaderUniform {
            name: "transformation_smoothness".to_string(),
            value: meditation * 0.4,
        });
    }

    /// Apply NUWE stress visualization
    fn apply_nuwe_stress_effects(&mut self, stress: f32) {
        // NUWE stress response: visualize stress through visual distortion
        if stress > 0.5 {
            self.visual_state.shader_uniforms.push(ShaderUniform {
                name: "stress_level".to_string(),
                value: stress * 0.6,
            });
            
            // Reduce color saturation under stress
            self.visual_state.color_intensity *= 1.0 - stress * 0.3;
        }
    }

    /// Update shader parameters based on emotional state with NUWE integration
    fn update_shader_params(&mut self, emotion: &DetailedEmotionalState) {
        self.shader_params.emotional_params.valence = emotion.valence;
        self.shader_params.emotional_params.arousal = emotion.arousal;
        self.shader_params.emotional_params.dominance = emotion.dominance;
        
        // Add extended emotional parameters for NUWE integration
        self.shader_params.custom_uniforms.push(UniformParam {
            name: "engagement".to_string(),
            value_type: UniformType::Float,
            value: vec![emotion.engagement],
        });
        
        self.shader_params.custom_uniforms.push(UniformParam {
            name: "focus".to_string(),
            value_type: UniformType::Float,
            value: vec![emotion.focus],
        });
        
        self.shader_params.custom_uniforms.push(UniformParam {
            name: "stress".to_string(),
            value_type: UniformType::Float,
            value: vec![emotion.stress],
        });
        
        self.shader_params.custom_uniforms.push(UniformParam {
            name: "relaxation".to_string(),
            value_type: UniformType::Float,
            value: vec![emotion.relaxation],
        });
        
        // Update time parameter
        self.shader_params.time = (env::block_timestamp() as f32) / 1_000_000_000.0;
    }

    /// Update fractal parameters based on emotional state with enhanced NUWE integration
    fn update_fractal_params(&mut self, emotion: &DetailedEmotionalState) {
        // Apply emotional modulation to fractal parameters with NUWE enhancements
        self.fractal_params.emotional_modulation.valence_influence = (emotion.valence.abs() * 0.5 + 0.3).clamp(0.3, 1.0);
        self.fractal_params.emotional_modulation.arousal_influence = emotion.arousal.clamp(0.1, 1.0);
        self.fractal_params.emotional_modulation.dominance_influence = (emotion.dominance * 0.5 + 0.2).clamp(0.2, 0.8);
        self.fractal_params.emotional_modulation.stress_factor = (1.0 - emotion.arousal).clamp(0.0, 0.5);
        self.fractal_params.emotional_modulation.engagement_boost = (emotion.arousal * 0.5 + 0.8).clamp(0.8, 1.5);
        
        // Arousal affects iteration count (more arousal = more detail) with NUWE scaling
        self.fractal_params.max_iterations = (100.0 + emotion.arousal * 300.0) as u32;
        
        // Dominance affects zoom (more dominance = more zoom) with NUWE perspective
        self.fractal_params.zoom *= 1.0 + (emotion.dominance * 0.2) as f64;
        
        // Stress affects complexity with NUWE stress response
        self.fractal_params.max_iterations = (self.fractal_params.max_iterations as f32 * (1.0 - emotion.stress * 0.4)) as u32;
        
        // Engagement affects color palette with NUWE engagement theory
        if emotion.engagement > 0.7 {
            // Expand color palette for high engagement
            self.fractal_params.color_palette.push(0xFF0000); // Add red for high engagement
            self.fractal_params.color_palette.push(0x00FF00); // Add green for high engagement
        }
    }

    /// Capture current visual state
    fn capture_state_snapshot(&self) -> VisualStateSnapshot {
        VisualStateSnapshot {
            zoom: self.visual_state.zoom,
            color_intensity: self.visual_state.color_intensity,
            animation_speed: self.visual_state.animation_speed,
            detail_level: self.visual_state.detail_level,
            fractal_type: self.visual_state.fractal_type.clone(),
            timestamp: env::block_timestamp(),
        }
    }

    /// Update emotional resonance
    fn update_resonance(&mut self, interaction: &EmotionalInteraction) {
        self.emotional_resonance.resonance_level += 
            interaction.emotional_state.intensity * 0.1;
        
        self.emotional_resonance.dominant_emotion = 
            interaction.emotional_state.primary_emotion.clone();
        
        let n = self.emotional_resonance.avg_intensity;
        self.emotional_resonance.avg_intensity = 
            (n * 0.9) + (interaction.emotional_state.intensity * 0.1);
            
        // Add to emotional journey
        let journey_point = EmotionalJourneyPoint {
            timestamp: interaction.timestamp,
            emotional_state: interaction.emotional_state.clone(),
            interaction_type: interaction.interaction_type.clone(),
            visual_complexity: self.calculate_visual_complexity(),
        };
        
        self.emotional_resonance.emotional_journey.push(journey_point);
        self.emotional_resonance.last_updated = env::block_timestamp();
    }

    /// Calculate visual complexity score
    fn calculate_visual_complexity(&self) -> f32 {
        let color_complexity = self.visual_state.color_palette.len() as f32 / 10.0;
        let detail_complexity = self.visual_state.detail_level as f32 / 100.0;
        let animation_complexity = self.visual_state.animation_speed;
        
        (color_complexity + detail_complexity + animation_complexity) / 3.0
    }

    /// Calculate hash for emotional state verification
    fn calculate_emotional_state_hash(&self, emotional_state: &DetailedEmotionalState) -> String {
        let emotional_repr = format!("{:?}{:?}{:?}{:?}{:?}{:?}{:?}{:?}", 
            emotional_state.valence,
            emotional_state.arousal,
            emotional_state.dominance,
            emotional_state.engagement,
            emotional_state.focus,
            emotional_state.stress,
            emotional_state.relaxation,
            emotional_state.intensity);
        
        let mut hasher = sha2::Sha256::new();
        hasher.update(emotional_repr.as_bytes());
        let result = hasher.finalize();
        
        format!("{:x}", result)
    }

    /// Calculate checksum for biometric data verification
    fn calculate_biometric_checksum(&self, biometric_data: &BiometricSnapshot) -> String {
        let biometric_repr = format!("{:?}{:?}{:?}{:?}{:?}{}", 
            biometric_data.eeg_data,
            biometric_data.heart_rate,
            biometric_data.gsr,
            biometric_data.facial_data,
            biometric_data.quality_score,
            biometric_data.timestamp);
        
        let mut hasher = sha2::Sha256::new();
        hasher.update(biometric_repr.as_bytes());
        let result = hasher.finalize();
        
        format!("{:x}", result)
    }

    /// Export NFT data with integrity verification
    pub fn export_data(&self) -> String {
        // Verify integrity before export
        let (is_valid, _) = self.verify_integrity();
        
        if !is_valid {
            env::log_str("Warning: Data integrity check failed during export");
        }
        
        // Create export data structure
        let export_data = serde_json::json!({
            "token_id": self.token_id,
            "owner": self.owner,
            "visual_state": self.visual_state,
            "interaction_history": self.get_interaction_summary(),
            "emotional_resonance": self.emotional_resonance,
            "metadata": self.metadata,
            "shader_params": self.shader_params,
            "fractal_params": self.fractal_params,
            "integrity_hash": self.integrity_hash,
            "export_timestamp": env::block_timestamp(),
            "data_valid": is_valid
        });
        
        export_data.to_string()
    }

    /// Get interaction history summary for privacy compliance
    fn get_interaction_summary(&self) -> Vec<serde_json::Value> {
        let mut summary = Vec::new();
        
        // Only include non-sensitive data in summary
        for i in 0..self.interaction_history.len() {
            if let Some(interaction) = self.interaction_history.get(i) {
                let summary_entry = serde_json::json!({
                    "timestamp": interaction.timestamp,
                    "interaction_type": interaction.interaction_type,
                    "emotional_intensity": interaction.emotional_state.intensity,
                    "visual_complexity_before": self.calculate_visual_complexity_from_snapshot(&interaction.state_before),
                    "visual_complexity_after": self.calculate_visual_complexity_from_snapshot(&interaction.state_after),
                    "data_hash": interaction.emotional_state.data_hash
                });
                summary.push(summary_entry);
            }
        }
        
        summary
    }

    /// Calculate visual complexity from snapshot for summary
    fn calculate_visual_complexity_from_snapshot(&self, snapshot: &VisualStateSnapshot) -> f32 {
        let color_complexity = 0.5; // Simplified for summary
        let detail_complexity = snapshot.detail_level as f32 / 100.0;
        let animation_complexity = snapshot.animation_speed;
        
        (color_complexity + detail_complexity + animation_complexity) / 3.0
    }

    /// Verify interaction history integrity
    pub fn verify_interaction_history(&self) -> (bool, String) {
        // Verify each interaction's emotional state hash
        for i in 0..self.interaction_history.len() {
            if let Some(interaction) = self.interaction_history.get(i) {
                let expected_hash = self.calculate_emotional_state_hash(&interaction.emotional_state);
                if expected_hash != interaction.emotional_state.data_hash {
                    return (false, format!("Emotional state hash mismatch at interaction {}", i));
                }
            }
        }
        
        (true, "Interaction history integrity verified".to_string())
    }

    /// Get data integrity report
    pub fn get_integrity_report(&self) -> serde_json::Value {
        let (main_valid, main_message) = self.verify_integrity();
        let (history_valid, history_message) = self.verify_interaction_history();
        
        serde_json::json!({
            "token_id": self.token_id,
            "main_data_valid": main_valid,
            "main_data_message": main_message,
            "history_valid": history_valid,
            "history_message": history_message,
            "integrity_hash": self.integrity_hash,
            "interaction_count": self.interaction_history.len(),
            "last_updated": self.emotional_resonance.last_updated,
            "report_timestamp": env::block_timestamp()
        })
    }
}

impl Default for VisualState {
    fn default() -> Self {
        Self {
            fractal_type: "mandelbrot".to_string(),
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            color_palette: vec![
                ColorRGB { r: 0, g: 0, b: 0 },
                ColorRGB { r: 255, g: 255, b: 255 },
            ],
            color_intensity: 0.5,
            animation_speed: 1.0,
            morphing_rate: 0.0,
            detail_level: 100,
            shader_uniforms: Vec::new(),
            emotional_modulation: EmotionalModulation {
                valence_influence: 0.5,
                arousal_influence: 0.3,
                dominance_influence: 0.2,
                engagement_boost: 1.0,
                stress_factor: 0.0,
            },
        }
    }
}

impl Default for EmotionalResonance {
    fn default() -> Self {
        Self {
            resonance_level: 0.0,
            dominant_emotion: "neutral".to_string(),
            unique_viewers: 0,
            total_engagement_seconds: 0,
            avg_intensity: 0.0,
            emotional_journey: Vec::new(),
            last_updated: env::block_timestamp(),
        }
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

impl Default for FractalParams {
    fn default() -> Self {
        Self {
            fractal_type: "Mandelbrot".to_string(),
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            max_iterations: 100,
            color_palette: vec![0x000000, 0xFFFFFF],
            julia_c_real: None,
            julia_c_imag: None,
            time_offset: 0.0,
            emotional_modulation: EmotionalModulationParams {
                valence_influence: 0.5,
                arousal_influence: 0.3,
                dominance_influence: 0.2,
                engagement_boost: 1.0,
                stress_factor: 0.0,
            },
        }
    }
}

impl Default for DetailedEmotionalState {
    fn default() -> Self {
        Self {
            valence: 0.0,
            arousal: 0.5,
            dominance: 0.5,
            engagement: 0.0,
            focus: 0.0,
            stress: 0.0,
            relaxation: 0.0,
            confidence: 0.8,
            primary_emotion: "neutral".to_string(),
            intensity: 0.0,
            data_hash: "".to_string(),
        }
    }
}
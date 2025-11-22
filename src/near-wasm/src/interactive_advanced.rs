//! Advanced Interactive NFT with EEG/BMI Integration
//! 
//! Revolutionary NFT system that responds to real-time emotional states
//! from brain-computer interfaces and biometric sensors

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near, AccountId, Timestamp};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};

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
}

/// Heart rate variability data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct HeartRateData {
    pub bpm: u32,
    pub hrv: f32, // Heart rate variability
    pub stress_index: f32,
}

/// Galvanic Skin Response data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GSRData {
    pub conductance: f32,
    pub arousal_level: f32,
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
}

/// Snapshot of visual state
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VisualStateSnapshot {
    pub zoom: f64,
    pub color_intensity: f32,
    pub animation_speed: f32,
    pub detail_level: u32,
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
}

/// Privacy settings
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PrivacySettings {
    pub store_biometric_data: bool,
    pub share_interaction_history: bool,
    pub allow_emotional_analytics: bool,
    pub anonymize_data: bool,
}

impl BiometricNFT {
    /// Create a new biometric NFT
    pub fn new(
        token_id: String,
        owner: AccountId,
        metadata: InteractiveMetadata,
    ) -> Self {
        Self {
            token_id,
            owner,
            visual_state: VisualState::default(),
            interaction_history: Vector::new(b"h"),
            authorized_profiles: LookupMap::new(b"a"),
            emotional_resonance: EmotionalResonance::default(),
            metadata,
            privacy: PrivacySettings {
                store_biometric_data: true,
                share_interaction_history: false,
                allow_emotional_analytics: true,
                anonymize_data: false,
            },
        }
    }

    /// Interact with NFT using real-time biometric data
    pub fn interact_with_biometrics(
        &mut self,
        emotional_state: DetailedEmotionalState,
        biometric_data: BiometricSnapshot,
        interaction_type: InteractionType,
    ) {
        let user = env::predecessor_account_id();
        
        // Capture state before interaction
        let state_before = self.capture_state_snapshot();
        
        // Apply emotional modulation to visual state
        self.apply_emotional_modulation(&emotional_state, &biometric_data);
        
        // Capture state after interaction
        let state_after = self.capture_state_snapshot();
        
        // Record interaction
        let interaction = EmotionalInteraction {
            timestamp: env::block_timestamp(),
            user: user.clone(),
            emotional_state,
            biometric_data,
            interaction_type,
            state_before,
            state_after,
        };
        
        self.interaction_history.push(&interaction);
        
        // Update emotional resonance
        self.update_resonance(&interaction);
    }

    /// Apply emotional modulation to visual parameters
    fn apply_emotional_modulation(
        &mut self,
        emotion: &DetailedEmotionalState,
        biometric: &BiometricSnapshot,
    ) {
        let rules = &self.metadata.interaction_rules;
        let sensitivity = rules.sensitivity;
        
        // Valence affects color warmth
        if rules.valence_affects_color {
            self.visual_state.color_intensity = 
                0.5 + (emotion.valence * sensitivity * 0.5);
            
            // Shift color palette based on valence
            for color in &mut self.visual_state.color_palette {
                if emotion.valence > 0.0 {
                    // Warm colors for positive valence
                    color.r = (color.r as f32 * (1.0 + emotion.valence * 0.3)) as u8;
                } else {
                    // Cool colors for negative valence
                    color.b = (color.b as f32 * (1.0 - emotion.valence * 0.3)) as u8;
                }
            }
        }
        
        // Arousal affects animation speed
        if rules.arousal_affects_speed {
            self.visual_state.animation_speed = 
                0.5 + (emotion.arousal * sensitivity);
        }
        
        // Dominance affects detail level
        if rules.dominance_affects_detail {
            self.visual_state.detail_level = 
                (50.0 + emotion.dominance * 150.0) as u32;
        }
        
        // Meditation (from EEG) affects morphing
        if rules.meditation_affects_morphing {
            if let Some(ref eeg) = biometric.eeg_data {
                self.visual_state.morphing_rate = eeg.meditation * sensitivity;
            }
        }
        
        // Stress affects complexity
        if rules.stress_affects_complexity {
            let complexity_reduction = emotion.stress * 0.5;
            self.visual_state.detail_level = 
                (self.visual_state.detail_level as f32 * (1.0 - complexity_reduction)) as u32;
        }
    }

    /// Capture current visual state
    fn capture_state_snapshot(&self) -> VisualStateSnapshot {
        VisualStateSnapshot {
            zoom: self.visual_state.zoom,
            color_intensity: self.visual_state.color_intensity,
            animation_speed: self.visual_state.animation_speed,
            detail_level: self.visual_state.detail_level,
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
        }
    }
}

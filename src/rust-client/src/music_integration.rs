//! # Music Integration Module
//!
//! This module integrates audio processing and music generation capabilities
//! using the tunes crate for music generation and rodio for audio playback
//! within the NFT blockchain interactive framework.
//! 
//! Reference patterns adapted from bevy_audio for robust audio handling and
//! emotional parameter mapping inspired by creative computing frameworks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[cfg(feature = "audio")]
use tunes::{Note, Scale, Chord, Progression, Rhythm, Instrument, Composition};
#[cfg(feature = "audio")]
use rodio::{OutputStream, Sink, Source, Sample};
#[cfg(feature = "audio")]
use std::sync::Arc;
#[cfg(feature = "audio")]
use std::time::Duration;

/// Configuration for music generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicConfig {
    pub tempo: f32,
    pub key: String,
    pub scale: String,
    pub complexity: f32,
    pub emotional_mapping: EmotionalMusicMapping,
}

impl Default for MusicConfig {
    fn default() -> Self {
        Self {
            tempo: 120.0,
            key: "C".to_string(),
            scale: "major".to_string(),
            complexity: 0.5,
            emotional_mapping: EmotionalMusicMapping::default(),
        }
    }
}

/// Custom audio source for generated music
#[cfg(feature = "audio")]
pub struct MusicSource {
    data: Vec<i16>,
    sample_rate: u32,
    current_sample: usize,
}

#[cfg(feature = "audio")]
impl MusicSource {
    pub fn new(audio_data: &[u8], sample_rate: u32) -> Self {
        // Convert byte data to i16 samples
        let mut data = Vec::new();
        for chunk in audio_data.chunks_exact(2) {
            let sample = i16::from_le_bytes([chunk[0], chunk[1]]);
            data.push(sample);
        }
        
        Self {
            data,
            sample_rate,
            current_sample: 0,
        }
    }
}

#[cfg(feature = "audio")]
impl Iterator for MusicSource {
    type Item = i16;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample < self.data.len() {
            let sample = self.data[self.current_sample];
            self.current_sample += 1;
            Some(sample)
        } else {
            None
        }
    }
}

#[cfg(feature = "audio")]
impl Source for MusicSource {
    fn current_frame_len(&self) -> Option<usize> {
        Some(self.data.len() - self.current_sample)
    }
    
    fn channels(&self) -> u16 {
        1 // Mono
    }
    
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    
    fn total_duration(&self) -> Option<std::time::Duration> {
        let samples_remaining = self.data.len() - self.current_sample;
        let seconds = samples_remaining as f64 / self.sample_rate as f64;
        Some(std::time::Duration::from_secs_f64(seconds))
    }
}

/// Mapping of emotions to musical parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalMusicMapping {
    pub valence_to_key: HashMap<String, String>,
    pub arousal_to_tempo: HashMap<String, f32>,
    pub dominance_to_complexity: HashMap<String, f32>,
}

impl Default for EmotionalMusicMapping {
    fn default() -> Self {
        let mut valence_to_key = HashMap::new();
        valence_to_key.insert("happy".to_string(), "C".to_string());
        valence_to_key.insert("sad".to_string(), "A".to_string());
        valence_to_key.insert("excited".to_string(), "G".to_string());
        valence_to_key.insert("calm".to_string(), "F".to_string());

        let mut arousal_to_tempo = HashMap::new();
        arousal_to_tempo.insert("low".to_string(), 60.0);
        arousal_to_tempo.insert("medium".to_string(), 120.0);
        arousal_to_tempo.insert("high".to_string(), 180.0);

        let mut dominance_to_complexity = HashMap::new();
        dominance_to_complexity.insert("simple".to_string(), 0.3);
        dominance_to_complexity.insert("moderate".to_string(), 0.6);
        dominance_to_complexity.insert("complex".to_string(), 0.9);

        Self {
            valence_to_key,
            arousal_to_tempo,
            dominance_to_complexity,
        }
    }
}

/// Generated music data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedMusic {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub config: MusicConfig,
    pub emotional_input: EmotionalInput,
    pub audio_data: Vec<u8>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Emotional input for music generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalInput {
    pub valence: f32,    // -1.0 to 1.0 (negative to positive)
    pub arousal: f32,    // 0.0 to 1.0 (calm to excited)
    pub dominance: f32,  // 0.0 to 1.0 (submissive to dominant)
}

/// Music generation engine
pub struct MusicEngine {
    config: MusicConfig,
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    sink: Option<Arc<Sink>>,
}

impl MusicEngine {
    /// Create a new music engine with default configuration
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match Self::with_config(MusicConfig::default()) {
                Ok(engine) => engine,
                Err(_) => Self {
                    config: MusicConfig::default(),
                    _stream: None,
                    sink: None,
                }
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            Self {
                config: MusicConfig::default(),
            }
        }
    }

    /// Create a new music engine with custom configuration
    pub fn with_config(config: MusicConfig) -> Result<Self, Box<dyn std::error::Error>> {
        #[cfg(feature = "audio")]
        {
            let (stream, stream_handle) = OutputStream::try_default()?;
            let sink = Arc::new(Sink::try_new(&stream_handle)?);
            
            Ok(Self {
                config,
                _stream: Some(stream),
                sink: Some(sink),
            })
        }
        #[cfg(not(feature = "audio"))]
        {
            Ok(Self { config })
        }
    }
    
    /// Play generated music
    #[cfg(feature = "audio")]
    pub fn play_music(&self, music: &GeneratedMusic) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(sink) = &self.sink {
            // Create a source from the audio data
            let source = MusicSource::new(&music.audio_data, 44100);
            sink.append(source);
            Ok(())
        } else {
            Err("Audio system not available".into())
        }
    }
    
    /// Stop playback
    #[cfg(feature = "audio")]
    pub fn stop(&self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
    }
    
    /// Check if currently playing
    #[cfg(feature = "audio")]
    pub fn is_playing(&self) -> bool {
        if let Some(sink) = &self.sink {
            !sink.empty()
        } else {
            false
        }
    }

    /// Generate music based on emotional input
    pub fn generate_music_from_emotion(&self, emotional_input: EmotionalInput) -> Result<GeneratedMusic, Box<dyn std::error::Error>> {
        // Map emotions to musical parameters
        let tempo = self.map_arousal_to_tempo(emotional_input.arousal);
        let key = self.map_valence_to_key(emotional_input.valence);
        let complexity = self.map_dominance_to_complexity(emotional_input.dominance);

        // Create music configuration based on emotional input
        let mut config = self.config.clone();
        config.tempo = tempo;
        config.key = key;
        config.complexity = complexity;

        // Generate audio data (placeholder - would use tunes crate in real implementation)
        let audio_data = self.generate_audio_data(&config)?;

        let generated_music = GeneratedMusic {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            config,
            emotional_input,
            audio_data,
            metadata: self.create_metadata(&emotional_input),
        };

        Ok(generated_music)
    }

    /// Map emotional valence to musical key
    fn map_valence_to_key(&self, valence: f32) -> String {
        let valence_clamped = valence.clamp(-1.0, 1.0);
        
        if valence_clamped > 0.5 {
            "C".to_string() // Happy, positive
        } else if valence_clamped > 0.0 {
            "G".to_string() // Mildly positive
        } else if valence_clamped > -0.5 {
            "A".to_string() // Mildly negative
        } else {
            "D".to_string() // Sad, negative
        }
    }

    /// Map emotional arousal to tempo
    fn map_arousal_to_tempo(&self, arousal: f32) -> f32 {
        let arousal_clamped = arousal.clamp(0.0, 1.0);
        // Map arousal to tempo range (60-180 BPM)
        60.0 + (arousal_clamped * 120.0)
    }

    /// Map emotional dominance to complexity
    fn map_dominance_to_complexity(&self, dominance: f32) -> f32 {
        let dominance_clamped = dominance.clamp(0.0, 1.0);
        dominance_clamped // Direct mapping for now
    }

    /// Generate audio data using the tunes crate
    fn generate_audio_data(&self, config: &MusicConfig) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        #[cfg(feature = "audio")]
        {
            // Create a composition based on emotional parameters
            let mut composition = Composition::new();
            
            // Set the scale based on valence (positive/negative emotion)
            let scale = match config.key.as_str() {
                "C" => Scale::major(),
                "A" => Scale::minor(),
                "G" => Scale::major(),
                "D" => Scale::minor(),
                _ => Scale::major(),
            };
            
            // Create a chord progression based on the scale
            let progression = Progression::from_scale(&scale, 4);
            
            // Generate melody based on emotional complexity
            let complexity = (config.complexity * 10.0) as usize + 1;
            let mut melody = Vec::new();
            
            // Create notes based on emotional input
            for i in 0..complexity {
                let note_index = i % scale.notes().count();
                if let Some(note) = scale.notes().nth(note_index) {
                    // Vary note duration based on tempo
                    let duration = match config.tempo {
                        t if t < 80 => 1.0,      // Slow tempo = longer notes
                        t if t < 120 => 0.5,     // Medium tempo = medium notes
                        _ => 0.25,                // Fast tempo = shorter notes
                    };
                    melody.push((note, duration));
                }
            }
            
            // Add the melody to the composition
            for (note, duration) in melody {
                composition.add_note(note, duration);
            }
            
            // Set rhythm based on tempo
            let rhythm = Rhythm::from_bpm(config.tempo as f64);
            composition.set_rhythm(rhythm);
            
            // Render the composition to audio data
            let audio_samples = composition.render(44100, 30.0)?; // 30 seconds at 44.1kHz
            
            // Convert samples to 16-bit PCM data
            let mut audio_data = Vec::with_capacity(audio_samples.len() * 2);
            for sample in audio_samples {
                let sample_i16 = (sample * 32767.0) as i16;
                audio_data.extend_from_slice(&sample_i16.to_le_bytes());
            }
            
            Ok(audio_data)
        }
        
        #[cfg(not(feature = "audio"))]
        {
            // Fallback to simple sine wave when audio feature is disabled
            let sample_rate = 44100;
            let duration_seconds = 30;
            let total_samples = sample_rate * duration_seconds;
            let frequency = self.key_to_frequency(&config.key);
            
            let mut audio_data = Vec::with_capacity(total_samples * 2);
            
            for i in 0..total_samples {
                let t = i as f32 / sample_rate as f32;
                let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin();
                let sample_i16 = (sample * 32767.0) as i16;
                audio_data.extend_from_slice(&sample_i16.to_le_bytes());
            }
            
            Ok(audio_data)
        }
    }

    /// Convert musical key to frequency (simplified)
    fn key_to_frequency(&self, key: &str) -> f32 {
        match key {
            "C" => 261.63, // Middle C
            "G" => 392.00, // G4
            "A" => 440.00, // A4
            "D" => 293.66, // D4
            _ => 440.00,   // Default to A4
        }
    }

    /// Create metadata for the generated music
    fn create_metadata(&self, emotional_input: &EmotionalInput) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();
        metadata.insert("emotional_category".to_string(), serde_json::json!(self.categorize_emotion(emotional_input)));
        metadata.insert("generation_method".to_string(), serde_json::json!("emotion_based"));
        metadata.insert("audio_format".to_string(), serde_json::json!("raw_16bit_pcm"));
        metadata.insert("sample_rate".to_string(), serde_json::json!(44100));
        metadata
    }

    /// Categorize emotional input
    fn categorize_emotion(&self, emotional_input: &EmotionalInput) -> String {
        let valence = emotional_input.valence;
        let arousal = emotional_input.arousal;

        match (valence > 0.0, arousal > 0.5) {
            (true, true) => "excited".to_string(),
            (true, false) => "happy".to_string(),
            (false, true) => "anxious".to_string(),
            (false, false) => "calm".to_string(),
        }
    }

    /// Generate music for a creative session
    pub fn generate_session_music(&self, session_emotions: &[EmotionalInput]) -> Result<Vec<GeneratedMusic>, Box<dyn std::error::Error>> {
        let mut generated_tracks = Vec::new();

        for emotion in session_emotions {
            let track = self.generate_music_from_emotion(emotion.clone())?;
            generated_tracks.push(track);
        }

        Ok(generated_tracks)
    }
}

impl Default for MusicEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Integration function for the main creative session
pub fn integrate_music_with_emotions(emotional_data: &crate::EmotionalData) -> Result<GeneratedMusic, Box<dyn std::error::Error>> {
    let engine = MusicEngine::new();
    
    let emotional_input = EmotionalInput {
        valence: emotional_data.valence,
        arousal: emotional_data.arousal,
        dominance: emotional_data.dominance,
    };

    engine.generate_music_from_emotion(emotional_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_music_engine_creation() {
        let engine = MusicEngine::new();
        assert_eq!(engine.config.tempo, 120.0);
    }

    #[test]
    fn test_emotion_mapping() {
        let engine = MusicEngine::new();
        
        let emotional_input = EmotionalInput {
            valence: 0.8,
            arousal: 0.9,
            dominance: 0.7,
        };

        let result = engine.generate_music_from_emotion(emotional_input);
        assert!(result.is_ok());
        
        let music = result.unwrap();
        assert_eq!(music.config.tempo > 150.0, true); // High arousal = high tempo
        assert_eq!(music.config.key, "C"); // High valence = C key
    }

    #[test]
    fn test_emotion_categorization() {
        let engine = MusicEngine::new();
        
        let excited = EmotionalInput { valence: 0.8, arousal: 0.9, dominance: 0.7 };
        let happy = EmotionalInput { valence: 0.7, arousal: 0.3, dominance: 0.6 };
        let anxious = EmotionalInput { valence: -0.6, arousal: 0.8, dominance: 0.4 };
        let calm = EmotionalInput { valence: 0.1, arousal: 0.2, dominance: 0.5 };

        assert_eq!(engine.categorize_emotion(&excited), "excited");
        assert_eq!(engine.categorize_emotion(&happy), "happy");
        assert_eq!(engine.categorize_emotion(&anxious), "anxious");
        assert_eq!(engine.categorize_emotion(&calm), "calm");
    }
}
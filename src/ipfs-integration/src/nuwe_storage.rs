//! NUWE Storage - IPFS integration for NUWE/Fractal Studio sessions
//! 
//! Handles storage of VJ performances, fractal sessions, and shader outputs

use crate::ipfs_client::IpfsClient;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// NUWE creative session for IPFS storage
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuweSession {
    pub session_id: String,
    pub session_type: SessionType,
    pub creator: String,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub fractal_params: Vec<FractalSnapshot>,
    pub shader_code: Option<String>,
    pub performance_metrics: PerformanceMetrics,
    pub emotional_data: Vec<EmotionalSnapshot>,
}

/// Type of NUWE session
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum SessionType {
    FractalStudio,
    WGSLStudio,
    VJPerformance,
    LiveCoding,
    ImmersiveVJ,
}

/// Snapshot of fractal parameters at a point in time
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FractalSnapshot {
    pub timestamp: u64,
    pub fractal_type: String,
    pub zoom: f64,
    pub center_x: f64,
    pub center_y: f64,
    pub iterations: u32,
    pub color_palette: Vec<u32>,
}

/// Performance metrics for session
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PerformanceMetrics {
    pub avg_fps: f32,
    pub peak_fps: f32,
    pub total_frames: u64,
    pub dropped_frames: u32,
    pub avg_render_time_ms: f32,
}

/// Emotional state snapshot
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmotionalSnapshot {
    pub timestamp: u64,
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
}

/// NUWE asset bundle for IPFS
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NuweAssetBundle {
    pub session: NuweSession,
    pub rendered_frames: Vec<FrameReference>,
    pub shader_outputs: Vec<ShaderOutput>,
    pub audio_track: Option<AudioReference>,
}

/// Reference to a rendered frame
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FrameReference {
    pub frame_number: u64,
    pub timestamp: u64,
    pub cid: String,
    pub format: String, // "png", "jpg", "webp"
    pub resolution: (u32, u32),
}

/// Shader compilation output
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ShaderOutput {
    pub shader_name: String,
    pub shader_type: String, // "wgsl", "glsl", "hlsl"
    pub compiled_code: String,
    pub compilation_time_ms: u32,
}

/// Audio track reference
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AudioReference {
    pub cid: String,
    pub duration_seconds: f32,
    pub format: String, // "wav", "mp3", "ogg"
    pub sample_rate: u32,
}

impl NuweSession {
    /// Create a new NUWE session
    pub fn new(session_id: String, session_type: SessionType, creator: String) -> Self {
        Self {
            session_id,
            session_type,
            creator,
            start_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            end_time: None,
            fractal_params: Vec::new(),
            shader_code: None,
            performance_metrics: PerformanceMetrics {
                avg_fps: 0.0,
                peak_fps: 0.0,
                total_frames: 0,
                dropped_frames: 0,
                avg_render_time_ms: 0.0,
            },
            emotional_data: Vec::new(),
        }
    }

    /// Add fractal snapshot to session
    pub fn add_fractal_snapshot(&mut self, snapshot: FractalSnapshot) {
        self.fractal_params.push(snapshot);
    }

    /// Add emotional data point
    pub fn add_emotional_data(&mut self, emotion: EmotionalSnapshot) {
        self.emotional_data.push(emotion);
    }

    /// Finalize session
    pub fn finalize(&mut self) {
        self.end_time = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
    }

    /// Get session duration in seconds
    pub fn duration_seconds(&self) -> Option<u64> {
        self.end_time.map(|end| end - self.start_time)
    }

    /// Store session to IPFS
    pub async fn store_to_ipfs(&self, client: &IpfsClient) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        client.add_json(&json).await
    }
}

impl NuweAssetBundle {
    /// Create a new asset bundle
    pub fn new(session: NuweSession) -> Self {
        Self {
            session,
            rendered_frames: Vec::new(),
            shader_outputs: Vec::new(),
            audio_track: None,
        }
    }

    /// Add rendered frame reference
    pub fn add_frame(&mut self, frame: FrameReference) {
        self.rendered_frames.push(frame);
    }

    /// Add shader output
    pub fn add_shader_output(&mut self, output: ShaderOutput) {
        self.shader_outputs.push(output);
    }

    /// Set audio track
    pub fn set_audio(&mut self, audio: AudioReference) {
        self.audio_track = Some(audio);
    }

    /// Store complete bundle to IPFS
    pub async fn store_to_ipfs(&self, client: &IpfsClient) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        client.add_json(&json).await
    }

    /// Get total storage size estimate in bytes
    pub fn estimated_size_bytes(&self) -> u64 {
        let mut size = 0u64;
        
        // Estimate session metadata
        size += 10_000; // ~10KB for session metadata
        
        // Estimate frame sizes (assuming average 1MB per frame)
        size += (self.rendered_frames.len() as u64) * 1_000_000;
        
        // Estimate shader outputs
        size += (self.shader_outputs.len() as u64) * 50_000; // ~50KB per shader
        
        // Estimate audio (if present)
        if let Some(ref audio) = self.audio_track {
            size += (audio.duration_seconds * audio.sample_rate as f32 * 4.0) as u64;
        }
        
        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nuwe_session_creation() {
        let session = NuweSession::new(
            "test_session".to_string(),
            SessionType::FractalStudio,
            "creator.near".to_string(),
        );
        
        assert_eq!(session.session_id, "test_session");
        assert!(session.fractal_params.is_empty());
    }

    #[test]
    fn test_add_fractal_snapshot() {
        let mut session = NuweSession::new(
            "test".to_string(),
            SessionType::FractalStudio,
            "creator".to_string(),
        );

        let snapshot = FractalSnapshot {
            timestamp: 12345,
            fractal_type: "mandelbrot".to_string(),
            zoom: 1.0,
            center_x: -0.5,
            center_y: 0.0,
            iterations: 100,
            color_palette: vec![0x000000, 0xFFFFFF],
        };

        session.add_fractal_snapshot(snapshot);
        assert_eq!(session.fractal_params.len(), 1);
    }

    #[test]
    fn test_asset_bundle_size_estimation() {
        let session = NuweSession::new(
            "test".to_string(),
            SessionType::VJPerformance,
            "creator".to_string(),
        );

        let mut bundle = NuweAssetBundle::new(session);
        
        // Add some frames
        for i in 0..10 {
            bundle.add_frame(FrameReference {
                frame_number: i,
                timestamp: 12345 + i * 33,
                cid: format!("Qm{}", i),
                format: "png".to_string(),
                resolution: (1920, 1080),
            });
        }

        let size = bundle.estimated_size_bytes();
        assert!(size > 0);
    }
}

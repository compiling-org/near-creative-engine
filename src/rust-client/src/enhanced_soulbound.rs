//! Enhanced soulbound token implementation with AI/ML integration and biometric authentication

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Enhanced soulbound token with AI-powered features and biometric authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnhancedSoulboundToken {
    pub token_id: String,
    pub owner_id: String,
    pub identity_data: IdentityData,
    pub creative_profile: CreativeProfile,
    pub reputation_score: f32,
    pub collaboration_history: Vec<CollaborationRecord>,
    pub ai_recommendations: Vec<String>,
    pub biometric_profile: BiometricProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityData {
    pub creative_profile: CreativeProfile,
    pub achievements: Vec<String>,
    pub verified: bool,
    pub reputation_score: f32,
    pub collaboration_history: Vec<CollaborationRecord>,
    pub biometric_profile: BiometricProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeProfile {
    pub primary_skill: String,
    pub experience_level: String,
    pub creative_style: String,
    pub skill_tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationRecord {
    pub partner_id: String,
    pub project_id: String,
    pub success_rating: f32,
    pub timestamp: u64,
    pub skills_contributed: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiometricProfile {
    pub eeg_fingerprint: Option<Vec<f32>>,
    pub emotional_signature: Option<Vec<f32>>,
    pub fingerprint_hash: Vec<u8>,
}

impl EnhancedSoulboundToken {
    pub fn new_full(
        token_id: String,
        owner_id: String,
        identity_data: IdentityData,
        creative_profile: CreativeProfile,
        reputation_score: f32,
        collaboration_history: Vec<CollaborationRecord>,
        ai_recommendations: Vec<String>,
        biometric_profile: BiometricProfile,
    ) -> Self {
        Self {
            token_id,
            owner_id,
            identity_data,
            creative_profile,
            reputation_score,
            collaboration_history,
            ai_recommendations,
            biometric_profile,
        }
    }

    pub fn new(token_id: String, owner_id: String, creative_skills: Vec<String>, experience_level: String) -> Self {
        let primary_skill = creative_skills.first().unwrap_or(&"general".to_string()).clone();
        let creative_profile = CreativeProfile {
            primary_skill: primary_skill.clone(),
            experience_level,
            creative_style: "modern".to_string(),
            skill_tags: creative_skills,
        };
        let identity_data = IdentityData {
            creative_profile: creative_profile.clone(),
            achievements: vec![],
            verified: false,
            reputation_score: 0.0,
            collaboration_history: vec![],
            biometric_profile: BiometricProfile { eeg_fingerprint: None, emotional_signature: None, fingerprint_hash: vec![] },
        };
        Self {
            token_id,
            owner_id,
            identity_data,
            creative_profile,
            reputation_score: 0.0,
            collaboration_history: vec![],
            ai_recommendations: vec![],
            biometric_profile: BiometricProfile { eeg_fingerprint: None, emotional_signature: None, fingerprint_hash: vec![] },
        }
    }

    pub fn update_reputation(&mut self, delta: f32) {
        self.reputation_score = (self.reputation_score + delta).clamp(0.0, 1.0);
    }

    pub fn record_collaboration(&mut self, record: CollaborationRecord) {
        self.collaboration_history.push(record);
        let avg = if self.collaboration_history.is_empty() {
            0.0
        } else {
            self.collaboration_history.iter().map(|r| r.success_rating).sum::<f32>()
                / (self.collaboration_history.len() as f32)
        };
        self.reputation_score = (self.reputation_score * 0.8) + (avg * 0.2);
    }

    pub fn get_skill_recommendations(&self) -> Vec<String> {
        self.ai_recommendations.clone()
    }
}
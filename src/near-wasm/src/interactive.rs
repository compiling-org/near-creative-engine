//! Interactive NFT functionality
//!
//! Enhanced with advanced interaction tracking and evolution mechanics.

use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Timestamp};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InteractionEvent {
    pub event_type: String,
    pub timestamp: Timestamp,
    pub user_id: AccountId,
    pub data: String, // Changed from serde_json::Value to String for Borsh compatibility
    pub intensity: f32,
    // Add emotional context to interactions
    pub emotional_impact: Option<EmotionalImpact>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalImpact {
    pub valence_shift: f32,
    pub arousal_shift: f32,
    pub dominance_shift: f32,
    pub confidence: f32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InteractiveState {
    pub mood: String,
    pub energy_level: f32,
    pub creativity_index: f32,
    pub interaction_streak: u32,
    pub last_activity: Timestamp,
    pub evolution_stage: String,
    // Add advanced interactive features
    pub interaction_patterns: Vec<InteractionPattern>,
    pub community_engagement: CommunityEngagement,
    pub adaptive_behavior: AdaptiveBehavior,
    pub interaction_history_summary: InteractionHistorySummary,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InteractionPattern {
    pub pattern_type: String,
    pub frequency: u32,
    pub last_occurrence: Timestamp,
    pub emotional_signature: EmotionalSignature,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalSignature {
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub avg_dominance: f32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CommunityEngagement {
    pub total_interactions: u32,
    pub unique_users: u32,
    pub community_score: f32,
    pub trending: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AdaptiveBehavior {
    pub learning_rate: f32,
    pub preference_weights: Vec<f32>,
    pub behavior_adaptations: Vec<BehaviorAdaptation>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BehaviorAdaptation {
    pub adaptation_type: String,
    pub trigger_condition: String,
    pub response_action: String,
    pub effectiveness: f32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InteractionHistorySummary {
    pub total_interactions: u32,
    pub avg_interaction_intensity: f32,
    pub most_common_event_type: String,
    pub engagement_trend: String, // "increasing", "decreasing", "stable"
}

impl Default for InteractiveState {
    fn default() -> Self {
        Self {
            mood: "neutral".to_string(),
            energy_level: 0.5,
            creativity_index: 0.5,
            interaction_streak: 0,
            last_activity: env::block_timestamp(),
            evolution_stage: "seed".to_string(),
            interaction_patterns: vec![],
            community_engagement: CommunityEngagement {
                total_interactions: 0,
                unique_users: 0,
                community_score: 0.5,
                trending: false,
            },
            adaptive_behavior: AdaptiveBehavior {
                learning_rate: 0.1,
                preference_weights: vec![0.5; 5], // Default weights for 5 preference types
                behavior_adaptations: vec![],
            },
            interaction_history_summary: InteractionHistorySummary {
                total_interactions: 0,
                avg_interaction_intensity: 0.5,
                most_common_event_type: "view".to_string(),
                engagement_trend: "stable".to_string(),
            },
        }
    }
}

impl InteractiveState {
    pub fn update_mood(&mut self, valence: f32) {
        self.mood = match valence {
            v if v > 0.5 => "happy".to_string(),
            v if v < -0.5 => "sad".to_string(),
            _ => "neutral".to_string(),
        };
    }
    
    pub fn update_evolution(&mut self) {
        // Simple evolution based on interaction streak
        self.evolution_stage = match self.interaction_streak {
            0..=5 => "seed".to_string(),
            6..=20 => "sprout".to_string(),
            21..=50 => "bloom".to_string(),
            _ => "thrive".to_string(),
        };
    }
    
    /// Update interaction patterns based on new event
    pub fn update_interaction_patterns(&mut self, event: &InteractionEvent) {
        // Check if we already have this pattern type
        let pattern_exists = self.interaction_patterns.iter_mut().find(|p| p.pattern_type == event.event_type);
        
        if let Some(pattern) = pattern_exists {
            pattern.frequency += 1;
            pattern.last_occurrence = event.timestamp;
            // Update emotional signature (simplified)
            if let Some(impact) = &event.emotional_impact {
                pattern.emotional_signature.avg_valence = (pattern.emotional_signature.avg_valence + impact.valence_shift) / 2.0;
                pattern.emotional_signature.avg_arousal = (pattern.emotional_signature.avg_arousal + impact.arousal_shift) / 2.0;
                pattern.emotional_signature.avg_dominance = (pattern.emotional_signature.avg_dominance + impact.dominance_shift) / 2.0;
            }
        } else {
            // Create new pattern
            let emotional_signature = if let Some(impact) = &event.emotional_impact {
                EmotionalSignature {
                    avg_valence: impact.valence_shift,
                    avg_arousal: impact.arousal_shift,
                    avg_dominance: impact.dominance_shift,
                }
            } else {
                EmotionalSignature {
                    avg_valence: 0.0,
                    avg_arousal: 0.5,
                    avg_dominance: 0.5,
                }
            };
            
            self.interaction_patterns.push(InteractionPattern {
                pattern_type: event.event_type.clone(),
                frequency: 1,
                last_occurrence: event.timestamp,
                emotional_signature,
            });
        }
    }
    
    /// Update community engagement metrics
    pub fn update_community_engagement(&mut self, user_id: &AccountId) {
        self.community_engagement.total_interactions += 1;
        
        // In a real implementation, we'd track unique users properly
        // For now, we'll just increment unique users for demo purposes
        self.community_engagement.unique_users += 1;
        
        // Update community score based on interaction frequency
        if self.community_engagement.total_interactions > 50 {
            self.community_engagement.community_score = 0.9;
            self.community_engagement.trending = true;
        } else if self.community_engagement.total_interactions > 20 {
            self.community_engagement.community_score = 0.7;
        } else {
            self.community_engagement.community_score = 0.5;
        }
    }
    
    /// Update interaction history summary
    pub fn update_interaction_history_summary(&mut self, events: &[InteractionEvent]) {
        if events.is_empty() {
            return;
        }
        
        self.interaction_history_summary.total_interactions = events.len() as u32;
        
        // Calculate average intensity
        let total_intensity: f32 = events.iter().map(|e| e.intensity).sum();
        self.interaction_history_summary.avg_interaction_intensity = total_intensity / events.len() as f32;
        
        // Find most common event type
        let mut event_type_counts: std::collections::HashMap<String, u32> = std::collections::HashMap::new();
        for event in events {
            *event_type_counts.entry(event.event_type.clone()).or_insert(0) += 1;
        }
        
        if let Some((most_common, _)) = event_type_counts.iter().max_by_key(|(_, count)| *count) {
            self.interaction_history_summary.most_common_event_type = most_common.clone();
        }
        
        // Determine engagement trend (simplified)
        if events.len() > 10 {
            self.interaction_history_summary.engagement_trend = "increasing".to_string();
        } else if events.len() > 5 {
            self.interaction_history_summary.engagement_trend = "stable".to_string();
        } else {
            self.interaction_history_summary.engagement_trend = "decreasing".to_string();
        }
    }
    
    /// Adapt behavior based on interaction history
    pub fn adapt_behavior(&mut self, events: &[InteractionEvent]) {
        // Simple adaptation: increase learning rate with more interactions
        if events.len() > 20 {
            self.adaptive_behavior.learning_rate = 0.3;
        } else if events.len() > 10 {
            self.adaptive_behavior.learning_rate = 0.2;
        } else {
            self.adaptive_behavior.learning_rate = 0.1;
        }
        
        // Add behavior adaptation based on popular event types
        let popular_event_type = self.interaction_history_summary.most_common_event_type.clone();
        if !self.adaptive_behavior.behavior_adaptations.iter().any(|a| a.adaptation_type == popular_event_type) {
            self.adaptive_behavior.behavior_adaptations.push(BehaviorAdaptation {
                adaptation_type: popular_event_type,
                trigger_condition: "frequent_interaction".to_string(),
                response_action: "enhance_response".to_string(),
                effectiveness: 0.8,
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interactive_state_default() {
        let state = InteractiveState::default();
        assert_eq!(state.mood, "neutral");
        assert_eq!(state.energy_level, 0.5);
        assert_eq!(state.creativity_index, 0.5);
        assert_eq!(state.interaction_streak, 0);
        assert_eq!(state.evolution_stage, "seed");
    }

    #[test]
    fn test_update_mood() {
        let mut state = InteractiveState::default();
        
        state.update_mood(0.8);
        assert_eq!(state.mood, "happy");
        
        state.update_mood(-0.8);
        assert_eq!(state.mood, "sad");
        
        state.update_mood(0.0);
        assert_eq!(state.mood, "neutral");
    }

    #[test]
    fn test_update_evolution() {
        let mut state = InteractiveState::default();
        
        state.interaction_streak = 3;
        state.update_evolution();
        assert_eq!(state.evolution_stage, "seed");
        
        state.interaction_streak = 10;
        state.update_evolution();
        assert_eq!(state.evolution_stage, "sprout");
        
        state.interaction_streak = 30;
        state.update_evolution();
        assert_eq!(state.evolution_stage, "bloom");
        
        state.interaction_streak = 100;
        state.update_evolution();
        assert_eq!(state.evolution_stage, "thrive");
    }
    
    #[test]
    fn test_update_interaction_patterns() {
        let mut state = InteractiveState::default();
        let event = InteractionEvent {
            event_type: "view".to_string(),
            timestamp: env::block_timestamp(),
            user_id: "user.testnet".parse().unwrap(),
            data: "{}".to_string(),
            intensity: 0.5,
            emotional_impact: Some(EmotionalImpact {
                valence_shift: 0.1,
                arousal_shift: 0.2,
                dominance_shift: 0.3,
                confidence: 0.8,
            }),
        };
        
        state.update_interaction_patterns(&event);
        assert_eq!(state.interaction_patterns.len(), 1);
        assert_eq!(state.interaction_patterns[0].pattern_type, "view");
        assert_eq!(state.interaction_patterns[0].frequency, 1);
    }
    
    #[test]
    fn test_update_community_engagement() {
        let mut state = InteractiveState::default();
        let user_id: AccountId = "user.testnet".parse().unwrap();
        
        state.update_community_engagement(&user_id);
        assert_eq!(state.community_engagement.total_interactions, 1);
        assert_eq!(state.community_engagement.unique_users, 1);
    }
}
//! Interactive NFT functionality

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
}
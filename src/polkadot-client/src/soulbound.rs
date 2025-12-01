//! Soulbound Token Client
//! 
//! Non-transferable tokens for creator identity and reputation across chains

use serde::{Deserialize, Serialize};
use subxt::utils::AccountId32;
use crate::EmotionalMetadata;

/// Soulbound token structure
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SoulboundToken {
    pub owner: AccountId32,
    pub token_id: u64,
    pub token_type: TokenType,
    pub metadata: Vec<u8>,
    pub issued_at: u64,
    pub is_revoked: bool,
}

/// Type of soulbound token
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum TokenType {
    CreatorIdentity,
    ReputationBadge,
    Achievement,
    Membership,
    Certification,
}

/// Reputation data for creators
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReputationData {
    pub score: u32,
    pub total_interactions: u32,
    pub badges: u32,
}

/// Advanced reputation system with emotional computing integration
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AdvancedReputation {
    pub score: f32,
    pub total_interactions: u32,
    pub badges: Vec<Badge>,
    pub emotional_consistency: f32,
    pub creative_diversity: f32,
    pub collaboration_score: f32,
    // Enhanced fields
    pub emotional_complexity: f32,
    pub creativity_index: f32,
    pub engagement_score: f32,
    pub reputation_trajectory: Vec<ReputationPoint>,
}

/// Point in reputation trajectory
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReputationPoint {
    pub score: f32,
    pub timestamp: u64,
}

/// Badge system for creator achievements
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Badge {
    Pioneer,
    Master,
    Collaborator,
    Innovator,
    EmotionalArtist,
    TechnicalExpert,
    CommunityLeader,
    TrendSetter,
}

/// Emotional reputation metrics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct EmotionalReputation {
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub emotional_range: f32,
    pub consistency_score: f32,
    pub emotional_growth: f32,
    // Enhanced fields
    pub emotional_volatility: f32,
    pub emotional_maturity: f32,
    pub empathy_index: f32,
}

/// Advanced soulbound token with enhanced metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdvancedSoulboundToken {
    pub owner: AccountId32,
    pub token_id: u64,
    pub token_type: TokenType,
    pub metadata: Vec<u8>,
    pub issued_at: u64,
    pub is_revoked: bool,
    // Enhanced fields
    pub reputation: AdvancedReputation,
    pub emotional_metrics: EmotionalReputation,
    pub cross_chain_provenance: Vec<String>,
    pub creative_traits: Vec<String>,
    // New enhanced fields
    pub emotional_journey: Vec<EmotionalMetadata>,
    pub interaction_patterns: Vec<InteractionPattern>,
    pub community_engagement: CommunityEngagement,
    pub adaptive_personality: AdaptivePersonality,
}

/// Interaction pattern for behavioral analysis
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InteractionPattern {
    pub pattern_type: String,
    pub frequency: u32,
    pub emotional_response: f32,
}

/// Community engagement metrics
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CommunityEngagement {
    pub total_interactions: u32,
    pub positive_feedback: u32,
    pub community_building: f32,
    pub influence_radius: u32,
}

/// Adaptive personality traits
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AdaptivePersonality {
    pub adaptability: f32,
    pub openness: f32,
    pub conscientiousness: f32,
    pub emotional_stability: f32,
    pub learning_rate: f32,
}

/// Soulbound token client
pub struct SoulboundTokenClient {
    // Client implementation would go here
}

impl SoulboundTokenClient {
    /// Create a new soulbound token
    pub fn new_soulbound_token(
        owner: AccountId32,
        token_id: u64,
        token_type: TokenType,
        metadata: Vec<u8>,
    ) -> SoulboundToken {
        SoulboundToken {
            owner,
            token_id,
            token_type,
            metadata,
            issued_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            is_revoked: false,
        }
    }

    /// Update reputation score
    pub fn update_reputation(
        reputation: &mut ReputationData,
        score_delta: i32,
    ) -> Result<(), &'static str> {
        let new_score = if score_delta >= 0 {
            reputation.score.saturating_add(score_delta as u32)
        } else {
            reputation.score.saturating_sub((-score_delta) as u32)
        };
        
        if new_score > 10000 {
            return Err("Reputation score too high");
        }
        
        reputation.score = new_score;
        reputation.total_interactions += 1;
        Ok(())
    }
    
    /// Create an advanced soulbound token with enhanced metadata
    pub fn new_advanced_soulbound_token(
        owner: AccountId32,
        token_id: u64,
        token_type: TokenType,
        metadata: Vec<u8>,
        creative_traits: Vec<String>,
    ) -> AdvancedSoulboundToken {
        AdvancedSoulboundToken {
            owner,
            token_id,
            token_type,
            metadata,
            issued_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            is_revoked: false,
            reputation: AdvancedReputation::default(),
            emotional_metrics: EmotionalReputation::default(),
            cross_chain_provenance: vec![],
            creative_traits,
            emotional_journey: vec![],
            interaction_patterns: vec![],
            community_engagement: CommunityEngagement::default(),
            adaptive_personality: AdaptivePersonality::default(),
        }
    }
    
    /// Update advanced reputation based on interaction quality and emotional consistency
    pub fn update_advanced_reputation(
        reputation: &mut AdvancedReputation,
        score_delta: f32,
        emotional_consistency: f32,
    ) -> Result<(), &'static str> {
        let new_score = (reputation.score + score_delta).max(0.0).min(100.0);
        reputation.score = new_score;
        reputation.total_interactions += 1;
        
        // Update emotional consistency
        reputation.emotional_consistency = (reputation.emotional_consistency * (reputation.total_interactions - 1) as f32 
            + emotional_consistency) / reputation.total_interactions as f32;
        
        // Add to reputation trajectory
        reputation.reputation_trajectory.push(ReputationPoint {
            score: new_score,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        
        // Award badges based on achievements
        if reputation.total_interactions >= 100 && !reputation.badges.contains(&Badge::Pioneer) {
            reputation.badges.push(Badge::Pioneer);
        }
        
        if reputation.score > 90.0 && !reputation.badges.contains(&Badge::Master) {
            reputation.badges.push(Badge::Master);
        }
        
        // Update complexity and creativity metrics
        reputation.emotional_complexity = Self::calculate_reputation_complexity(&reputation.reputation_trajectory);
        reputation.creativity_index = Self::calculate_creativity_index(&reputation.reputation_trajectory);
        reputation.engagement_score = Self::calculate_engagement_score(reputation.total_interactions, reputation.emotional_complexity);
        
        Ok(())
    }
    
    /// Calculate reputation complexity based on trajectory
    fn calculate_reputation_complexity(trajectory: &[ReputationPoint]) -> f32 {
        if trajectory.len() < 2 {
            return 0.0;
        }
        
        let mut total_variance = 0.0;
        for i in 1..trajectory.len() {
            let diff = trajectory[i].score - trajectory[i-1].score;
            total_variance += diff.abs();
        }
        
        // Normalize by number of points
        (total_variance / trajectory.len() as f32).clamp(0.0, 1.0)
    }
    
    /// Calculate creativity index based on reputation growth patterns
    fn calculate_creativity_index(trajectory: &[ReputationPoint]) -> f32 {
        if trajectory.len() < 3 {
            return 0.0;
        }
        
        // Look for non-linear growth patterns as indicators of creativity
        let mut changes = Vec::new();
        for i in 1..trajectory.len() {
            changes.push(trajectory[i].score - trajectory[i-1].score);
        }
        
        // Calculate variance in changes (higher variance suggests creative approaches)
        let avg_change: f32 = changes.iter().sum::<f32>() / changes.len() as f32;
        let variance: f32 = changes.iter().map(|&c| (c - avg_change).powi(2)).sum::<f32>() / changes.len() as f32;
        
        // Normalize to 0-1 range
        (variance.sqrt() / 10.0).clamp(0.0, 1.0)
    }
    
    /// Calculate engagement score
    fn calculate_engagement_score(interactions: u32, complexity: f32) -> f32 {
        let interaction_component = (interactions as f32 / 1000.0).min(1.0);
        0.7 * interaction_component + 0.3 * complexity
    }
    
    /// Calculate emotional metrics from interaction data
    pub fn calculate_emotional_metrics(
        emotional_data: &[EmotionalMetadata],
    ) -> EmotionalReputation {
        if emotional_data.is_empty() {
            return EmotionalReputation::default();
        }
        
        let sum_valence: f32 = emotional_data.iter().map(|e| e.valence).sum();
        let sum_arousal: f32 = emotional_data.iter().map(|e| e.arousal).sum();
        let sum_confidence: f32 = emotional_data.iter().map(|e| e.confidence).sum();
        
        let count = emotional_data.len() as f32;
        let avg_valence = sum_valence / count;
        let avg_arousal = sum_arousal / count;
        let avg_confidence = sum_confidence / count;
        
        // Calculate emotional range (max distance from average)
        let max_distance = emotional_data.iter().map(|e| {
            let dv = e.valence - avg_valence;
            let da = e.arousal - avg_arousal;
            (dv * dv + da * da).sqrt()
        }).fold(0.0f32, f32::max);
        
        // Calculate consistency (inverse of variance)
        let variance_sum: f32 = emotional_data.iter().map(|e| {
            let dv = e.valence - avg_valence;
            let da = e.arousal - avg_arousal;
            dv * dv + da * da
        }).sum();
        
        let consistency = 1.0 - (variance_sum / count).sqrt() / 1.414; // Normalize by max possible distance
        
        // Calculate volatility (standard deviation)
        let volatility = (variance_sum / count).sqrt();
        
        // Calculate emotional maturity (based on confidence growth)
        let maturity = if emotional_data.len() > 1 {
            let first_confidence = emotional_data[0].confidence;
            let last_confidence = emotional_data[emotional_data.len() - 1].confidence;
            ((last_confidence - first_confidence) / emotional_data.len() as f32 + 1.0) / 2.0
        } else {
            avg_confidence
        }.clamp(0.0, 1.0);
        
        EmotionalReputation {
            avg_valence,
            avg_arousal,
            emotional_range: max_distance,
            consistency_score: consistency.max(0.0).min(1.0),
            emotional_growth: 0.0, // Would be calculated based on historical data
            emotional_volatility: volatility.clamp(0.0, 1.0),
            emotional_maturity: maturity,
            empathy_index: 0.5, // Placeholder
        }
    }
    
    /// Add cross-chain provenance to token
    pub fn add_cross_chain_provenance(
        token: &mut AdvancedSoulboundToken,
        chain: String,
    ) {
        if !token.cross_chain_provenance.contains(&chain) {
            token.cross_chain_provenance.push(chain);
        }
    }
    
    /// Add emotional data to token's journey
    pub fn add_emotional_data(
        token: &mut AdvancedSoulboundToken,
        emotional_data: EmotionalMetadata,
    ) {
        token.emotional_journey.push(emotional_data);
        token.emotional_metrics = Self::calculate_emotional_metrics(&token.emotional_journey);
    }
    
    /// Update community engagement metrics
    pub fn update_community_engagement(
        token: &mut AdvancedSoulboundToken,
        interaction_type: &str,
        is_positive: bool,
    ) {
        token.community_engagement.total_interactions += 1;
        
        if is_positive {
            token.community_engagement.positive_feedback += 1;
        }
        
        // Update community building score based on interaction types
        match interaction_type {
            "collaboration" => token.community_engagement.community_building += 0.1,
            "mentorship" => token.community_engagement.community_building += 0.15,
            "leadership" => token.community_engagement.community_building += 0.2,
            _ => token.community_engagement.community_building += 0.05,
        }
        
        token.community_engagement.community_building = token.community_engagement.community_building.min(1.0);
    }
    
    /// Adapt personality based on interactions
    pub fn adapt_personality(
        token: &mut AdvancedSoulboundToken,
        interaction_emotional_impact: f32,
    ) {
        let learning_rate = token.adaptive_personality.learning_rate;
        
        // Adjust adaptability based on emotional impact
        token.adaptive_personality.adaptability = 
            (token.adaptive_personality.adaptability + interaction_emotional_impact * learning_rate).clamp(0.0, 1.0);
            
        // Adjust emotional stability based on consistency of interactions
        token.adaptive_personality.emotional_stability = 
            (token.adaptive_personality.emotional_stability + (1.0 - interaction_emotional_impact.abs()) * learning_rate).clamp(0.0, 1.0);
    }
}
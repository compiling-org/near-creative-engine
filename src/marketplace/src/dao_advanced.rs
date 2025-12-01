//! Advanced DAO Governance with Emotional Weighting
//! 
//! Revolutionary DAO system that incorporates emotional state data
//! from EEG/BMI sensors into voting and proposal mechanisms

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance, Timestamp};
use near_sdk::collections::{UnorderedMap, UnorderedSet, LookupMap};

/// Advanced DAO with emotional governance
#[derive(BorshDeserialize, BorshSerialize)]
pub struct EmotionalDAO {
    /// DAO members with their emotional profiles
    pub members: UnorderedMap<AccountId, MemberProfile>,
    
    /// Active proposals
    pub proposals: UnorderedMap<u64, EmotionalProposal>,
    
    /// Voting history with emotional states
    pub vote_history: LookupMap<(u64, AccountId), EmotionalVote>,
    
    /// DAO configuration
    pub config: DAOConfig,
    
    /// Next proposal ID
    pub next_proposal_id: u64,
    
    /// Reputation scores
    pub reputation: LookupMap<AccountId, ReputationScore>,
}

/// Member profile with emotional baseline
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MemberProfile {
    pub account: AccountId,
    pub joined_at: Timestamp,
    pub voting_power: u64,
    pub emotional_baseline: EmotionalBaseline,
    pub participation_count: u32,
    pub soulbound_token_id: Option<String>,
}

/// Emotional baseline for a member
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalBaseline {
    pub avg_valence: f32,
    pub avg_arousal: f32,
    pub avg_dominance: f32,
    pub consistency_score: f32, // How consistent their emotional states are
}

/// Proposal with emotional consensus requirements
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalProposal {
    pub proposal_id: u64,
    pub proposer: AccountId,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    
    /// Traditional votes
    pub votes_for: u64,
    pub votes_against: u64,
    
    /// Emotional consensus metrics
    pub emotional_consensus: EmotionalConsensus,
    
    /// Required emotional alignment for passage
    pub required_alignment: f32,
    
    pub created_at: Timestamp,
    pub end_time: Timestamp,
    pub status: ProposalStatus,
    
    /// Attached deposit for execution
    pub execution_deposit: Balance,
}

/// Emotional consensus tracking
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalConsensus {
    /// Average emotional state of voters
    pub avg_voter_valence: f32,
    pub avg_voter_arousal: f32,
    pub avg_voter_dominance: f32,
    
    /// Emotional variance (lower = more consensus)
    pub emotional_variance: f32,
    
    /// Percentage of voters with aligned emotions
    pub alignment_percentage: f32,
    
    /// Number of votes cast
    pub total_votes: u32,
}

/// Vote with emotional state data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalVote {
    pub voter: AccountId,
    pub proposal_id: u64,
    pub vote: VoteChoice,
    pub voting_power: u64,
    
    /// Emotional state at time of voting
    pub emotional_state: EmotionalState,
    
    /// EEG/BMI sensor data (optional)
    pub sensor_data: Option<SensorData>,
    
    pub timestamp: Timestamp,
}

/// Emotional state captured during voting
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalState {
    pub valence: f32,
    pub arousal: f32,
    pub dominance: f32,
    pub confidence: f32,
    pub source: EmotionSource,
}

/// Source of emotional data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum EmotionSource {
    EEG,
    BMI,
    FacialRecognition,
    VoiceAnalysis,
    HeartRate,
    GSR, // Galvanic Skin Response
    Combined,
    SelfReported,
}

/// EEG/BMI sensor data
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SensorData {
    pub sensor_type: String,
    pub raw_values: Vec<f32>,
    pub processed_features: Vec<f32>,
    pub data_quality: f32,
    pub timestamp: Timestamp,
    pub data_hash: String, // IPFS CID of full sensor recording
}

/// Vote choice
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteChoice {
    For,
    Against,
    Abstain,
}

/// Proposal types
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalType {
    ParameterChange { parameter: String, new_value: String },
    FundingRequest { recipient: AccountId, amount: Balance },
    MembershipChange { account: AccountId, action: MemberAction },
    PolicyUpdate { policy_name: String, new_policy: String },
    EmergencyAction { action: String },
}

/// Member action
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum MemberAction {
    Add,
    Remove,
    UpdateVotingPower(u64),
}

/// Proposal status
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

/// DAO configuration
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DAOConfig {
    pub quorum_percentage: u32,
    pub voting_period_days: u64,
    pub execution_delay_days: u64,
    pub min_emotional_alignment: f32,
    pub use_emotional_weighting: bool,
    pub require_sensor_data: bool,
}

/// Reputation score
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, Default)]
#[serde(crate = "near_sdk::serde")]
pub struct ReputationScore {
    pub score: u32,
    pub participation_bonus: u32,
    pub consistency_bonus: u32,
    pub contribution_count: u32,
}

impl EmotionalDAO {
    /// Create a new emotional DAO
    pub fn new(config: DAOConfig) -> Self {
        Self {
            members: UnorderedMap::new(b"m"),
            proposals: UnorderedMap::new(b"p"),
            vote_history: LookupMap::new(b"v"),
            config,
            next_proposal_id: 1,
            reputation: LookupMap::new(b"r"),
        }
    }

    /// Add a member to the DAO
    pub fn add_member(&mut self, account: AccountId, voting_power: u64) {
        let profile = MemberProfile {
            account: account.clone(),
            joined_at: env::block_timestamp(),
            voting_power,
            emotional_baseline: EmotionalBaseline::default(),
            participation_count: 0,
            soulbound_token_id: None,
        };
        
        self.members.insert(&account, &profile);
        
        let reputation = ReputationScore::default();
        self.reputation.insert(&account, &reputation);
    }

    /// Create a proposal
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        proposal_type: ProposalType,
        required_alignment: f32,
        execution_deposit: Balance,
    ) -> u64 {
        let proposal_id = self.next_proposal_id;
        self.next_proposal_id += 1;

        let voting_period_ns = self.config.voting_period_days * 24 * 60 * 60 * 1_000_000_000;
        
        let proposal = EmotionalProposal {
            proposal_id,
            proposer: env::predecessor_account_id(),
            title,
            description,
            proposal_type,
            votes_for: 0,
            votes_against: 0,
            emotional_consensus: EmotionalConsensus::default(),
            required_alignment,
            created_at: env::block_timestamp(),
            end_time: env::block_timestamp() + voting_period_ns,
            status: ProposalStatus::Active,
            execution_deposit,
        };

        self.proposals.insert(&proposal_id, &proposal);
        proposal_id
    }

    /// Cast a vote with emotional state
    pub fn vote(
        &mut self,
        proposal_id: u64,
        vote_choice: VoteChoice,
        emotional_state: EmotionalState,
        sensor_data: Option<SensorData>,
    ) {
        let voter = env::predecessor_account_id();
        
        // Verify member exists
        let member = self.members.get(&voter)
            .expect("Not a DAO member");

        // Verify proposal exists and is active
        let mut proposal = self.proposals.get(&proposal_id)
            .expect("Proposal not found");
        
        assert_eq!(proposal.status, ProposalStatus::Active, "Proposal not active");
        assert!(env::block_timestamp() < proposal.end_time, "Voting period ended");

        // If sensor data required, validate it
        if self.config.require_sensor_data {
            assert!(sensor_data.is_some(), "Sensor data required");
            if let Some(ref data) = sensor_data {
                assert!(data.data_quality > 0.7, "Sensor data quality too low");
            }
        }

        // Record the vote
        let vote = EmotionalVote {
            voter: voter.clone(),
            proposal_id,
            vote: vote_choice.clone(),
            voting_power: member.voting_power,
            emotional_state: emotional_state.clone(),
            sensor_data,
            timestamp: env::block_timestamp(),
        };

        self.vote_history.insert(&(proposal_id, voter.clone()), &vote);

        // Update proposal vote counts
        match vote_choice {
            VoteChoice::For => proposal.votes_for += member.voting_power,
            VoteChoice::Against => proposal.votes_against += member.voting_power,
            VoteChoice::Abstain => {},
        }

        // Update emotional consensus
        self.update_emotional_consensus(&mut proposal, &emotional_state);

        self.proposals.insert(&proposal_id, &proposal);

        // Update member participation
        let mut member_profile = member;
        member_profile.participation_count += 1;
        self.members.insert(&voter, &member_profile);
    }

    /// Update emotional consensus metrics
    fn update_emotional_consensus(
        &self,
        proposal: &mut EmotionalProposal,
        new_state: &EmotionalState,
    ) {
        let consensus = &mut proposal.emotional_consensus;
        let n = consensus.total_votes as f32;
        
        // Update rolling averages
        consensus.avg_voter_valence = 
            (consensus.avg_voter_valence * n + new_state.valence) / (n + 1.0);
        consensus.avg_voter_arousal = 
            (consensus.avg_voter_arousal * n + new_state.arousal) / (n + 1.0);
        consensus.avg_voter_dominance = 
            (consensus.avg_voter_dominance * n + new_state.dominance) / (n + 1.0);
        
        consensus.total_votes += 1;

        // Calculate emotional variance (simplified)
        let deviation = ((new_state.valence - consensus.avg_voter_valence).powi(2)
            + (new_state.arousal - consensus.avg_voter_arousal).powi(2)
            + (new_state.dominance - consensus.avg_voter_dominance).powi(2))
            .sqrt();
        
        consensus.emotional_variance = 
            (consensus.emotional_variance * n + deviation) / (n + 1.0);
    }

    /// Finalize a proposal
    pub fn finalize_proposal(&mut self, proposal_id: u64) -> ProposalStatus {
        let mut proposal = self.proposals.get(&proposal_id)
            .expect("Proposal not found");

        assert_eq!(proposal.status, ProposalStatus::Active, "Proposal not active");
        assert!(env::block_timestamp() >= proposal.end_time, "Voting period not ended");

        // Calculate quorum
        let total_voting_power: u64 = self.members.iter()
            .map(|(_, member)| member.voting_power)
            .sum();
        
        let total_votes = proposal.votes_for + proposal.votes_against;
        let participation_percentage = (total_votes * 100) / total_voting_power;

        // Check traditional quorum
        let quorum_met = participation_percentage >= self.config.quorum_percentage as u64;
        
        // Check majority
        let majority_for = proposal.votes_for > proposal.votes_against;

        // Check emotional alignment if required
        let emotional_alignment_met = if self.config.use_emotional_weighting {
            proposal.emotional_consensus.emotional_variance < proposal.required_alignment
        } else {
            true
        };

        // Determine outcome
        proposal.status = if quorum_met && majority_for && emotional_alignment_met {
            ProposalStatus::Passed
        } else {
            ProposalStatus::Rejected
        };

        self.proposals.insert(&proposal_id, &proposal);
        proposal.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emotional_dao_creation() {
        let config = DAOConfig {
            quorum_percentage: 51,
            voting_period_days: 7,
            execution_delay_days: 2,
            min_emotional_alignment: 0.5,
            use_emotional_weighting: true,
            require_sensor_data: false,
        };

        let dao = EmotionalDAO::new(config);
        assert_eq!(dao.next_proposal_id, 1);
    }
}

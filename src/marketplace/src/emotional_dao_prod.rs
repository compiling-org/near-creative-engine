// Production Emotional DAO Contract
// Based on Sputnik DAO V2 architecture with emotional voting
// Reference: https://github.com/near-daos/sputnik-dao-contract

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, Timestamp,
};
use std::collections::HashMap;

/// Proposal status
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    InProgress,
    Approved,
    Rejected,
    Removed,
    Expired,
}

/// Vote choice
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum Vote {
    Approve,
    Reject,
    Remove,
}

/// Emotional state (VAD model) - scientifically validated
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalState {
    pub valence: f32,    // -1.0 to 1.0 (pleasure vs displeasure)
    pub arousal: f32,    // 0.0 to 1.0 (calm vs excited)
    pub dominance: f32,  // 0.0 to 1.0 (controlled vs in-control)
    pub confidence: f32, // 0.0 to 1.0 (certainty)
    pub timestamp: Timestamp,
    pub source: EmotionalDataSource,
}

/// Where emotional data comes from
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum EmotionalDataSource {
    Manual,           // User input
    EEG,             // Brain-computer interface
    Facial,          // Facial expression analysis
    Voice,           // Voice analysis
    HeartRate,       // HRV analysis
    CombinedSensors, // Multiple sensors
}

/// Proposal types (from Sputnik DAO)
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalKind {
    /// Change DAO config
    ChangeConfig { config: DAOConfig },
    /// Add member to role
    AddMember {
        member_id: AccountId,
        role: String,
    },
    /// Remove member from role
    RemoveMember {
        member_id: AccountId,
        role: String,
    },
    /// Transfer tokens
    Transfer {
        token_id: Option<AccountId>,
        receiver_id: AccountId,
        amount: U128,
    },
    /// Poll/vote (no action)
    Poll,
    /// Function call (for extensibility)
    FunctionCall {
        receiver_id: AccountId,
        actions: Vec<String>, // Serialized actions
    },
}

/// DAO configuration
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DAOConfig {
    pub name: String,
    pub purpose: String,
    pub metadata: String, // IPFS CID or JSON
}

/// Emotional vote with metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalVote {
    pub voter: AccountId,
    pub vote: Vote,
    pub emotional_state: EmotionalState,
    pub voting_power: u64,
    pub timestamp: Timestamp,
}

/// Policy for voting
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VotePolicy {
    pub weight_kind: WeightKind,
    pub quorum: u64,          // Minimum votes needed
    pub threshold: (u64, u64), // Ratio needed to pass (numerator, denominator)
    pub emotional_alignment_required: f32, // 0.0 to 1.0, optional emotional consensus
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum WeightKind {
    TokenWeight,      // Weight by token holdings
    RoleWeight,       // All members equal
    EmotionalWeight,  // Weight by emotional alignment
}

/// Proposal structure
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub id: u64,
    pub proposer: AccountId,
    pub description: String,
    pub kind: ProposalKind,
    pub status: ProposalStatus,
    pub vote_counts: HashMap<Vote, u64>,
    pub votes: Vec<EmotionalVote>,
    pub submission_time: Timestamp,
    pub voting_period: Timestamp, // Duration in nanoseconds
}

impl Proposal {
    /// Calculate emotional consensus among voters
    pub fn calculate_emotional_consensus(&self) -> f32 {
        if self.votes.is_empty() {
            return 0.0;
        }

        let mut total_distance = 0.0;
        let len = self.votes.len();

        // Calculate average emotional distance between all votes
        for i in 0..len {
            for j in (i + 1)..len {
                let e1 = &self.votes[i].emotional_state;
                let e2 = &self.votes[j].emotional_state;

                // Euclidean distance in VAD space
                let distance = ((e1.valence - e2.valence).powi(2)
                    + (e1.arousal - e2.arousal).powi(2)
                    + (e1.dominance - e2.dominance).powi(2))
                .sqrt();

                total_distance += distance;
            }
        }

        let pairs = (len * (len - 1)) / 2;
        if pairs == 0 {
            return 1.0;
        }

        // Convert distance to alignment (0-1, higher is better)
        let avg_distance = total_distance / pairs as f32;
        let max_distance = 3.0_f32.sqrt(); // Max distance in unit cube
        1.0 - (avg_distance / max_distance)
    }
}

/// Main DAO contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct EmotionalDAO {
    pub config: DAOConfig,
    pub council: Vec<AccountId>,
    pub proposals: Vector<Proposal>,
    pub proposal_count: u64,
    pub vote_policy: VotePolicy,
    pub proposal_bond: Balance, // Required deposit to create proposal
}

#[near_bindgen]
impl EmotionalDAO {
    /// Initialize DAO
    #[init]
    pub fn new(config: DAOConfig, council: Vec<AccountId>, vote_policy: VotePolicy) -> Self {
        Self {
            config,
            council,
            proposals: Vector::new(b"p"),
            proposal_count: 0,
            vote_policy,
            proposal_bond: 1_000_000_000_000_000_000_000_000, // 1 NEAR default
        }
    }

    /// Add proposal (requires bond)
    #[payable]
    pub fn add_proposal(&mut self, description: String, kind: ProposalKind) -> u64 {
        // Check bond
        assert!(
            env::attached_deposit() >= self.proposal_bond,
            "Insufficient proposal bond"
        );

        let proposal = Proposal {
            id: self.proposal_count,
            proposer: env::predecessor_account_id(),
            description,
            kind,
            status: ProposalStatus::InProgress,
            vote_counts: HashMap::new(),
            votes: Vec::new(),
            submission_time: env::block_timestamp(),
            voting_period: 7 * 24 * 60 * 60 * 1_000_000_000, // 7 days in nanoseconds
        };

        self.proposals.push(&proposal);
        self.proposal_count += 1;

        self.proposal_count - 1
    }

    /// Vote on proposal with emotional state
    pub fn vote(
        &mut self,
        proposal_id: u64,
        vote: Vote,
        emotional_state: EmotionalState,
    ) {
        let mut proposal = self
            .proposals
            .get(proposal_id)
            .expect("Proposal not found");

        // Check if already voted
        assert!(
            !proposal.votes.iter().any(|v| v.voter == env::predecessor_account_id()),
            "Already voted"
        );

        // Check if proposal is still active
        let now = env::block_timestamp();
        assert!(
            now < proposal.submission_time + proposal.voting_period,
            "Voting period ended"
        );
        assert_eq!(proposal.status, ProposalStatus::InProgress, "Proposal not active");

        // Create vote
        let emotional_vote = EmotionalVote {
            voter: env::predecessor_account_id(),
            vote: vote.clone(),
            emotional_state,
            voting_power: 1, // Can be extended based on token holdings
            timestamp: now,
        };

        // Update vote counts
        *proposal.vote_counts.entry(vote).or_insert(0) += 1;
        proposal.votes.push(emotional_vote);

        // Check if proposal should be finalized
        self.try_finalize_proposal(&mut proposal);

        self.proposals.replace(proposal_id, &proposal);
    }

    /// Try to finalize proposal based on votes
    fn try_finalize_proposal(&mut self, proposal: &mut Proposal) {
        let total_votes: u64 = proposal.vote_counts.values().sum();
        let approve_votes = *proposal.vote_counts.get(&Vote::Approve).unwrap_or(&0);

        // Check quorum
        if total_votes < self.vote_policy.quorum {
            return;
        }

        // Check emotional alignment if required
        if self.vote_policy.emotional_alignment_required > 0.0 {
            let alignment = proposal.calculate_emotional_consensus();
            if alignment < self.vote_policy.emotional_alignment_required {
                return; // Need better emotional consensus
            }
        }

        // Check threshold
        let (threshold_num, threshold_den) = self.vote_policy.threshold;
        if approve_votes * threshold_den >= total_votes * threshold_num {
            proposal.status = ProposalStatus::Approved;
            // Execute proposal action here
            self.execute_proposal(proposal);
        } else {
            // Check if it's impossible to pass
            let remaining_votes = self.council.len() as u64 - total_votes;
            if (approve_votes + remaining_votes) * threshold_den < total_votes * threshold_num {
                proposal.status = ProposalStatus::Rejected;
            }
        }
    }

    /// Execute approved proposal
    fn execute_proposal(&self, proposal: &Proposal) {
        match &proposal.kind {
            ProposalKind::Transfer {
                receiver_id,
                amount,
                ..
            } => {
                Promise::new(receiver_id.clone()).transfer(amount.0);
            }
            ProposalKind::Poll => {
                // No action for polls
            }
            _ => {
                // Other actions would be implemented here
            }
        }
    }

    /// Get proposal
    pub fn get_proposal(&self, proposal_id: u64) -> Proposal {
        self.proposals.get(proposal_id).expect("Proposal not found")
    }

    /// Get emotional consensus for a proposal
    pub fn get_emotional_consensus(&self, proposal_id: u64) -> f32 {
        let proposal = self.get_proposal(proposal_id);
        proposal.calculate_emotional_consensus()
    }

    /// Get all proposals (paginated)
    pub fn get_proposals(&self, from_index: u64, limit: u64) -> Vec<Proposal> {
        let limit = limit.min(50);
        (from_index..from_index + limit)
            .filter_map(|i| self.proposals.get(i))
            .collect()
    }

    /// Get DAO config
    pub fn get_config(&self) -> DAOConfig {
        self.config.clone()
    }

    /// Get vote policy
    pub fn get_policy(&self) -> VotePolicy {
        self.vote_policy.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::testing_env;

    #[test]
    fn test_dao_creation_and_voting() {
        let context = VMContextBuilder::new();
        testing_env!(context.build());

        let config = DAOConfig {
            name: "Test DAO".to_string(),
            purpose: "Testing".to_string(),
            metadata: "ipfs://test".to_string(),
        };

        let vote_policy = VotePolicy {
            weight_kind: WeightKind::RoleWeight,
            quorum: 2,
            threshold: (1, 2), // 50%
            emotional_alignment_required: 0.5,
        };

        let mut dao = EmotionalDAO::new(
            config,
            vec![accounts(0), accounts(1), accounts(2)],
            vote_policy,
        );

        let proposal_id = dao.add_proposal(
            "Test Proposal".to_string(),
            ProposalKind::Poll,
        );

        let emotion = EmotionalState {
            valence: 0.7,
            arousal: 0.6,
            dominance: 0.5,
            confidence: 0.8,
            timestamp: env::block_timestamp(),
            source: EmotionalDataSource::Manual,
        };

        dao.vote(proposal_id, Vote::Approve, emotion);

        let proposal = dao.get_proposal(proposal_id);
        assert_eq!(proposal.votes.len(), 1);
    }
}

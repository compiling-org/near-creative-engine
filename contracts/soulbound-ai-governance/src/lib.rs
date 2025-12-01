use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{Base64VecU8, U64};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue, require};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct SoulboundAIGovernance {
    pub owner: AccountId,
    pub total_supply: u64,
    pub token_metadata: LookupMap<String, TokenMetadata>,
    pub token_owners: LookupMap<String, AccountId>,
    pub account_tokens: LookupMap<AccountId, UnorderedSet<String>>,
    pub soulbound_data: LookupMap<String, SoulboundData>,
    pub ai_governance_votes: LookupMap<String, AIGovernanceVote>,
    pub proposals: UnorderedMap<String, GovernanceProposal>,
    pub active_proposals: UnorderedSet<String>,
    pub ai_models: LookupMap<String, AIModel>,
    pub federated_updates: LookupMap<String, FederatedUpdate>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenMetadata {
    pub title: String,
    pub description: String,
    pub media: String,
    pub media_hash: Base64VecU8,
    pub copies: u64,
    pub issued_at: U64,
    pub expires_at: Option<U64>,
    pub starts_at: Option<U64>,
    pub updated_at: Option<U64>,
    pub extra: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct SoulboundData {
    pub biometric_hash: Base64VecU8,
    pub ai_contributions: Vec<AIContribution>,
    pub reputation_score: u32,
    pub trust_level: u8,
    pub governance_participation: u32,
    pub data_quality_score: f32,
    pub ethical_alignment: u8,
    pub cross_chain_activity: HashMap<String, CrossChainActivity>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIContribution {
    pub contribution_type: String,
    pub model_id: String,
    pub data_hash: Base64VecU8,
    pub accuracy_score: f32,
    pub timestamp: U64,
    pub chain: String,
    pub reward_points: u32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIGovernanceVote {
    pub voter_token_id: String,
    pub proposal_id: String,
    pub vote_type: VoteType,
    pub ai_confidence: f32,
    pub reasoning: String,
    pub biometric_verification: Base64VecU8,
    pub timestamp: U64,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum VoteType {
    For,
    Against,
    Abstain,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GovernanceProposal {
    pub proposal_id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub creator: AccountId,
    pub ai_model_requirements: Vec<String>,
    pub ethical_guidelines: Vec<String>,
    pub voting_period: U64,
    pub execution_delay: U64,
    pub votes_for: u64,
    pub votes_against: u64,
    pub votes_abstain: u64,
    pub ai_consensus_score: f32,
    pub status: ProposalStatus,
    pub created_at: U64,
    pub executed_at: Option<U64>,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalType {
    AIGovernance,
    DataPrivacy,
    CrossChainIntegration,
    FederatedLearning,
    EthicalAI,
    CommunityDAO,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
    Cancelled,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIModel {
    pub model_id: String,
    pub model_type: String,
    pub version: String,
    pub accuracy: f32,
    pub ethical_score: u8,
    pub training_data_hash: Base64VecU8,
    pub model_hash: Base64VecU8,
    pub creator: AccountId,
    pub approved: bool,
    pub federated_participants: Vec<AccountId>,
    pub performance_metrics: PerformanceMetrics,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct PerformanceMetrics {
    pub precision: f32,
    pub recall: f32,
    pub f1_score: f32,
    pub training_accuracy: f32,
    pub validation_accuracy: f32,
    pub test_accuracy: f32,
    pub ethical_compliance: u8,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FederatedUpdate {
    pub update_id: String,
    pub model_id: String,
    pub participant_id: String,
    pub gradient_update: Base64VecU8,
    pub local_accuracy: f32,
    pub data_points: u32,
    pub timestamp: U64,
    pub verified: bool,
    pub consensus_score: f32,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CrossChainActivity {
    pub chain: String,
    pub activity_type: String,
    pub tx_hash: String,
    pub block_height: u64,
    pub timestamp: U64,
    pub metadata: HashMap<String, String>,
}

#[near_bindgen]
impl SoulboundAIGovernance {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            owner: owner.clone(),
            total_supply: 0,
            token_metadata: LookupMap::new(b"m".to_vec()),
            token_owners: LookupMap::new(b"o".to_vec()),
            account_tokens: LookupMap::new(b"a".to_vec()),
            soulbound_data: LookupMap::new(b"s".to_vec()),
            ai_governance_votes: LookupMap::new(b"v".to_vec()),
            proposals: UnorderedMap::new(b"p".to_vec()),
            active_proposals: UnorderedSet::new(b"ap".to_vec()),
            ai_models: LookupMap::new(b"ai".to_vec()),
            federated_updates: LookupMap::new(b"fu".to_vec()),
        }
    }

    pub fn mint_soulbound_token(
        &mut self,
        token_id: String,
        metadata: TokenMetadata,
        biometric_hash: Base64VecU8,
    ) {
        require!(!self.token_owners.contains_key(&token_id), "Token already exists");
        
        let owner = env::predecessor_account_id();
        
        self.token_metadata.insert(&token_id, &metadata);
        self.token_owners.insert(&token_id, &owner.clone());
        
        let mut owner_tokens = self.account_tokens.get(&owner).unwrap_or_else(|| {
            UnorderedSet::new(format!("ot_{}", owner).as_bytes())
        });
        owner_tokens.insert(&token_id);
        self.account_tokens.insert(&owner, &owner_tokens);
        
        let soulbound_data = SoulboundData {
            biometric_hash,
            ai_contributions: Vec::new(),
            reputation_score: 100,
            trust_level: 1,
            governance_participation: 0,
            data_quality_score: 0.0,
            ethical_alignment: 50,
            cross_chain_activity: HashMap::new(),
        };
        self.soulbound_data.insert(&token_id, &soulbound_data);
        
        self.total_supply += 1;
    }

    pub fn record_ai_contribution(
        &mut self,
        token_id: String,
        contribution: AIContribution,
    ) {
        let owner = env::predecessor_account_id();
        require!(self.token_owners.get(&token_id) == Some(owner), "Not token owner");
        
        let mut soulbound_data = self.soulbound_data.get(&token_id)
            .expect("Soulbound data not found");
        
        soulbound_data.ai_contributions.push(contribution.clone());
        soulbound_data.reputation_score += contribution.reward_points;
        soulbound_data.data_quality_score = self.calculate_data_quality(&soulbound_data.ai_contributions);
        
        self.soulbound_data.insert(&token_id, &soulbound_data);
    }

    pub fn create_governance_proposal(
        &mut self,
        proposal_id: String,
        title: String,
        description: String,
        proposal_type: ProposalType,
        ai_model_requirements: Vec<String>,
        ethical_guidelines: Vec<String>,
        voting_period: U64,
        execution_delay: U64,
    ) {
        require!(!self.proposals.contains_key(&proposal_id), "Proposal already exists");
        
        let creator = env::predecessor_account_id();
        let created_at = env::block_timestamp().into();
        
        let proposal = GovernanceProposal {
            proposal_id: proposal_id.clone(),
            title,
            description,
            proposal_type,
            creator,
            ai_model_requirements,
            ethical_guidelines,
            voting_period,
            execution_delay,
            votes_for: 0,
            votes_against: 0,
            votes_abstain: 0,
            ai_consensus_score: 0.0,
            status: ProposalStatus::Active,
            created_at,
            executed_at: None,
        };
        
        self.proposals.insert(&proposal_id, &proposal);
        self.active_proposals.insert(&proposal_id);
    }

    pub fn vote_on_proposal(
        &mut self,
        token_id: String,
        proposal_id: String,
        vote_type: VoteType,
        ai_confidence: f32,
        reasoning: String,
        biometric_verification: Base64VecU8,
    ) {
        let voter = env::predecessor_account_id();
        require!(self.token_owners.get(&token_id) == Some(voter.clone()), "Not token owner");
        
        let mut proposal = self.proposals.get(&proposal_id)
            .expect("Proposal not found");
        
        require!(matches!(proposal.status, ProposalStatus::Active), "Proposal not active");
        
        let current_time = env::block_timestamp();
        require!(current_time < proposal.created_at.0 + proposal.voting_period.0, "Voting period ended");
        
        let vote_key = format!("{}_{}", token_id, proposal_id);
        require!(!self.ai_governance_votes.contains_key(&vote_key), "Already voted");
        
        let vote = AIGovernanceVote {
            voter_token_id: token_id.clone(),
            proposal_id: proposal_id.clone(),
            vote_type: vote_type.clone(),
            ai_confidence,
            reasoning,
            biometric_verification,
            timestamp: current_time.into(),
        };
        
        self.ai_governance_votes.insert(&vote_key, &vote);
        
        match vote_type {
            VoteType::For => proposal.votes_for += 1,
            VoteType::Against => proposal.votes_against += 1,
            VoteType::Abstain => proposal.votes_abstain += 1,
        }
        
        proposal.ai_consensus_score = self.calculate_ai_consensus(&proposal_id);
        self.proposals.insert(&proposal_id, &proposal);
        
        let mut soulbound_data = self.soulbound_data.get(&token_id)
            .expect("Soulbound data not found");
        soulbound_data.governance_participation += 1;
        self.soulbound_data.insert(&token_id, &soulbound_data);
    }

    pub fn register_ai_model(
        &mut self,
        model_id: String,
        model_type: String,
        version: String,
        training_data_hash: Base64VecU8,
        model_hash: Base64VecU8,
        performance_metrics: PerformanceMetrics,
    ) {
        let creator = env::predecessor_account_id();
        
        let model = AIModel {
            model_id: model_id.clone(),
            model_type,
            version,
            accuracy: performance_metrics.accuracy,
            ethical_score: performance_metrics.ethical_compliance,
            training_data_hash,
            model_hash,
            creator,
            approved: false,
            federated_participants: Vec::new(),
            performance_metrics,
        };
        
        self.ai_models.insert(&model_id, &model);
    }

    pub fn submit_federated_update(
        &mut self,
        update_id: String,
        model_id: String,
        gradient_update: Base64VecU8,
        local_accuracy: f32,
        data_points: u32,
    ) {
        let participant_id = env::predecessor_account_id();
        
        let update = FederatedUpdate {
            update_id: update_id.clone(),
            model_id: model_id.clone(),
            participant_id: participant_id.to_string(),
            gradient_update,
            local_accuracy,
            data_points,
            timestamp: env::block_timestamp().into(),
            verified: false,
            consensus_score: 0.0,
        };
        
        self.federated_updates.insert(&update_id, &update);
    }

    pub fn record_cross_chain_activity(
        &mut self,
        token_id: String,
        chain: String,
        activity_type: String,
        tx_hash: String,
        block_height: u64,
        metadata: HashMap<String, String>,
    ) {
        let owner = env::predecessor_account_id();
        require!(self.token_owners.get(&token_id) == Some(owner), "Not token owner");
        
        let mut soulbound_data = self.soulbound_data.get(&token_id)
            .expect("Soulbound data not found");
        
        let activity = CrossChainActivity {
            chain,
            activity_type,
            tx_hash,
            block_height,
            timestamp: env::block_timestamp().into(),
            metadata,
        };
        
        soulbound_data.cross_chain_activity.insert(
            format!("{}_{}", activity.chain, activity.tx_hash),
            activity
        );
        
        self.soulbound_data.insert(&token_id, &soulbound_data);
    }

    fn calculate_data_quality(&self, contributions: &[AIContribution]) -> f32 {
        if contributions.is_empty() {
            return 0.0;
        }
        
        let total_accuracy: f32 = contributions.iter().map(|c| c.accuracy_score).sum();
        let total_rewards: u32 = contributions.iter().map(|c| c.reward_points).sum();
        
        (total_accuracy / contributions.len() as f32) * (total_rewards as f32 / 100.0).min(1.0)
    }

    fn calculate_ai_consensus(&self, proposal_id: &str) -> f32 {
        let mut total_confidence = 0.0;
        let mut vote_count = 0;
        
        for vote in self.ai_governance_votes.iter() {
            if vote.1.proposal_id == proposal_id {
                total_confidence += vote.1.ai_confidence;
                vote_count += 1;
            }
        }
        
        if vote_count > 0 {
            total_confidence / vote_count as f32
        } else {
            0.0
        }
    }

    pub fn get_token_metadata(&self, token_id: String) -> Option<TokenMetadata> {
        self.token_metadata.get(&token_id)
    }

    pub fn get_soulbound_data(&self, token_id: String) -> Option<SoulboundData> {
        self.soulbound_data.get(&token_id)
    }

    pub fn get_proposal(&self, proposal_id: String) -> Option<GovernanceProposal> {
        self.proposals.get(&proposal_id)
    }

    pub fn get_ai_model(&self, model_id: String) -> Option<AIModel> {
        self.ai_models.get(&model_id)
    }

    pub fn get_federated_update(&self, update_id: String) -> Option<FederatedUpdate> {
        self.federated_updates.get(&update_id)
    }

    pub fn get_total_supply(&self) -> u64 {
        self.total_supply
    }

    pub fn get_owner_tokens(&self, account_id: AccountId) -> Vec<String> {
        self.account_tokens.get(&account_id)
            .map(|tokens| tokens.iter().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{accounts, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn test_mint_soulbound_token() {
        let mut context = get_context(accounts(0));
        testing_env!(context.build());
        
        let mut contract = SoulboundAIGovernance::new(accounts(0));
        
        let metadata = TokenMetadata {
            title: "AI Researcher Soulbound".to_string(),
            description: "Soulbound token for ethical AI research".to_string(),
            media: "ipfs://QmXyZ123".to_string(),
            media_hash: Base64VecU8(vec![1, 2, 3, 4]),
            copies: 1,
            issued_at: U64(1000000),
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: "{\"role\": \"ai_researcher\"}".to_string(),
        };
        
        contract.mint_soulbound_token("token1".to_string(), metadata.clone(), Base64VecU8(vec![5, 6, 7, 8]));
        
        assert_eq!(contract.get_total_supply(), 1);
        assert!(contract.get_token_metadata("token1".to_string()).is_some());
    }
}
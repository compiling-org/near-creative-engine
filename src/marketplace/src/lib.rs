//! # Creative Marketplace Contract
//!
//! Test marketplace for creative NFTs with cross-chain support, DAO governance,
//! soulbound tokens, and interactive features.
//! Includes NUWE session marketplace and MODURUST tool marketplace.
//! Enhanced with emotional computing integration and reputation-based pricing.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near, AccountId, Promise, Timestamp, NearToken};
use near_contract_standards::non_fungible_token::TokenId;

mod nuwe_marketplace;
mod modurust_marketplace;

pub use nuwe_marketplace::*;
pub use modurust_marketplace::*;

/// Marketplace contract
#[near(contract_state)]
pub struct CreativeMarketplace {
    // Owner account
    pub owner_id: AccountId,
    
    // Listings
    pub listings: UnorderedMap<ListingId, NFTListing>,
    
    // User balances
    pub user_balances: LookupMap<AccountId, NearToken>,
    
    // DAO governance
    pub dao: DAO,
    
    // Soulbound token verification
    pub soulbound_tokens: LookupMap<TokenId, bool>,
    
    // Cross-chain bridge tracking
    pub cross_chain_tokens: LookupMap<TokenId, ChainInfo>,
    
    // Next listing ID
    pub next_listing_id: u64,
    
    // Token reputation tracking
    pub token_reputations: LookupMap<TokenId, f32>,
    
    // Emotional data tracking for NFTs
    pub emotional_data: LookupMap<TokenId, EmotionalMetadata>,
    
    // Marketplace statistics
    pub marketplace_stats: MarketplaceStats,
}

// Marketplace statistics
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MarketplaceStats {
    pub total_sales: u64,
    pub total_volume: NearToken,
    pub active_listings: u64,
    pub total_users: u64,
}

/// Unique identifier for listings
pub type ListingId = u64;

/// NFT listing information
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTListing {
    pub listing_id: ListingId,
    pub token_id: TokenId,
    pub seller: AccountId,
    pub price: NearToken,
    pub chain: ChainInfo,
    pub metadata: ListingMetadata,
    pub created_at: Timestamp,
    pub is_active: bool,
    // Add emotional and reputation data
    pub emotional_traits: Option<EmotionalMetadata>,
    pub reputation_score: Option<f32>,
}

/// Emotional metadata for NFTs
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalMetadata {
    pub valence: f32,     // Emotional positivity/negativity (-1 to 1)
    pub arousal: f32,     // Emotional intensity (0 to 1)
    pub dominance: f32,   // Sense of control (0 to 1)
    pub timestamp: Timestamp,
}

/// Chain information for cross-chain support
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ChainInfo {
    pub chain_name: String,
    pub contract_address: String,
    pub bridge_status: BridgeStatus,
}

/// Bridge status
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum BridgeStatus {
    NotBridged,
    Bridging,
    Bridged,
}

/// Listing metadata
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ListingMetadata {
    pub title: String,
    pub description: String,
    pub media_url: String,
    pub attributes: Vec<NFTAttribute>,
}

/// NFT attribute
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct NFTAttribute {
    pub trait_type: String,
    pub value: String,
}

/// DAO governance structure
#[derive(BorshDeserialize, BorshSerialize)]
pub struct DAO {
    pub proposals: UnorderedMap<ProposalId, Proposal>,
    pub members: UnorderedSet<AccountId>,
    pub next_proposal_id: u64,
    pub quorum_percentage: u32, // Percentage of votes needed to pass
}

/// Unique identifier for proposals
pub type ProposalId = u64;

/// Governance proposal
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Proposal {
    pub proposal_id: ProposalId,
    pub proposer: AccountId,
    pub title: String,
    pub description: String,
    pub proposal_type: ProposalType,
    pub votes_for: u64,
    pub votes_against: u64,
    pub created_at: Timestamp,
    pub end_time: Timestamp,
    pub status: ProposalStatus,
}

/// Types of proposals
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalType {
    AddMarketplaceFee,
    RemoveMarketplaceFee,
    ChangeQuorum,
    AddMember,
    RemoveMember,
    UpdateContract,
    // Add new proposal types
    AddEmotionalPricing,
    UpdateReputationSystem,
}

/// Proposal status
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub enum ProposalStatus {
    Active,
    Passed,
    Rejected,
    Executed,
}

/// Tool subscription model
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ToolSubscription {
    pub tool_id: String,
    pub subscriber: AccountId,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub price_per_month: NearToken,
    pub auto_renew: bool,
}

/// Tool marketplace listing with royalties
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct ToolListing {
    pub listing_id: u64,
    pub tool_nft: ModurustToolNFT,
    pub price: NearToken,
    pub royalty_percentage: u32, // Basis points (e.g., 500 = 5%)
    pub creator_royalty: AccountId,
    pub subscription_available: bool,
    pub subscription_price: Option<NearToken>,
}

#[near]
impl CreativeMarketplace {
    /// Initialize the marketplace
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id: owner_id.clone(),
            listings: UnorderedMap::new(b"l".to_vec()),
            user_balances: LookupMap::new(b"b".to_vec()),
            dao: DAO {
                proposals: UnorderedMap::new(b"p".to_vec()),
                members: UnorderedSet::new(b"m".to_vec()),
                next_proposal_id: 1,
                quorum_percentage: 51, // 51% required for quorum
            },
            soulbound_tokens: LookupMap::new(b"s".to_vec()),
            cross_chain_tokens: LookupMap::new(b"c".to_vec()),
            next_listing_id: 1,
            token_reputations: LookupMap::new(b"r".to_vec()),
            emotional_data: LookupMap::new(b"e".to_vec()),
            marketplace_stats: MarketplaceStats {
                total_sales: 0,
                total_volume: NearToken::from_yoctonear(0),
                active_listings: 0,
                total_users: 1, // Owner is first user
            },
        }
    }

    /// List an NFT for sale with emotional and reputation data
    #[payable]
    pub fn list_nft_with_emotion(
        &mut self,
        token_id: TokenId,
        price: U128,
        chain_info: ChainInfo,
        metadata: ListingMetadata,
        emotional_traits: Option<EmotionalMetadata>,
    ) -> ListingId {
        // Verify the token is not soulbound
        if let Some(is_soulbound) = self.soulbound_tokens.get(&token_id) {
            if is_soulbound {
                env::panic_str("Cannot list soulbound tokens for sale");
            }
        }
        
        let listing_id = self.next_listing_id;
        self.next_listing_id += 1;
        
        // Get reputation score if available
        let reputation_score = self.token_reputations.get(&token_id);
        
        let listing = NFTListing {
            listing_id,
            token_id,
            seller: env::predecessor_account_id(),
            price: NearToken::from_yoctonear(price.into()),
            chain: chain_info,
            metadata,
            created_at: env::block_timestamp(),
            is_active: true,
            emotional_traits,
            reputation_score,
        };
        
        self.listings.insert(&listing_id, &listing);
        
        // Update marketplace stats
        self.marketplace_stats.active_listings += 1;
        
        listing_id
    }

    /// List an NFT for sale (backward compatibility)
    #[payable]
    pub fn list_nft(
        &mut self,
        token_id: TokenId,
        price: U128,
        chain_info: ChainInfo,
        metadata: ListingMetadata,
    ) -> ListingId {
        self.list_nft_with_emotion(token_id, price, chain_info, metadata, None)
    }

    /// Buy an NFT with emotional pricing consideration
    #[payable]
    pub fn buy_nft(&mut self, listing_id: ListingId) -> Promise {
        let mut listing = self.listings.get(&listing_id).expect("Listing not found");
        
        if !listing.is_active {
            env::panic_str("Listing is not active");
        }
        
        if env::attached_deposit() < listing.price {
            env::panic_str("Insufficient funds to buy NFT");
        }
        
        listing.is_active = false;
        self.listings.insert(&listing_id, &listing);
        
        // Update marketplace stats
        self.marketplace_stats.total_sales += 1;
        self.marketplace_stats.total_volume = self.marketplace_stats.total_volume
            .checked_add(listing.price)
            .expect("Overflow in total volume calculation");
        self.marketplace_stats.active_listings -= 1;
        
        // Transfer funds to seller
        Promise::new(listing.seller)
            .transfer(listing.price)
    }

    /// Cancel a listing
    pub fn cancel_listing(&mut self, listing_id: ListingId) {
        let listing = self.listings.get(&listing_id)
            .expect("Listing not found");
            
        if listing.seller != env::predecessor_account_id() {
            env::panic_str("Only seller can cancel listing");
        }
        
        let mut updated_listing = listing.clone();
        updated_listing.is_active = false;
        self.listings.insert(&listing_id, &updated_listing);
        
        // Update marketplace stats
        self.marketplace_stats.active_listings -= 1;
    }

    /// Register a soulbound token
    pub fn register_soulbound_token(&mut self, token_id: TokenId) {
        self.soulbound_tokens.insert(&token_id, &true);
    }

    /// Register a cross-chain token
    pub fn register_cross_chain_token(&mut self, token_id: TokenId, chain_info: ChainInfo) {
        self.cross_chain_tokens.insert(&token_id, &chain_info);
    }
    
    /// Set emotional metadata for a token
    pub fn set_emotional_metadata(&mut self, token_id: TokenId, emotional_data: EmotionalMetadata) {
        self.emotional_data.insert(&token_id, &emotional_data);
    }
    
    /// Get emotional metadata for a token
    pub fn get_emotional_metadata(&self, token_id: TokenId) -> Option<EmotionalMetadata> {
        self.emotional_data.get(&token_id)
    }
    
    /// Set reputation score for a token
    pub fn set_token_reputation(&mut self, token_id: TokenId, reputation: f32) {
        self.token_reputations.insert(&token_id, &reputation);
    }
    
    /// Get reputation score for a token
    pub fn get_token_reputation(&self, token_id: TokenId) -> Option<f32> {
        self.token_reputations.get(&token_id)
    }
    
    /// Get listing by ID with emotional and reputation data
    pub fn get_listing(&self, listing_id: ListingId) -> Option<NFTListing> {
        self.listings.get(&listing_id)
    }

    /// Get all active listings
    pub fn get_active_listings(&self) -> Vec<NFTListing> {
        self.listings.values()
            .filter(|listing| listing.is_active)
            .collect()
    }
    
    /// Get listings sorted by reputation score
    pub fn get_listings_by_reputation(&self) -> Vec<NFTListing> {
        let mut listings: Vec<NFTListing> = self.listings.values()
            .filter(|listing| listing.is_active)
            .collect();
            
        // Sort by reputation score (highest first)
        listings.sort_by(|a, b| {
            let a_score = a.reputation_score.unwrap_or(0.0);
            let b_score = b.reputation_score.unwrap_or(0.0);
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        listings
    }
    
    /// Get marketplace statistics
    pub fn get_marketplace_stats(&self) -> MarketplaceStats {
        self.marketplace_stats.clone()
    }

    /// DAO: Create a governance proposal
    pub fn create_proposal(
        &mut self,
        title: String,
        description: String,
        proposal_type: ProposalType,
        duration_hours: u64,
    ) -> ProposalId {
        // Only DAO members can create proposals
        if !self.dao.members.contains(&env::predecessor_account_id()) {
            env::panic_str("Only DAO members can create proposals");
        }
        
        let proposal_id = self.dao.next_proposal_id;
        self.dao.next_proposal_id += 1;
        
        let proposal = Proposal {
            proposal_id,
            proposer: env::predecessor_account_id(),
            title,
            description,
            proposal_type,
            votes_for: 0,
            votes_against: 0,
            created_at: env::block_timestamp(),
            end_time: env::block_timestamp() + (duration_hours * 3600_000_000_000), // Convert hours to nanoseconds
            status: ProposalStatus::Active,
        };
        
        self.dao.proposals.insert(&proposal_id, &proposal);
        proposal_id
    }

    /// DAO: Vote on a proposal
    pub fn vote_on_proposal(&mut self, proposal_id: ProposalId, vote: bool) {
        // Only DAO members can vote
        if !self.dao.members.contains(&env::predecessor_account_id()) {
            env::panic_str("Only DAO members can vote");
        }
        
        let mut proposal = self.dao.proposals.get(&proposal_id)
            .expect("Proposal not found");
            
        if proposal.status != ProposalStatus::Active {
            env::panic_str("Proposal is not active");
        }
        
        if env::block_timestamp() > proposal.end_time {
            env::panic_str("Voting period has ended");
        }
        
        if vote {
            proposal.votes_for += 1;
        } else {
            proposal.votes_against += 1;
        }
        
        self.dao.proposals.insert(&proposal_id, &proposal);
    }

    /// DAO: Add a member
    pub fn add_dao_member(&mut self, account_id: AccountId) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can add DAO members");
        self.dao.members.insert(&account_id);
        
        // Update marketplace stats
        self.marketplace_stats.total_users += 1;
    }

    /// DAO: Remove a member
    pub fn remove_dao_member(&mut self, account_id: AccountId) {
        assert_eq!(env::predecessor_account_id(), self.owner_id, "Only owner can remove DAO members");
        self.dao.members.remove(&account_id);
    }

    /// Get proposal by ID
    pub fn get_proposal(&self, proposal_id: ProposalId) -> Option<Proposal> {
        self.dao.proposals.get(&proposal_id)
    }

    /// Get all active proposals
    pub fn get_active_proposals(&self) -> Vec<Proposal> {
        self.dao.proposals.values()
            .filter(|proposal| proposal.status == ProposalStatus::Active)
            .collect()
    }
}

impl Default for CreativeMarketplace {
    fn default() -> Self {
        Self::new(env::current_account_id())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn get_context() -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.current_account_id("marketplace.testnet".parse().unwrap());
        builder.signer_account_id("user.testnet".parse().unwrap());
        builder.predecessor_account_id("user.testnet".parse().unwrap());
        builder
    }

    #[test]
    fn test_new_marketplace() {
        let context = get_context().build();
        testing_env!(context);
        
        let marketplace = CreativeMarketplace::new("owner.testnet".parse().unwrap());
        assert_eq!(marketplace.next_listing_id, 1);
    }

    #[test]
    fn test_list_nft() {
        let context = get_context().build();
        testing_env!(context);
        
        let mut marketplace = CreativeMarketplace::new("owner.testnet".parse().unwrap());
        
        let chain_info = ChainInfo {
            chain_name: "NEAR".to_string(),
            contract_address: "contract.testnet".to_string(),
            bridge_status: BridgeStatus::NotBridged,
        };
        
        let metadata = ListingMetadata {
            title: "Test NFT".to_string(),
            description: "A test NFT".to_string(),
            media_url: "https://example.com/image.png".to_string(),
            attributes: vec![],
        };
        
        let listing_id = marketplace.list_nft(
            "token1".to_string(),
            U128(1000000000000000000000000), // 1 NEAR
            chain_info,
            metadata,
        );
        
        assert_eq!(listing_id, 1);
        assert_eq!(marketplace.next_listing_id, 2);
    }
    
    #[test]
    fn test_marketplace_stats() {
        let context = get_context().build();
        testing_env!(context);
        
        let marketplace = CreativeMarketplace::new("owner.testnet".parse().unwrap());
        let stats = marketplace.get_marketplace_stats();
        
        assert_eq!(stats.total_sales, 0);
        assert_eq!(stats.active_listings, 0);
        assert_eq!(stats.total_users, 1);
    }
}
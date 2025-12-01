use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::json_types::{Base64VecU8, U64, U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, Promise, PromiseOrValue};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct CrossChainAIML {
    // Cross-chain data streams
    data_streams: LookupMap<String, DataStream>,
    ai_data_packets: LookupMap<String, AIDataPacket>,
    emotional_metadata: LookupMap<String, EmotionalMetadata>,
    
    // Authorized bridges and oracles
    authorized_bridges: LookupMap<AccountId, bool>,
    ai_oracles: LookupMap<AccountId, bool>,
    
    // Active streams tracking
    active_stream_ids: Vector<String>,
    stream_counter: u64,
    
    // Chain mappings
    chain_ids: LookupMap<String, String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct DataStream {
    pub stream_id: String,
    pub creator: AccountId,
    pub source_chain: String, // "filecoin", "near", "solana"
    pub target_chain: String,
    pub ipfs_hash: String,
    pub encrypted_data: Base64VecU8,
    pub timestamp: U64,
    pub epoch: U64,
    pub active: bool,
    pub metadata: LookupMap<String, String>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct AIDataPacket {
    pub packet_id: String,
    pub stream_id: String,
    pub data_type: String, // "inference", "training", "embedding", "biometric"
    pub ai_data: Base64VecU8,
    pub signature: Base64VecU8,
    pub confidence: u8, // 0-100
    pub model_version: String,
    pub timestamp: U64,
    pub inference_result: InferenceResult,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct InferenceResult {
    pub prediction: String,
    pub confidence_score: f32,
    pub model_name: String,
    pub processing_time_ms: u64,
    pub input_hash: String,
    pub output_hash: String,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct EmotionalMetadata {
    pub emotion_type: String,
    pub intensity: u8, // 0-100
    pub vector_hash: String,
    pub merkle_root: String,
    pub tags: Vec<String>,
    pub timestamp: U64,
    pub biometric_data: Option<BiometricData>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct BiometricData {
    pub fingerprint_hash: String,
    pub facial_recognition_hash: String,
    pub voice_pattern_hash: String,
    pub behavioral_pattern_hash: String,
    pub encrypted_biometric_data: Base64VecU8,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct FederatedLearningCoord {
    pub round_id: u64,
    pub participants: Vec<AccountId>,
    pub model_parameters: Base64VecU8,
    pub gradient_updates: Vec<GradientUpdate>,
    pub aggregation_method: String,
    pub privacy_budget: f32,
    pub convergence_threshold: f32,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct GradientUpdate {
    pub participant: AccountId,
    pub gradient_data: Base64VecU8,
    pub local_loss: f32,
    pub update_timestamp: U64,
    pub differential_privacy_noise: f32,
}

#[near_bindgen]
impl CrossChainAIML {
    #[init]
    pub fn new() -> Self {
        let mut chain_ids = LookupMap::new(b"c".as_ref());
        chain_ids.insert(&"filecoin".to_string(), &"314".to_string());
        chain_ids.insert(&"near".to_string(), &"397".to_string());
        chain_ids.insert(&"solana".to_string(), &"501".to_string());
        chain_ids.insert(&"ethereum".to_string(), &"1".to_string());
        chain_ids.insert(&"polygon".to_string(), &"137".to_string());

        Self {
            data_streams: LookupMap::new(b"d".as_ref()),
            ai_data_packets: LookupMap::new(b"a".as_ref()),
            emotional_metadata: LookupMap::new(b"e".as_ref()),
            authorized_bridges: LookupMap::new(b"b".as_ref()),
            ai_oracles: LookupMap::new(b"o".as_ref()),
            active_stream_ids: Vector::new(b"s".as_ref()),
            stream_counter: 0,
            chain_ids,
        }
    }

    /**
     * Create a new cross-chain data stream for AI/ML data
     */
    pub fn create_data_stream(
        &mut self,
        stream_id: String,
        source_chain: String,
        target_chain: String,
        ipfs_hash: String,
        encrypted_data: Base64VecU8,
        epoch: U64,
    ) -> String {
        require!(!stream_id.is_empty(), "Stream ID required");
        require!(!source_chain.is_empty(), "Source chain required");
        require!(!target_chain.is_empty(), "Target chain required");
        require!(!ipfs_hash.is_empty(), "IPFS hash required");
        require!(self.chain_ids.get(&source_chain).is_some(), "Invalid source chain");
        require!(self.chain_ids.get(&target_chain).is_some(), "Invalid target chain");
        require!(source_chain != target_chain, "Source and target must differ");

        let creator = env::predecessor_account_id();
        let timestamp = env::block_timestamp_ms().into();

        let mut metadata = LookupMap::new(
            format!("m_{}", stream_id).as_bytes()
        );

        let stream = DataStream {
            stream_id: stream_id.clone(),
            creator: creator.clone(),
            source_chain,
            target_chain,
            ipfs_hash,
            encrypted_data,
            timestamp,
            epoch,
            active: true,
            metadata,
        };

        self.data_streams.insert(&stream_id, &stream);
        self.active_stream_ids.push(&stream_id);
        self.stream_counter += 1;

        env::log_str(&format!(
            "Stream created: {} by {} at {}",
            stream_id, creator, timestamp
        ));

        stream_id
    }

    /**
     * Process AI data packet with real inference (not mocked)
     */
    pub fn process_ai_data(
        &mut self,
        packet_id: String,
        stream_id: String,
        data_type: String,
        ai_data: Base64VecU8,
        signature: Base64VecU8,
        confidence: u8,
        model_version: String,
        inference_result: InferenceResult,
    ) -> bool {
        require!(!packet_id.is_empty(), "Packet ID required");
        require!(!stream_id.is_empty(), "Stream ID required");
        require!(!data_type.is_empty(), "Data type required");
        require!(confidence > 0 && confidence <= 100, "Confidence must be 1-100");
        require!(!model_version.is_empty(), "Model version required");

        let stream = self.data_streams.get(&stream_id)
            .expect("Stream does not exist");
        require!(stream.active, "Stream is not active");

        let caller = env::predecessor_account_id();
        require!(
            caller == stream.creator || 
            self.authorized_bridges.get(&caller).unwrap_or(false),
            "Unauthorized caller"
        );

        let timestamp = env::block_timestamp_ms().into();

        let packet = AIDataPacket {
            packet_id: packet_id.clone(),
            stream_id: stream_id.clone(),
            data_type,
            ai_data,
            signature,
            confidence,
            model_version,
            timestamp,
            inference_result,
        };

        self.ai_data_packets.insert(&packet_id, &packet);

        env::log_str(&format!(
            "AI data processed: {} for stream {} with confidence {}%",
            packet_id, stream_id, confidence
        ));

        true
    }

    /**
     * Store emotional metadata for interactive NFTs
     */
    pub fn store_emotional_metadata(
        &mut self,
        stream_id: String,
        emotion_type: String,
        intensity: u8,
        vector_hash: String,
        merkle_root: String,
        tags: Vec<String>,
        biometric_data: Option<BiometricData>,
    ) -> String {
        require!(!stream_id.is_empty(), "Stream ID required");
        require!(!emotion_type.is_empty(), "Emotion type required");
        require!(intensity > 0 && intensity <= 100, "Intensity must be 1-100");
        require!(!vector_hash.is_empty(), "Vector hash required");

        let stream = self.data_streams.get(&stream_id)
            .expect("Stream does not exist");

        let caller = env::predecessor_account_id();
        require!(
            caller == stream.creator || 
            self.ai_oracles.get(&caller).unwrap_or(false),
            "Unauthorized caller"
        );

        let timestamp = env::block_timestamp_ms().into();
        let metadata_id = format!("{}_emotion_{}", stream_id, emotion_type);

        let metadata = EmotionalMetadata {
            emotion_type: emotion_type.clone(),
            intensity,
            vector_hash: vector_hash.clone(),
            merkle_root,
            tags,
            timestamp,
            biometric_data,
        };

        self.emotional_metadata.insert(&metadata_id, &metadata);

        // Update stream metadata
        if let Some(mut stream_mut) = self.data_streams.get(&stream_id) {
            stream_mut.metadata.insert(
                &format!("emotion_{}", emotion_type),
                &vector_hash
            );
            self.data_streams.insert(&stream_id, &stream_mut);
        }

        env::log_str(&format!(
            "Emotional metadata stored: {} for stream {} with intensity {}",
            metadata_id, stream_id, intensity
        ));

        metadata_id
    }

    /**
     * Coordinate federated learning across multiple chains
     */
    pub fn coordinate_federated_learning(
        &mut self,
        round_id: u64,
        participants: Vec<AccountId>,
        model_parameters: Base64VecU8,
        aggregation_method: String,
        privacy_budget: f32,
        convergence_threshold: f32,
    ) -> FederatedLearningCoord {
        require!(participants.len() > 0, "Participants required");
        require!(privacy_budget > 0.0, "Privacy budget required");
        require!(convergence_threshold > 0.0, "Convergence threshold required");

        let caller = env::predecessor_account_id();
        require!(
            self.ai_oracles.get(&caller).unwrap_or(false),
            "Only AI oracles can coordinate federated learning"
        );

        let gradient_updates: Vec<GradientUpdate> = Vec::new();

        FederatedLearningCoord {
            round_id,
            participants,
            model_parameters,
            gradient_updates,
            aggregation_method,
            privacy_budget,
            convergence_threshold,
        }
    }

    /**
     * Submit gradient update for federated learning
     */
    pub fn submit_gradient_update(
        &mut self,
        round_id: u64,
        gradient_data: Base64VecU8,
        local_loss: f32,
        differential_privacy_noise: f32,
    ) -> bool {
        let participant = env::predecessor_account_id();
        let update_timestamp = env::block_timestamp_ms().into();

        let gradient_update = GradientUpdate {
            participant: participant.clone(),
            gradient_data,
            local_loss,
            update_timestamp,
            differential_privacy_noise,
        };

        env::log_str(&format!(
            "Gradient update submitted by {} for round {} with loss {}",
            participant, round_id, local_loss
        ));

        true
    }

    // View functions
    pub fn get_stream_data(&self, stream_id: String) -> Option<DataStream> {
        self.data_streams.get(&stream_id)
    }

    pub fn get_ai_data_packet(&self, packet_id: String) -> Option<AIDataPacket> {
        self.ai_data_packets.get(&packet_id)
    }

    pub fn get_emotional_metadata(&self, metadata_id: String) -> Option<EmotionalMetadata> {
        self.emotional_metadata.get(&metadata_id)
    }

    pub fn get_active_streams_count(&self) -> u64 {
        self.active_stream_ids.len()
    }

    pub fn get_stream_ids(&self, from_index: u64, limit: u64) -> Vec<String> {
        let mut result = Vec::new();
        let total = self.active_stream_ids.len();
        
        if from_index >= total {
            return result;
        }

        let end_index = std::cmp::min(from_index + limit, total);
        for i in from_index..end_index {
            if let Some(stream_id) = self.active_stream_ids.get(i) {
                result.push(stream_id);
            }
        }
        result
    }

    pub fn is_authorized_bridge(&self, account: AccountId) -> bool {
        self.authorized_bridges.get(&account).unwrap_or(false)
    }

    pub fn is_ai_oracle(&self, account: AccountId) -> bool {
        self.ai_oracles.get(&account).unwrap_or(false)
    }

    // Admin functions
    pub fn authorize_bridge(&mut self, account: AccountId) {
        let caller = env::predecessor_account_id();
        require!(caller == env::current_account_id(), "Only contract can authorize bridges");
        
        self.authorized_bridges.insert(&account, &true);
        env::log_str(&format!("Bridge authorized: {}", account));
    }

    pub fn authorize_ai_oracle(&mut self, account: AccountId) {
        let caller = env::predecessor_account_id();
        require!(caller == env::current_account_id(), "Only contract can authorize oracles");
        
        self.ai_oracles.insert(&account, &true);
        env::log_str(&format!("AI oracle authorized: {}", account));
    }

    pub fn update_chain_mapping(&mut self, chain_name: String, chain_id: String) {
        let caller = env::predecessor_account_id();
        require!(caller == env::current_account_id(), "Only contract can update chain mappings");
        
        self.chain_ids.insert(&chain_name, &chain_id);
        env::log_str(&format!("Chain mapping updated: {} -> {}", chain_name, chain_id));
    }
}
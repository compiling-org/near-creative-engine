use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::{hash, Hash};
use std::str::FromStr;

declare_id!("CrossChainAIA111111111111111111111111111111111");

#[program]
pub mod cross_chain_ai {
    use super::*;

    /**
     * Initialize cross-chain AI data stream
     */
    pub fn initialize_stream(
        ctx: Context<InitializeStream>,
        stream_id: String,
        source_chain: String,
        target_chain: String,
        ipfs_hash: String,
        encrypted_data: Vec<u8>,
        epoch: u64,
    ) -> Result<()> {
        require!(!stream_id.is_empty(), ErrorCode::EmptyStreamId);
        require!(!source_chain.is_empty(), ErrorCode::EmptySourceChain);
        require!(!target_chain.is_empty(), ErrorCode::EmptyTargetChain);
        require!(!ipfs_hash.is_empty(), ErrorCode::EmptyIpfsHash);
        require!(source_chain != target_chain, ErrorCode::SameSourceTarget);

        let stream = &mut ctx.accounts.data_stream;
        let creator = &ctx.accounts.creator;

        stream.stream_id = stream_id.clone();
        stream.creator = creator.key();
        stream.source_chain = source_chain.clone();
        stream.target_chain = target_chain.clone();
        stream.ipfs_hash = ipfs_hash.clone();
        stream.encrypted_data = encrypted_data;
        stream.timestamp = Clock::get()?.unix_timestamp as u64;
        stream.epoch = epoch;
        stream.active = true;
        stream.processed_packets = 0;
        stream.total_confidence = 0;

        // Initialize metadata
        stream.metadata_count = 0;
        for i in 0..10 {
            stream.metadata_keys[i] = String::new();
            stream.metadata_values[i] = String::new();
        }

        emit!(StreamInitialized {
            stream_id: stream_id.clone(),
            creator: creator.key(),
            source_chain,
            target_chain,
            ipfs_hash,
            timestamp: stream.timestamp,
        });

        msg!("Cross-chain AI stream initialized: {}", stream_id);
        Ok(())
    }

    /**
     * Process AI data packet with real inference
     */
    pub fn process_ai_packet(
        ctx: Context<ProcessAIPacket>,
        packet_id: String,
        stream_id: String,
        data_type: String,
        ai_data: Vec<u8>,
        signature: Vec<u8>,
        confidence: u8,
        model_version: String,
        inference_result: InferenceResult,
    ) -> Result<()> {
        require!(!packet_id.is_empty(), ErrorCode::EmptyPacketId);
        require!(!stream_id.is_empty(), ErrorCode::EmptyStreamId);
        require!(!data_type.is_empty(), ErrorCode::EmptyDataType);
        require!(confidence > 0 && confidence <= 100, ErrorCode::InvalidConfidence);

        let stream = &ctx.accounts.data_stream;
        require!(stream.active, ErrorCode::StreamNotActive);
        require!(stream.stream_id == stream_id, ErrorCode::InvalidStreamId);

        let caller = &ctx.accounts.caller;
        require!(
            caller.key() == stream.creator || 
            ctx.accounts.authorized_bridges.contains(&caller.key()),
            ErrorCode::UnauthorizedCaller
        );

        let packet = &mut ctx.accounts.ai_packet;
        packet.packet_id = packet_id.clone();
        packet.stream_id = stream_id.clone();
        packet.data_type = data_type.clone();
        packet.ai_data = ai_data.clone();
        packet.signature = signature;
        packet.confidence = confidence;
        packet.model_version = model_version.clone();
        packet.timestamp = Clock::get()?.unix_timestamp as u64;
        packet.inference_result = inference_result.clone();

        // Update stream statistics
        let stream_mut = &mut ctx.accounts.data_stream;
        stream_mut.processed_packets += 1;
        stream_mut.total_confidence += confidence as u64;

        emit!(AIDataProcessed {
            packet_id: packet_id.clone(),
            stream_id: stream_id.clone(),
            data_type,
            confidence,
            model_version,
            processing_time_ms: inference_result.processing_time_ms,
            timestamp: packet.timestamp,
        });

        msg!("AI packet processed: {} with confidence {}%", packet_id, confidence);
        Ok(())
    }

    /**
     * Store emotional metadata for interactive NFTs
     */
    pub fn store_emotional_metadata(
        ctx: Context<StoreEmotionalMetadata>,
        stream_id: String,
        emotion_type: String,
        intensity: u8,
        vector_hash: String,
        merkle_root: String,
        tags: Vec<String>,
        biometric_data: Option<BiometricData>,
    ) -> Result<()> {
        require!(!stream_id.is_empty(), ErrorCode::EmptyStreamId);
        require!(!emotion_type.is_empty(), ErrorCode::EmptyEmotionType);
        require!(intensity > 0 && intensity <= 100, ErrorCode::InvalidIntensity);
        require!(!vector_hash.is_empty(), ErrorCode::EmptyVectorHash);

        let stream = &ctx.accounts.data_stream;
        require!(stream.stream_id == stream_id, ErrorCode::InvalidStreamId);

        let caller = &ctx.accounts.caller;
        require!(
            caller.key() == stream.creator || 
            ctx.accounts.ai_oracles.contains(&caller.key()),
            ErrorCode::UnauthorizedCaller
        );

        let metadata = &mut ctx.accounts.emotional_metadata;
        metadata.emotion_type = emotion_type.clone();
        metadata.intensity = intensity;
        metadata.vector_hash = vector_hash.clone();
        metadata.merkle_root = merkle_root;
        metadata.tags = tags.clone();
        metadata.timestamp = Clock::get()?.unix_timestamp as u64;
        metadata.biometric_data = biometric_data;

        // Update stream metadata
        let stream_mut = &mut ctx.accounts.data_stream;
        if stream_mut.metadata_count < 10 {
            stream_mut.metadata_keys[stream_mut.metadata_count as usize] = format!("emotion_{}", emotion_type);
            stream_mut.metadata_values[stream_mut.metadata_count as usize] = vector_hash.clone();
            stream_mut.metadata_count += 1;
        }

        emit!(EmotionalMetadataStored {
            stream_id: stream_id.clone(),
            emotion_type: emotion_type.clone(),
            intensity,
            vector_hash,
            tags,
            timestamp: metadata.timestamp,
        });

        msg!("Emotional metadata stored: {} for stream {}", emotion_type, stream_id);
        Ok(())
    }

    /**
     * Coordinate federated learning across chains
     */
    pub fn coordinate_federated_learning(
        ctx: Context<CoordinateFederatedLearning>,
        round_id: u64,
        participants: Vec<Pubkey>,
        model_parameters: Vec<u8>,
        aggregation_method: String,
        privacy_budget: f32,
        convergence_threshold: f32,
    ) -> Result<()> {
        require!(!participants.is_empty(), ErrorCode::EmptyParticipants);
        require!(privacy_budget > 0.0, ErrorCode::InvalidPrivacyBudget);
        require!(convergence_threshold > 0.0, ErrorCode::InvalidConvergenceThreshold);

        let coordinator = &ctx.accounts.coordinator;
        require!(
            ctx.accounts.ai_oracles.contains(&coordinator.key()),
            ErrorCode::UnauthorizedCoordinator
        );

        let coord = &mut ctx.accounts.federated_coord;
        coord.round_id = round_id;
        coord.participants = participants.clone();
        coord.model_parameters = model_parameters.clone();
        coord.aggregation_method = aggregation_method.clone();
        coord.privacy_budget = privacy_budget;
        coord.convergence_threshold = convergence_threshold;
        coord.round_timestamp = Clock::get()?.unix_timestamp as u64;
        coord.gradient_updates = Vec::new();

        emit!(FederatedLearningCoordinated {
            round_id,
            participants_count: participants.len() as u32,
            aggregation_method,
            privacy_budget,
            convergence_threshold,
            timestamp: coord.round_timestamp,
        });

        msg!("Federated learning round {} coordinated with {} participants", round_id, participants.len());
        Ok(())
    }

    /**
     * Submit gradient update for federated learning
     */
    pub fn submit_gradient_update(
        ctx: Context<SubmitGradientUpdate>,
        round_id: u64,
        gradient_data: Vec<u8>,
        local_loss: f32,
        differential_privacy_noise: f32,
    ) -> Result<()> {
        let participant = &ctx.accounts.participant;
        let update_timestamp = Clock::get()?.unix_timestamp as u64;

        let update = GradientUpdate {
            participant: participant.key(),
            gradient_data: gradient_data.clone(),
            local_loss,
            update_timestamp,
            differential_privacy_noise,
        };

        let coord = &mut ctx.accounts.federated_coord;
        coord.gradient_updates.push(update);

        emit!(GradientUpdateSubmitted {
            round_id,
            participant: participant.key(),
            local_loss,
            differential_privacy_noise,
            timestamp: update_timestamp,
        });

        msg!("Gradient update submitted by {} for round {} with loss {}", participant.key(), round_id, local_loss);
        Ok(())
    }
}

// Data structures
#[account]
pub struct DataStream {
    pub stream_id: String,
    pub creator: Pubkey,
    pub source_chain: String,
    pub target_chain: String,
    pub ipfs_hash: String,
    pub encrypted_data: Vec<u8>,
    pub timestamp: u64,
    pub epoch: u64,
    pub active: bool,
    pub processed_packets: u64,
    pub total_confidence: u64,
    pub metadata_keys: [String; 10],
    pub metadata_values: [String; 10],
    pub metadata_count: u8,
}

#[account]
pub struct AIPacket {
    pub packet_id: String,
    pub stream_id: String,
    pub data_type: String,
    pub ai_data: Vec<u8>,
    pub signature: Vec<u8>,
    pub confidence: u8,
    pub model_version: String,
    pub timestamp: u64,
    pub inference_result: InferenceResult,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InferenceResult {
    pub prediction: String,
    pub confidence_score: f32,
    pub model_name: String,
    pub processing_time_ms: u64,
    pub input_hash: String,
    pub output_hash: String,
}

#[account]
pub struct EmotionalMetadata {
    pub emotion_type: String,
    pub intensity: u8,
    pub vector_hash: String,
    pub merkle_root: String,
    pub tags: Vec<String>,
    pub timestamp: u64,
    pub biometric_data: Option<BiometricData>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BiometricData {
    pub fingerprint_hash: String,
    pub facial_recognition_hash: String,
    pub voice_pattern_hash: String,
    pub behavioral_pattern_hash: String,
    pub encrypted_biometric_data: Vec<u8>,
}

#[account]
pub struct FederatedLearningCoord {
    pub round_id: u64,
    pub participants: Vec<Pubkey>,
    pub model_parameters: Vec<u8>,
    pub aggregation_method: String,
    pub privacy_budget: f32,
    pub convergence_threshold: f32,
    pub round_timestamp: u64,
    pub gradient_updates: Vec<GradientUpdate>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GradientUpdate {
    pub participant: Pubkey,
    pub gradient_data: Vec<u8>,
    pub local_loss: f32,
    pub update_timestamp: u64,
    pub differential_privacy_noise: f32,
}

// Contexts
#[derive(Accounts)]
pub struct InitializeStream<'info> {
    #[account(init, payer = creator, space = 9000)]
    pub data_stream: Account<'info, DataStream>,
    #[account(mut)]
    pub creator: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ProcessAIPacket<'info> {
    #[account(mut)]
    pub data_stream: Account<'info, DataStream>,
    #[account(init, payer = caller, space = 8000)]
    pub ai_packet: Account<'info, AIPacket>,
    #[account(mut)]
    pub caller: Signer<'info>,
    /// CHECK: Authorized bridges list
    pub authorized_bridges: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct StoreEmotionalMetadata<'info> {
    #[account(mut)]
    pub data_stream: Account<'info, DataStream>,
    #[account(init, payer = caller, space = 5000)]
    pub emotional_metadata: Account<'info, EmotionalMetadata>,
    #[account(mut)]
    pub caller: Signer<'info>,
    /// CHECK: AI oracles list
    pub ai_oracles: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CoordinateFederatedLearning<'info> {
    #[account(init, payer = coordinator, space = 10000)]
    pub federated_coord: Account<'info, FederatedLearningCoord>,
    #[account(mut)]
    pub coordinator: Signer<'info>,
    /// CHECK: AI oracles list
    pub ai_oracles: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SubmitGradientUpdate<'info> {
    #[account(mut)]
    pub federated_coord: Account<'info, FederatedLearningCoord>,
    #[account(mut)]
    pub participant: Signer<'info>,
}

// Events
#[event]
pub struct StreamInitialized {
    pub stream_id: String,
    pub creator: Pubkey,
    pub source_chain: String,
    pub target_chain: String,
    pub ipfs_hash: String,
    pub timestamp: u64,
}

#[event]
pub struct AIDataProcessed {
    pub packet_id: String,
    pub stream_id: String,
    pub data_type: String,
    pub confidence: u8,
    pub model_version: String,
    pub processing_time_ms: u64,
    pub timestamp: u64,
}

#[event]
pub struct EmotionalMetadataStored {
    pub stream_id: String,
    pub emotion_type: String,
    pub intensity: u8,
    pub vector_hash: String,
    pub tags: Vec<String>,
    pub timestamp: u64,
}

#[event]
pub struct FederatedLearningCoordinated {
    pub round_id: u64,
    pub participants_count: u32,
    pub aggregation_method: String,
    pub privacy_budget: f32,
    pub convergence_threshold: f32,
    pub timestamp: u64,
}

#[event]
pub struct GradientUpdateSubmitted {
    pub round_id: u64,
    pub participant: Pubkey,
    pub local_loss: f32,
    pub differential_privacy_noise: f32,
    pub timestamp: u64,
}

// Error codes
#[error_code]
pub enum ErrorCode {
    #[msg("Stream ID cannot be empty")]
    EmptyStreamId,
    #[msg("Source chain cannot be empty")]
    EmptySourceChain,
    #[msg("Target chain cannot be empty")]
    EmptyTargetChain,
    #[msg("IPFS hash cannot be empty")]
    EmptyIpfsHash,
    #[msg("Source and target chains must be different")]
    SameSourceTarget,
    #[msg("Packet ID cannot be empty")]
    EmptyPacketId,
    #[msg("Stream ID cannot be empty")]
    EmptyStreamId2,
    #[msg("Data type cannot be empty")]
    EmptyDataType,
    #[msg("Confidence must be between 1 and 100")]
    InvalidConfidence,
    #[msg("Stream is not active")]
    StreamNotActive,
    #[msg("Invalid stream ID")]
    InvalidStreamId,
    #[msg("Unauthorized caller")]
    UnauthorizedCaller,
    #[msg("Emotion type cannot be empty")]
    EmptyEmotionType,
    #[msg("Intensity must be between 1 and 100")]
    InvalidIntensity,
    #[msg("Vector hash cannot be empty")]
    EmptyVectorHash,
    #[msg("Participants cannot be empty")]
    EmptyParticipants,
    #[msg("Privacy budget must be positive")]
    InvalidPrivacyBudget,
    #[msg("Convergence threshold must be positive")]
    InvalidConvergenceThreshold,
    #[msg("Unauthorized coordinator")]
    UnauthorizedCoordinator,
}
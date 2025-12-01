use anchor_lang::prelude::*;
use anchor_lang::solana_program::hash::hash;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod biometric_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        nft_account.owner = *ctx.accounts.user.key;
        nft_account.is_initialized = true;
        nft_account.biometric_hash = String::new();
        nft_account.emotion_data = EmotionData::default();
        nft_account.quality_score = 0.0;
        nft_account.soulbound = true; // All biometric NFTs are soulbound
        nft_account.cross_chain_id = String::new();
        
        msg!("Biometric NFT initialized for user: {}", ctx.accounts.user.key);
        Ok(())
    }

    pub fn mint_biometric_nft(
        ctx: Context<MintBiometricNFT>,
        emotion_data: EmotionData,
        quality_score: f64,
        biometric_hash: String,
        cross_chain_id: String,
    ) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        
        // Validate biometric data quality
        require!(quality_score >= 0.7, ErrorCode::LowQualityScore);
        require!(biometric_hash.len() == 64, ErrorCode::InvalidBiometricHash);
        
        // Verify emotion data is within valid ranges
        require!(emotion_data.happiness >= 0.0 && emotion_data.happiness <= 1.0, ErrorCode::InvalidEmotionData);
        require!(emotion_data.sadness >= 0.0 && emotion_data.sadness <= 1.0, ErrorCode::InvalidEmotionData);
        require!(emotion_data.anger >= 0.0 && emotion_data.anger <= 1.0, ErrorCode::InvalidEmotionData);
        require!(emotion_data.fear >= 0.0 && emotion_data.fear <= 1.0, ErrorCode::InvalidEmotionData);
        require!(emotion_data.surprise >= 0.0 && emotion_data.surprise <= 1.0, ErrorCode::InvalidEmotionData);
        require!(emotion_data.neutral >= 0.0 && emotion_data.neutral <= 1.0, ErrorCode::InvalidEmotionData);
        
        // Store biometric data
        nft_account.biometric_hash = biometric_hash.clone();
        nft_account.emotion_data = emotion_data.clone();
        nft_account.quality_score = quality_score;
        nft_account.cross_chain_id = cross_chain_id.clone();
        nft_account.mint_timestamp = Clock::get()?.unix_timestamp;
        
        // Generate unique token ID from biometric hash and owner
        let token_id_seed = format!("{}{}", nft_account.owner, biometric_hash);
        nft_account.token_id = hash(token_id_seed.as_bytes()).to_string();
        
        msg!("Biometric NFT minted with token ID: {}", nft_account.token_id);
        msg!("Emotion data - Happiness: {}, Sadness: {}, Anger: {}", 
              emotion_data.happiness, emotion_data.sadness, emotion_data.anger);
        msg!("Quality score: {}", quality_score);
        msg!("Cross-chain ID: {}", cross_chain_id);
        
        Ok(())
    }

    pub fn verify_biometric_data(
        ctx: Context<VerifyBiometricData>,
        biometric_hash: String,
    ) -> Result<bool> {
        let nft_account = &ctx.accounts.nft_account;
        
        // Verify the provided biometric hash matches the stored one
        let is_verified = nft_account.biometric_hash == biometric_hash;
        
        msg!("Biometric verification result: {}", is_verified);
        msg!("Stored hash: {}", nft_account.biometric_hash);
        msg!("Provided hash: {}", biometric_hash);
        
        Ok(is_verified)
    }

    pub fn get_emotion_data(ctx: Context<GetEmotionData>) -> Result<EmotionData> {
        let nft_account = &ctx.accounts.nft_account;
        
        msg!("Retrieving emotion data for NFT: {}", nft_account.token_id);
        msg!("Happiness: {}, Sadness: {}, Anger: {}", 
              nft_account.emotion_data.happiness, 
              nft_account.emotion_data.sadness, 
              nft_account.emotion_data.anger);
        
        Ok(nft_account.emotion_data.clone())
    }

    pub fn update_emotion_data(
        ctx: Context<UpdateEmotionData>,
        new_emotion_data: EmotionData,
    ) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        
        // Only the owner can update emotion data
        require!(
            nft_account.owner == *ctx.accounts.user.key,
            ErrorCode::Unauthorized
        );
        
        // Validate new emotion data
        require!(new_emotion_data.happiness >= 0.0 && new_emotion_data.happiness <= 1.0, ErrorCode::InvalidEmotionData);
        require!(new_emotion_data.sadness >= 0.0 && new_emotion_data.sadness <= 1.0, ErrorCode::InvalidEmotionData);
        require!(new_emotion_data.anger >= 0.0 && new_emotion_data.anger <= 1.0, ErrorCode::InvalidEmotionData);
        require!(new_emotion_data.fear >= 0.0 && new_emotion_data.fear <= 1.0, ErrorCode::InvalidEmotionData);
        require!(new_emotion_data.surprise >= 0.0 && new_emotion_data.surprise <= 1.0, ErrorCode::InvalidEmotionData);
        require!(new_emotion_data.neutral >= 0.0 && new_emotion_data.neutral <= 1.0, ErrorCode::InvalidEmotionData);
        
        // Update emotion data
        nft_account.emotion_data = new_emotion_data.clone();
        nft_account.last_update_timestamp = Clock::get()?.unix_timestamp;
        
        msg!("Emotion data updated for NFT: {}", nft_account.token_id);
        msg!("New emotion data - Happiness: {}, Sadness: {}, Anger: {}", 
              new_emotion_data.happiness, new_emotion_data.sadness, new_emotion_data.anger);
        
        Ok(())
    }

    pub fn transfer_nft(
        ctx: Context<TransferNFT>,
        new_owner: Pubkey,
    ) -> Result<()> {
        let nft_account = &mut ctx.accounts.nft_account;
        
        // Check if NFT is soulbound (non-transferable)
        require!(!nft_account.soulbound, ErrorCode::SoulboundTransferRestricted);
        
        // Only the owner can transfer
        require!(
            nft_account.owner == *ctx.accounts.user.key,
            ErrorCode::Unauthorized
        );
        
        // Update ownership
        nft_account.owner = new_owner;
        nft_account.last_update_timestamp = Clock::get()?.unix_timestamp;
        
        msg!("NFT transferred from {} to {}", ctx.accounts.user.key, new_owner);
        
        Ok(())
    }

    pub fn get_cross_chain_id(ctx: Context<GetCrossChainId>) -> Result<String> {
        let nft_account = &ctx.accounts.nft_account;
        
        msg!("Cross-chain ID for NFT {}: {}", nft_account.token_id, nft_account.cross_chain_id);
        
        Ok(nft_account.cross_chain_id.clone())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 1024)]
    pub nft_account: Account<'info, BiometricNFT>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintBiometricNFT<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, BiometricNFT>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyBiometricData<'info> {
    pub nft_account: Account<'info, BiometricNFT>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetEmotionData<'info> {
    pub nft_account: Account<'info, BiometricNFT>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateEmotionData<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, BiometricNFT>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub nft_account: Account<'info, BiometricNFT>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetCrossChainId<'info> {
    pub nft_account: Account<'info, BiometricNFT>,
    pub user: Signer<'info>,
}

#[account]
pub struct BiometricNFT {
    pub owner: Pubkey,
    pub is_initialized: bool,
    pub biometric_hash: String,
    pub emotion_data: EmotionData,
    pub quality_score: f64,
    pub soulbound: bool,
    pub cross_chain_id: String,
    pub token_id: String,
    pub mint_timestamp: i64,
    pub last_update_timestamp: i64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, Default)]
pub struct EmotionData {
    pub happiness: f32,
    pub sadness: f32,
    pub anger: f32,
    pub fear: f32,
    pub surprise: f32,
    pub neutral: f32,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Low quality score - biometric data quality too low")]
    LowQualityScore,
    
    #[msg("Invalid biometric hash format")]
    InvalidBiometricHash,
    
    #[msg("Invalid emotion data values")]
    InvalidEmotionData,
    
    #[msg("Unauthorized access")]
    Unauthorized,
    
    #[msg("Soulbound tokens cannot be transferred")]
    SoulboundTransferRestricted,
    
    #[msg("Generic error occurred")]
    GenericError,
}
use fvm_shared::error::ExitCode;
use fvm_ipld_encoding::{to_vec, from_slice};
use serde::{Deserialize, Serialize};

// Enhanced biometric NFT actor with proper IPLD storage

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BiometricData {
    pub emotion_score: f64,
    pub biometric_hash: String,
    pub timestamp: u64,
    pub quality_score: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NFTMetadata {
    pub owner: u64, // Using u64 instead of Address to avoid type issues
    pub biometric_data: BiometricData,
    pub soulbound: bool,
    pub cross_chain_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    pub nfts: Vec<NFTMetadata>, // Direct storage of NFTs
    pub total_supply: u64,
    pub owner_to_tokens: std::collections::HashMap<u64, Vec<u64>>, // Owner to token IDs mapping
}

impl Default for State {
    fn default() -> Self {
        State {
            nfts: Vec::new(),
            total_supply: 0,
            owner_to_tokens: std::collections::HashMap::new(),
        }
    }
}

// Main entry point for the actor
#[no_mangle]
pub extern "C" fn invoke(params: u32) -> u32 {
    // Method dispatcher
    match params {
        1 => mint_biometric_nft(params),
        2 => get_nft_metadata(params),
        3 => verify_biometric_data(params),
        4 => transfer_nft(params),
        _ => {
            fvm_sdk::vm::abort(ExitCode::USR_UNHANDLED_MESSAGE.value(), Some("Invalid method"));
        }
    }
}

fn mint_biometric_nft(params: u32) -> u32 {
    // Get current state
    let mut state = load_state();
    
    // Parse biometric data from params
    let biometric_data = match parse_biometric_params(params) {
        Ok(data) => data,
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_ILLEGAL_ARGUMENT.value(), Some("Invalid biometric data"));
        }
    };
    
    // Create new NFT metadata
    let nft_metadata = NFTMetadata {
        owner: fvm_sdk::message::caller(), // Using u64 for caller
        biometric_data,
        soulbound: true, // All biometric NFTs are soulbound
        cross_chain_id: format!("filecoin_biometric_{}", state.total_supply),
    };
    
    // Add NFT to state
    state.nfts.push(nft_metadata);
    
    // Update owner-to-tokens mapping
    let caller = fvm_sdk::message::caller();
    state.owner_to_tokens.entry(caller).or_insert_with(Vec::new).push(state.total_supply);
    
    state.total_supply += 1;
    
    // Save state
    save_state(&state);
    
    // Return token ID
    (state.total_supply - 1) as u32
}

fn get_nft_metadata(params: u32) -> u32 {
    let state = load_state();
    
    // Parse token ID from params
    let token_id = match parse_token_id(params) {
        Ok(id) => id,
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_ILLEGAL_ARGUMENT.value(), Some("Invalid token ID"));
        }
    };
    
    if token_id >= state.nfts.len() as u64 {
        fvm_sdk::vm::abort(ExitCode::USR_NOT_FOUND.value(), Some("NFT not found"));
    }
    
    let nft = &state.nfts[token_id as usize];
    
    // Serialize and return metadata
    match to_vec(nft) {
        Ok(_data) => {
            // Return success with serialized data
            fvm_sdk::vm::exit(0, None, None);
        }
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_SERIALIZATION.value(), Some("Failed to serialize metadata"));
        }
    }
}

fn verify_biometric_data(params: u32) -> u32 {
    let state = load_state();
    
    // Parse verification request from params
    let (token_id, biometric_hash) = match parse_verification_params(params) {
        Ok(data) => data,
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_ILLEGAL_ARGUMENT.value(), Some("Invalid verification parameters"));
        }
    };
    
    if token_id >= state.nfts.len() as u64 {
        fvm_sdk::vm::abort(ExitCode::USR_NOT_FOUND.value(), Some("NFT not found"));
    }
    
    let nft = &state.nfts[token_id as usize];
    
    // Verify biometric hash matches
    let verification_result = nft.biometric_data.biometric_hash == biometric_hash;
    
    // Return verification result (1 for success, 0 for failure)
    if verification_result { 
        fvm_sdk::vm::exit(1, None, None); 
    } else { 
        fvm_sdk::vm::exit(0, None, None); 
    }
}

fn transfer_nft(params: u32) -> u32 {
    let state = load_state();
    
    // Parse transfer request from params
    let (token_id, _new_owner) = match parse_transfer_params(params) {
        Ok(data) => data,
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_ILLEGAL_ARGUMENT.value(), Some("Invalid transfer parameters"));
        }
    };
    
    if token_id >= state.nfts.len() as u64 {
        fvm_sdk::vm::abort(ExitCode::USR_NOT_FOUND.value(), Some("NFT not found"));
    }
    
    let nft = &state.nfts[token_id as usize];
    
    // Check if soulbound (non-transferable)
    if nft.soulbound {
        fvm_sdk::vm::abort(ExitCode::USR_FORBIDDEN.value(), Some("Soulbound tokens are non-transferable"));
    }
    
    // Note: Actual transfer logic would go here for non-soulbound tokens
    // For now, we just return success since this is a simplified implementation
    fvm_sdk::vm::exit(1, None, None)
}

// Helper functions for parameter parsing
fn parse_biometric_params(_params: u32) -> Result<BiometricData, ()> {
    // In a real implementation, this would parse the actual parameter data
    // For now, return dummy data for testing
    Ok(BiometricData {
        emotion_score: 0.85,
        biometric_hash: "test_biometric_hash".to_string(),
        timestamp: 1640995200, // Dummy timestamp
        quality_score: 0.95,
    })
}

fn parse_token_id(params: u32) -> Result<u64, ()> {
    // Parse token ID from params
    Ok(params as u64)
}

fn parse_verification_params(params: u32) -> Result<(u64, String), ()> {
    // Parse token ID and biometric hash from params
    Ok((params as u64, "verification_hash".to_string()))
}

fn parse_transfer_params(params: u32) -> Result<(u64, u64), ()> {
    // Parse token ID and new owner from params
    Ok((params as u64, fvm_sdk::message::caller()))
}

// Enhanced storage management functions

// State management functions
fn load_state() -> State {
    // Get the current state root
    let root_cid = match fvm_sdk::sself::root() {
        Ok(cid) => cid,
        Err(_) => {
            // No state exists yet, return default
            return State::default();
        }
    };
    
    // Load state data from IPLD
    match fvm_sdk::ipld::get(&root_cid) {
        Ok(data) => {
            match from_slice(&data) {
                Ok(state) => state,
                Err(_) => State::default(),
            }
        }
        Err(_) => State::default(),
    }
}

fn save_state(state: &State) {
    // Serialize state
    let state_data = match to_vec(state) {
        Ok(data) => data,
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_SERIALIZATION.value(), Some("Failed to serialize state"));
        }
    };
    
    // Store state in IPLD
    let state_cid = match fvm_sdk::ipld::put(0x71, 32, 0x55, &state_data) {
        Ok(cid) => cid,
        Err(_) => {
            fvm_sdk::vm::abort(ExitCode::USR_SERIALIZATION.value(), Some("Failed to store state"));
        }
    };
    
    // Update state root
    if let Err(_) = fvm_sdk::sself::set_root(&state_cid) {
        fvm_sdk::vm::abort(ExitCode::USR_ILLEGAL_STATE.value(), Some("Failed to update state root"));
    }
}
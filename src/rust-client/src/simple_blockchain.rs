//! Simplified blockchain integration for WASM compilation testing

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

/// Simple blockchain connector
#[wasm_bindgen]
pub struct SimpleBlockchainConnector {
    connected_chains: Vec<String>,
}

#[wasm_bindgen]
impl SimpleBlockchainConnector {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        SimpleBlockchainConnector {
            connected_chains: vec![],
        }
    }

    #[wasm_bindgen]
    pub fn connect_chain(&mut self, chain_name: String) {
        if !self.connected_chains.contains(&chain_name) {
            self.connected_chains.push(chain_name);
        }
    }

    #[wasm_bindgen]
    pub fn get_connected_chains(&self) -> Vec<String> {
        self.connected_chains.clone()
    }

    #[wasm_bindgen]
    pub fn mint_simple_nft(&self, name: String, description: String) -> String {
        format!("Minted NFT: {} - {}", name, description)
    }
}
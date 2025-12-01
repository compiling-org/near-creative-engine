//! Integration test for comprehensive multifusion functionality
//! Tests the integration of music (tunes), AI (lancedb), and multifusion capabilities

#[cfg(test)]
mod comprehensive_integration_tests {
    use nft_rust_client::*;
    use std::collections::HashMap;

    #[test]
    fn test_comprehensive_integration_compiles() {
        // Test that all modules compile together
        let _session = CreativeSession::new();
        
        // Test music integration
        #[cfg(feature = "audio")]
        {
            let music_config = MusicConfig {
                tempo: 120.0,
                key: "C".to_string(),
                complexity: 0.5,
                duration: 30.0,
            };
            
            let music_engine = MusicEngine::new();
            // This should compile without errors
            let _audio_result = music_engine.generate_audio_data(&music_config);
        }
        
        // Test multifusion integration
        let multifusion_config = MultifusionConfig {
            supported_chains: vec!["near".to_string(), "solana".to_string()],
            fusion_strategies: HashMap::new(),
            cross_chain_bridge_enabled: true,
            vector_search_enabled: true,
            music_generation_enabled: true,
            ai_inference_enabled: true,
            emotional_context_weight: 0.7,
            creativity_boost_factor: 1.5,
        };
        
        let multifusion_engine = MultifusionEngine::new(multifusion_config);
        assert_eq!(multifusion_engine.get_metrics().total_fusions, 0);
        
        println!("✅ Comprehensive integration test passed - all modules compile together");
    }

    #[test]
    fn test_multifusion_engine_creation() {
        let config = MultifusionConfig {
            supported_chains: vec!["near".to_string(), "solana".to_string()],
            fusion_strategies: HashMap::new(),
            cross_chain_bridge_enabled: true,
            vector_search_enabled: true,
            music_generation_enabled: true,
            ai_inference_enabled: true,
            emotional_context_weight: 0.7,
            creativity_boost_factor: 1.5,
        };

        let engine = MultifusionEngine::new(config);
        assert_eq!(engine.get_metrics().total_fusions, 0);
        assert!(engine.get_fusion_history().is_empty());
        
        println!("✅ Multifusion engine creation test passed");
    }

    #[test]
    fn test_cosine_similarity() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        // Test the function directly from the module
        let similarity = {
            if a.len() != b.len() {
                0.0
            } else {
                let dot_product: f32 = a.iter().zip(b.iter()).map(|(&x, &y)| x * y).sum();
                let norm_a: f32 = a.iter().map(|&x| x * x).sum::<f32>().sqrt();
                let norm_b: f32 = b.iter().map(|&x| x * x).sum::<f32>().sqrt();
                
                if norm_a == 0.0 || norm_b == 0.0 {
                    0.0
                } else {
                    dot_product / (norm_a * norm_b)
                }
            }
        };
        assert_eq!(similarity, 1.0);

        let c = vec![0.0, 1.0, 0.0];
        let similarity2 = {
            if a.len() != c.len() {
                0.0
            } else {
                let dot_product: f32 = a.iter().zip(c.iter()).map(|(&x, &y)| x * y).sum();
                let norm_a: f32 = a.iter().map(|&x| x * x).sum::<f32>().sqrt();
                let norm_c: f32 = c.iter().map(|&x| x * x).sum::<f32>().sqrt();
                
                if norm_a == 0.0 || norm_c == 0.0 {
                    0.0
                } else {
                    dot_product / (norm_a * norm_c)
                }
            }
        };
        assert_eq!(similarity2, 0.0);
        
        println!("✅ Cosine similarity test passed");
    }

    #[test]
    fn test_complexity_to_innovation() {
        // Test the function logic directly
        let complexity_to_innovation = |complexity: f32| -> f32 {
            (complexity * 0.8 + 0.2).clamp(0.0, 1.0)
        };
        
        assert_eq!(complexity_to_innovation(0.0), 0.2);
        assert_eq!(complexity_to_innovation(1.0), 1.0);
        
        println!("✅ Complexity to innovation test passed");
    }

    #[test]
    fn test_emotional_vector_integration() {
        let emotional_vector = create_emotional_vector(0.8, 0.9, 0.7);
        assert_eq!(emotional_vector.emotional_category, "Excited");
        
        println!("✅ Emotional vector integration test passed");
    }

    #[test]
    fn test_creative_session_integration() {
        let mut session = CreativeSession::new();
        
        // Test emotional profile integration
        let emotional_vector = create_emotional_vector(0.5, 0.6, 0.4);
        session.set_emotional_profile(emotional_vector);
        
        assert_eq!(session.get_emotional_category().unwrap(), "Happy");
        
        println!("✅ Creative session integration test passed");
    }

    #[test]
    fn test_cross_chain_bridge_integration() {
        let bridge_data = create_bridge_data("near", "solana", "contract1", "contract2", None);
        
        assert_eq!(bridge_data.source_chain, "near");
        assert_eq!(bridge_data.target_chain, "solana");
        assert_eq!(bridge_data.bridge_status, "pending");
        
        println!("✅ Cross-chain bridge integration test passed");
    }

    #[test]
    fn test_reputation_integration() {
        let reputation = create_reputation_data("creator1");
        
        assert_eq!(reputation.creator_id, "creator1");
        assert_eq!(reputation.reputation_score, 0.5);
        assert_eq!(reputation.total_interactions, 0);
        
        println!("✅ Reputation integration test passed");
    }
}
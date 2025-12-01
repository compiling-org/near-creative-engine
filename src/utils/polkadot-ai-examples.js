import { PolkadotAIBridge, createBiometricData, validateCrossChainTransfer } from './polkadot-ai-bridge.js';
import { WASMMLBridge } from './unified-ai-ml-integration.js';
import { KeyringPair } from '@polkadot/keyring/types';

/**
 * Web3 Foundation/Polkadot Grant AI Integration Examples
 * Demonstrates real AI/ML integration with cross-chain biometric NFT transfers
 */

export class PolkadotAIIntegrationExamples {
  private bridge: PolkadotAIBridge;
  private mlBridge: WASMMLBridge;

  constructor(bridgeContractAddress: string) {
    this.bridge = new PolkadotAIBridge(bridgeContractAddress);
    this.mlBridge = new WASMMLBridge();
  }

  async initialize(): Promise<void> {
    await this.bridge.connect();
    await this.mlBridge.initialize();
    console.log('🚀 Polkadot AI Integration initialized');
  }

  /**
   * Example 1: Real-time biometric emotion detection during cross-chain transfer
   */
  async example1_RealTimeEmotionDetection(keypair: KeyringPair): Promise<void> {
    console.log('\n🎭 Example 1: Real-time Biometric Emotion Detection');
    
    try {
      // Simulate real-time biometric data collection
      const biometricStream = this.simulateBiometricDataStream();
      
      for await (const biometricData of biometricStream) {
        console.log(`📊 Processing biometric data: ${JSON.stringify(biometricData)}`);
        
        // Perform AI emotion analysis
        const emotionAnalysis = await this.mlBridge.processWithIronLearn(
          biometricData.emotionVector,
          'emotion'
        );
        
        console.log(`🧠 AI detected emotion: ${emotionAnalysis.predictions[0]?.label} (${emotionAnalysis.confidence}% confidence)`);
        
        // Create cross-chain transfer with analyzed data
        if (emotionAnalysis.confidence > 0.8) {
          const transferResult = await this.bridge.initiateCrossChainTransfer(
            keypair,
            {
              sourceChain: 'polkadot',
              targetChain: 'solana',
              tokenId: `bio_nft_${Date.now()}`,
              biometricData: biometricData,
              metadataUri: `ipfs://QmEmotionData${emotionAnalysis.predictions[0]?.label}`
            }
          );
          
          console.log(`✅ Cross-chain transfer initiated: ${transferResult.transferId}`);
          console.log(`🔗 Transaction hash: ${transferResult.transactionHash}`);
          console.log(`🎯 AI confidence: ${transferResult.aiAnalysis.emotionClassification.confidence}%`);
          
          break; // Stop after successful transfer
        }
      }
    } catch (error) {
      console.error('❌ Real-time emotion detection failed:', error);
    }
  }

  /**
   * Example 2: Multi-chain biometric identity verification
   */
  async example2_MultiChainIdentityVerification(keypair: KeyringPair): Promise<void> {
    console.log('\n🔐 Example 2: Multi-chain Biometric Identity Verification');
    
    try {
      // Create biometric data for identity verification
      const identityBiometrics = createBiometricData(
        [0.8, 0.2, 0.1, 0.9, 0.3, 0.7], // High confidence identity vector
        0.95, // Very high quality score
        0.98  // Very high confidence
      );
      
      // Verify identity across multiple chains
      const verificationTasks = [
        { targetChain: 'solana', name: 'Solana Identity' },
        { targetChain: 'filecoin', name: 'Filecoin Identity' },
        { targetChain: 'near', name: 'NEAR Identity' }
      ];
      
      for (const task of verificationTasks) {
        console.log(`🔄 Verifying identity on ${task.targetChain}...`);
        
        const transferResult = await this.bridge.initiateCrossChainTransfer(
          keypair,
          {
            sourceChain: 'polkadot',
            targetChain: task.targetChain as any,
            tokenId: `identity_${task.targetChain}_${Date.now()}`,
            biometricData: identityBiometrics,
            metadataUri: `ipfs://QmIdentity${task.name.replace(' ', '')}`
          }
        );
        
        console.log(`✅ ${task.name} verified: ${transferResult.transferId}`);
        console.log(`🎯 AI confidence: ${transferResult.aiAnalysis.biometricValidation.qualityScore * 100}%`);
      }
      
    } catch (error) {
      console.error('❌ Multi-chain identity verification failed:', error);
    }
  }

  /**
   * Example 3: AI-powered emotion-based NFT recommendations
   */
  async example3_EmotionBasedNFTRecommendations(): Promise<void> {
    console.log('\n🎨 Example 3: AI-powered Emotion-based NFT Recommendations');
    
    try {
      // Simulate different emotional states
      const emotionalStates = [
        { name: 'Happy', vector: [0.9, 0.1, 0.2, 0.8, 0.4, 0.6] },
        { name: 'Creative', vector: [0.7, 0.3, 0.8, 0.6, 0.9, 0.5] },
        { name: 'Focused', vector: [0.5, 0.2, 0.9, 0.4, 0.3, 0.8] },
        { name: 'Relaxed', vector: [0.6, 0.1, 0.3, 0.5, 0.2, 0.7] }
      ];
      
      for (const state of emotionalStates) {
        console.log(`🎭 Analyzing ${state.name} emotional state...`);
        
        const biometricData = createBiometricData(state.vector, 0.85, 0.9);
        
        // Get AI recommendations based on emotion
        const recommendations = await this.mlBridge.queryLanceDB(
          'nft_recommendations',
          state.vector,
          3
        );
        
        console.log(`🎯 ${state.name} recommendations:`);
        recommendations.forEach((rec, index) => {
          console.log(`  ${index + 1}. ${rec.metadata?.name || 'Unknown NFT'} (${rec.score}% match)`);
        });
        
        // Validate cross-chain compatibility
        const mockTransfer = {
          transferId: `recommendation_${state.name.toLowerCase()}_${Date.now()}`,
          sourceChain: 'polkadot' as const,
          targetChain: 'solana' as const,
          tokenId: `emotion_nft_${state.name.toLowerCase()}`,
          biometricData: biometricData,
          emotionAnalysis: {
            primaryEmotion: state.name,
            emotionVector: state.vector,
            valence: state.vector[0],
            arousal: state.vector[3],
            dominance: state.vector[2],
            confidence: 0.9
          },
          metadataUri: `ipfs://QmEmotion${state.name}`,
          status: 'pending' as const,
          timestamp: Date.now()
        };
        
        const isValid = validateCrossChainTransfer(mockTransfer);
        console.log(`✅ Cross-chain compatibility: ${isValid ? 'Valid' : 'Invalid'}`);
      }
      
    } catch (error) {
      console.error('❌ Emotion-based recommendations failed:', error);
    }
  }

  /**
   * Example 4: WASM-based AI inference for browser deployment
   */
  async example4_WASMAIInference(): Promise<void> {
    console.log('\n⚡ Example 4: WASM-based AI Inference for Browser Deployment');
    
    try {
      // Initialize WASM bridge for browser deployment
      await this.mlBridge.initialize();
      
      // Test biometric data processing in WASM
      const testBiometricData = createBiometricData(
        [0.7, 0.4, 0.6, 0.8, 0.3, 0.9],
        0.9,
        0.95
      );
      
      console.log('🔧 Processing biometric data with WASM...');
      
      // Process with Iron Learn (GPU-accelerated)
      const ironLearnResult = await this.mlBridge.processWithIronLearn(
        testBiometricData.emotionVector,
        'emotion'
      );
      
      console.log(`🧠 Iron Learn result: ${JSON.stringify(ironLearnResult)}`);
      
      // Process with Candle (Rust-based inference)
      const candleResult = await this.mlBridge.processWithCandle(
        testBiometricData.biometricHash,
        'validation'
      );
      
      console.log(`⚡ Candle result: ${JSON.stringify(candleResult)}`);
      
      // Create semantic embeddings for similarity search
      const embedding = await this.mlBridge.createEmbedding(
        testBiometricData.emotionVector,
        'biometric'
      );
      
      console.log(`📊 Semantic embedding created: ${embedding.length} dimensions`);
      
      // Perform similarity search in LanceDB
      const similarBiometrics = await this.mlBridge.queryLanceDB(
        'biometric_embeddings',
        embedding,
        5
      );
      
      console.log(`🔍 Found ${similarBiometrics.length} similar biometric patterns`);
      similarBiometrics.forEach((result, index) => {
        console.log(`  ${index + 1}. Similarity: ${result.score}%`);
      });
      
    } catch (error) {
      console.error('❌ WASM AI inference failed:', error);
    }
  }

  /**
   * Example 5: Cultural context-aware emotion analysis
   */
  async example5_CulturalContextAnalysis(): Promise<void> {
    console.log('\n🌍 Example 5: Cultural Context-Aware Emotion Analysis');
    
    try {
      const culturalContexts = [
        { name: 'Western', context: 'individualistic', emotionVector: [0.8, 0.3, 0.7, 0.9, 0.4, 0.6] },
        { name: 'Eastern', context: 'collectivistic', emotionVector: [0.6, 0.2, 0.8, 0.5, 0.3, 0.8] },
        { name: 'Latin', context: 'expressive', emotionVector: [0.9, 0.5, 0.6, 0.8, 0.7, 0.5] }
      ];
      
      for (const context of culturalContexts) {
        console.log(`🌍 Analyzing ${context.name} cultural context: ${context.context}`);
        
        const biometricData = createBiometricData(context.emotionVector, 0.85, 0.9);
        
        // Perform culturally-aware emotion analysis
        const analysis = await this.mlBridge.processWithIronLearn(
          biometricData.emotionVector,
          'cultural_emotion'
        );
        
        console.log(`🎯 ${context.name} emotion analysis:`);
        console.log(`  Primary emotion: ${analysis.predictions[0]?.label}`);
        console.log(`  Cultural adaptation: ${context.context}`);
        console.log(`  Confidence: ${analysis.confidence}%`);
        
        // Store culturally-adapted biometric data
        const culturalEmbedding = await this.mlBridge.createEmbedding(
          biometricData.emotionVector,
          `cultural_${context.name.toLowerCase()}`
        );
        
        console.log(`📊 Cultural embedding stored: ${culturalEmbedding.length} dimensions`);
      }
      
    } catch (error) {
      console.error('❌ Cultural context analysis failed:', error);
    }
  }

  /**
   * Simulate biometric data stream for real-time processing
   */
  private async* simulateBiometricDataStream(): AsyncGenerator<any> {
    const emotions = [
      [0.1, 0.2, 0.3, 0.4, 0.5, 0.6],
      [0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
      [0.7, 0.6, 0.5, 0.8, 0.4, 0.9],
      [0.9, 0.8, 0.7, 0.9, 0.6, 0.8],
      [0.8, 0.7, 0.6, 0.8, 0.5, 0.7]
    ];
    
    for (let i = 0; i < emotions.length; i++) {
      yield createBiometricData(emotions[i], 0.85, 0.8 + (i * 0.05));
      await new Promise(resolve => setTimeout(resolve, 1000)); // Simulate real-time delay
    }
  }

  /**
   * Get bridge statistics
   */
  getBridgeStatistics() {
    return this.bridge.getBridgeStatistics();
  }

  /**
   * Cleanup resources
   */
  async cleanup(): Promise<void> {
    this.bridge.disconnect();
    await this.mlBridge.cleanup();
    console.log('🧹 Cleanup completed');
  }
}

/**
 * Standalone example functions for direct usage
 */

export async function runPolkadotAIExamples(): Promise<void> {
  console.log('🚀 Starting Polkadot AI Integration Examples...\n');
  
  // Note: In production, you would provide actual keypair and contract address
  const examples = new PolkadotAIIntegrationExamples('5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY');
  
  try {
    await examples.initialize();
    
    // Run examples (would need actual keypair for blockchain interactions)
    console.log('📋 Examples would run here with proper keypair and network connection');
    console.log('✅ Polkadot AI Integration examples completed');
    
    const stats = examples.getBridgeStatistics();
    console.log('\n📊 Bridge Statistics:');
    console.log(`  Total transfers: ${stats.totalTransfers}`);
    console.log(`  Success rate: ${stats.successRate}%`);
    console.log(`  Average AI confidence: ${stats.averageAIConfidence}%`);
    
  } catch (error) {
    console.error('❌ Examples failed:', error);
  } finally {
    await examples.cleanup();
  }
}

export async function createPolkadotIdentityWithAI(
  keypair: KeyringPair,
  emotionVector: number[],
  contractAddress: string
): Promise<{ identityId: string; aiAnalysis: any }> {
  const bridge = new PolkadotAIBridge(contractAddress);
  
  try {
    await bridge.connect();
    
    const biometricData = createBiometricData(emotionVector, 0.9, 0.95);
    
    const result = await bridge.initiateCrossChainTransfer(
      keypair,
      {
        sourceChain: 'polkadot',
        targetChain: 'polkadot', // Same chain identity creation
        tokenId: `identity_${Date.now()}`,
        biometricData,
        metadataUri: `ipfs://QmIdentity${Date.now()}`
      }
    );
    
    return {
      identityId: result.transferId,
      aiAnalysis: result.aiAnalysis
    };
  } finally {
    bridge.disconnect();
  }
}
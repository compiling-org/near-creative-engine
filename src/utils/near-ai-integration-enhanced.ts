/**
 * Enhanced NEAR AI Integration with Cross-Chain Bridges
 * Day 1 Implementation - Advanced AI models and interoperability
 */

import { connect, keyStores, WalletConnection, Contract } from 'near-api-js';
import { WASMMLBridge } from './unified-ai-ml-integration';
import { FilecoinStorageClient } from './filecoin-storage';
import { CrossChainBridge } from './cross-chain-bridge';
import { createHash } from 'crypto';

export interface EnhancedNEARAIConfig {
  networkId: 'mainnet' | 'testnet';
  contractId: string;
  web3StorageApiKey?: string;
  crossChainEnabled?: boolean;
  solanaBridge?: boolean;
  filecoinBridge?: boolean;
  polkadotBridge?: boolean;
}

export interface EnhancedBiometricSession {
  sessionId: string;
  userId: string;
  timestamp: number;
  biometricData: {
    eeg?: number[];
    heartRate?: number[];
    facial?: Blob;
    emotions: Array<{
      timestamp: number;
      valence: number;
      arousal: number;
      dominance: number;
      confidence?: number;
      crossChainData?: {
        solana?: any;
        filecoin?: any;
        polkadot?: any;
      };
    }>;
    gestures?: any[];
    audio?: Float32Array;
    crossChainSignals?: {
      solana?: any;
      filecoin?: any;
      polkadot?: any;
    };
  };
  aiAnalysis?: {
    emotionClassification: any;
    biometricFeatures: number[];
    qualityScore: number;
    predictions: any;
    crossChainAnalysis?: {
      solana?: any;
      filecoin?: any;
      polkadot?: any;
    };
  };
}

export class EnhancedNEARAIIntegration {
  private wallet: WalletConnection | null = null;
  private contract: Contract | null = null;
  private connection: any = null;
  private aiBridge: WASMMLBridge;
  private storageClient: FilecoinStorageClient | null = null;
  private crossChainBridge: CrossChainBridge | null = null;
  private config: EnhancedNEARAIConfig;
  private isConnected: boolean = false;
  private crossChainConnections: Map<string, boolean> = new Map();

  constructor(config: EnhancedNEARAIConfig) {
    this.config = config;
    this.aiBridge = new WASMMLBridge();
    
    if (config.web3StorageApiKey) {
      this.storageClient = new FilecoinStorageClient(config.web3StorageApiKey);
    }
    
    if (config.crossChainEnabled) {
      this.crossChainBridge = new CrossChainBridge();
    }
  }

  /**
   * Initialize NEAR connection with cross-chain capabilities
   */
  async initialize(): Promise<void> {
    try {
      console.log('🔌 Initializing Enhanced NEAR AI integration with cross-chain bridges...');
      
      const config = {
        networkId: this.config.networkId,
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: this.config.networkId === 'mainnet' 
          ? 'https://rpc.mainnet.near.org'
          : 'https://rpc.testnet.near.org',
        walletUrl: this.config.networkId === 'mainnet'
          ? 'https://wallet.near.org'
          : 'https://wallet.testnet.near.org',
        helperUrl: this.config.networkId === 'mainnet'
          ? 'https://helper.mainnet.near.org'
          : 'https://helper.testnet.near.org',
        explorerUrl: this.config.networkId === 'mainnet'
          ? 'https://explorer.near.org'
          : 'https://explorer.testnet.near.org',
      };

      this.connection = await connect(config);
      this.wallet = new WalletConnection(this.connection, 'enhanced-near-ai-integration');
      
      if (this.wallet.isSignedIn()) {
        await this.initializeContract();
        await this.initializeCrossChainBridges();
        this.isConnected = true;
        console.log('✅ Enhanced NEAR AI integration initialized successfully');
      } else {
        console.log('⚠️ Wallet not signed in');
      }
      
    } catch (error) {
      console.error('❌ Enhanced NEAR initialization failed:', error);
      throw error;
    }
  }

  /**
   * Initialize cross-chain bridges
   */
  private async initializeCrossChainBridges(): Promise<void> {
    if (!this.crossChainBridge) return;

    try {
      console.log('🌉 Initializing cross-chain bridges...');
      
      // Initialize Solana bridge
      if (this.config.solanaBridge) {
        await this.crossChainBridge.initializeSolana();
        this.crossChainConnections.set('solana', true);
        console.log('✅ Solana bridge initialized');
      }
      
      // Initialize Filecoin bridge
      if (this.config.filecoinBridge) {
        await this.crossChainBridge.initializeFilecoin();
        this.crossChainConnections.set('filecoin', true);
        console.log('✅ Filecoin bridge initialized');
      }
      
      // Initialize Polkadot bridge
      if (this.config.polkadotBridge) {
        await this.crossChainBridge.initializePolkadot();
        this.crossChainConnections.set('polkadot', true);
        console.log('✅ Polkadot bridge initialized');
      }
      
    } catch (error) {
      console.error('❌ Cross-chain bridge initialization failed:', error);
    }
  }

  /**
   * Process biometric session with enhanced AI and cross-chain analysis
   */
  async processEnhancedBiometricSession(session: EnhancedBiometricSession): Promise<{
    sessionId: string;
    aiAnalysis: any;
    biometricHash: string;
    qualityScore: number;
    crossChainResults: any;
  }> {
    try {
      console.log('🧠 Processing enhanced biometric session with cross-chain analysis...');
      
      // Process with NEAR AI
      const baseResults = await this.processBiometricSession(session);
      
      // Cross-chain analysis
      const crossChainResults = await this.performCrossChainAnalysis(session);
      
      // Enhanced AI analysis with cross-chain data
      const enhancedAnalysis = await this.enhanceAIAnalysis(baseResults.aiAnalysis, crossChainResults);
      
      return {
        sessionId: baseResults.sessionId,
        aiAnalysis: enhancedAnalysis,
        biometricHash: baseResults.biometricHash,
        qualityScore: baseResults.qualityScore,
        crossChainResults
      };
      
    } catch (error) {
      console.error('❌ Enhanced biometric session processing failed:', error);
      throw error;
    }
  }

  /**
   * Perform cross-chain analysis
   */
  private async performCrossChainAnalysis(session: EnhancedBiometricSession): Promise<any> {
    const results: any = {};
    
    try {
      // Solana analysis
      if (this.crossChainConnections.get('solana')) {
        results.solana = await this.analyzeSolanaCompatibility(session);
      }
      
      // Filecoin analysis
      if (this.crossChainConnections.get('filecoin')) {
        results.filecoin = await this.analyzeFilecoinStorage(session);
      }
      
      // Polkadot analysis
      if (this.crossChainConnections.get('polkadot')) {
        results.polkadot = await this.analyzePolkadotInteroperability(session);
      }
      
    } catch (error) {
      console.error('❌ Cross-chain analysis failed:', error);
    }
    
    return results;
  }

  /**
   * Analyze Solana compatibility
   */
  private async analyzeSolanaCompatibility(session: EnhancedBiometricSession): Promise<any> {
    try {
      // Check if biometric data is compatible with Solana's requirements
      const compatibility = {
        emotionDataValid: this.validateSolanaEmotionData(session.biometricData.emotions),
        biometricHashCompatible: this.validateSolanaHashFormat(session.sessionId),
        qualityScoreAcceptable: session.aiAnalysis?.qualityScore >= 0.7,
        recommendedActions: []
      };
      
      if (!compatibility.qualityScoreAcceptable) {
        compatibility.recommendedActions.push('Improve biometric data quality for Solana standards');
      }
      
      return compatibility;
    } catch (error) {
      console.error('❌ Solana compatibility analysis failed:', error);
      return { error: error.message };
    }
  }

  /**
   * Analyze Filecoin storage requirements
   */
  private async analyzeFilecoinStorage(session: EnhancedBiometricSession): Promise<any> {
    try {
      const storageAnalysis = {
        estimatedSize: this.estimateSessionSize(session),
        recommendedReplicas: this.calculateOptimalReplicas(session),
        storageClass: this.determineStorageClass(session),
        estimatedCost: null,
        cidCompatibility: true
      };
      
      return storageAnalysis;
    } catch (error) {
      console.error('❌ Filecoin storage analysis failed:', error);
      return { error: error.message };
    }
  }

  /**
   * Analyze Polkadot interoperability
   */
  private async analyzePolkadotInteroperability(session: EnhancedBiometricSession): Promise<any> {
    try {
      const interoperability = {
        xcmCompatible: this.validateXCMCompatibility(session),
        parachainReady: this.validateParachainRequirements(session),
        bridgeAvailability: true,
        recommendedParachain: this.recommendParachain(session)
      };
      
      return interoperability;
    } catch (error) {
      console.error('❌ Polkadot interoperability analysis failed:', error);
      return { error: error.message };
    }
  }

  /**
   * Enhanced AI analysis with cross-chain data
   */
  private async enhanceAIAnalysis(baseAnalysis: any, crossChainResults: any): Promise<any> {
    try {
      const enhancedAnalysis = {
        ...baseAnalysis,
        crossChainInsights: {
          multiChainEmotionConsistency: this.calculateMultiChainConsistency(crossChainResults),
          optimalDeploymentChain: this.recommendOptimalChain(crossChainResults),
          interoperabilityScore: this.calculateInteroperabilityScore(crossChainResults),
          crossChainRecommendations: this.generateCrossChainRecommendations(crossChainResults)
        },
        enhancedConfidence: this.calculateEnhancedConfidence(baseAnalysis, crossChainResults)
      };
      
      return enhancedAnalysis;
    } catch (error) {
      console.error('❌ Enhanced AI analysis failed:', error);
      return baseAnalysis;
    }
  }

  /**
   * Create cross-chain biometric NFT
   */
  async createCrossChainBiometricNFT(
    session: EnhancedBiometricSession,
    generatedArt: any,
    targetChains: string[] = ['near']
  ): Promise<{
    nearResult?: any;
    solanaResult?: any;
    filecoinResult?: any;
    polkadotResult?: any;
    crossChainTxId?: string;
  }> {
    try {
      console.log('🎨 Creating cross-chain biometric NFT...');
      
      const results: any = {};
      
      // Process enhanced biometric session
      const enhancedSession = await this.processEnhancedBiometricSession(session);
      
      // Create NEAR NFT (always primary)
      if (targetChains.includes('near')) {
        results.nearResult = await this.createAIBiometricNFT(session, generatedArt);
      }
      
      // Create Solana NFT if requested and bridge available
      if (targetChains.includes('solana') && this.crossChainConnections.get('solana')) {
        try {
          results.solanaResult = await this.createSolanaBiometricNFT(enhancedSession, generatedArt);
        } catch (error) {
          console.warn('Solana NFT creation failed:', error);
        }
      }
      
      // Store on Filecoin if requested
      if (targetChains.includes('filecoin') && this.crossChainConnections.get('filecoin')) {
        try {
          results.filecoinResult = await this.storeFilecoinMetadata(enhancedSession, generatedArt);
        } catch (error) {
          console.warn('Filecoin storage failed:', error);
        }
      }
      
      // Create Polkadot NFT if requested
      if (targetChains.includes('polkadot') && this.crossChainConnections.get('polkadot')) {
        try {
          results.polkadotResult = await this.createPolkadotBiometricNFT(enhancedSession, generatedArt);
        } catch (error) {
          console.warn('Polkadot NFT creation failed:', error);
        }
      }
      
      // Generate cross-chain transaction ID
      if (Object.keys(results).length > 0) {
        results.crossChainTxId = this.generateCrossChainTxId(results);
      }
      
      return results;
      
    } catch (error) {
      console.error('❌ Cross-chain biometric NFT creation failed:', error);
      throw error;
    }
  }

  /**
   * Helper methods for validation and calculations
   */
  private validateSolanaEmotionData(emotions: any[]): boolean {
    return emotions.every(e => 
      typeof e.valence === 'number' && 
      typeof e.arousal === 'number' && 
      typeof e.dominance === 'number'
    );
  }

  private validateSolanaHashFormat(sessionId: string): boolean {
    return sessionId.length <= 64 && /^[a-zA-Z0-9_-]+$/.test(sessionId);
  }

  private estimateSessionSize(session: EnhancedBiometricSession): number {
    const jsonString = JSON.stringify(session);
    return new Blob([jsonString]).size;
  }

  private calculateOptimalReplicas(session: EnhancedBiometricSession): number {
    const size = this.estimateSessionSize(session);
    return size > 1024 * 1024 ? 3 : 2; // 3 replicas for >1MB, 2 for smaller
  }

  private determineStorageClass(session: EnhancedBiometricSession): string {
    const qualityScore = session.aiAnalysis?.qualityScore || 0.5;
    return qualityScore > 0.8 ? 'premium' : 'standard';
  }

  private validateXCMCompatibility(session: EnhancedBiometricSession): boolean {
    return session.biometricData.emotions.length > 0 && 
           session.sessionId.length <= 64;
  }

  private validateParachainRequirements(session: EnhancedBiometricSession): boolean {
    return session.aiAnalysis?.qualityScore >= 0.6;
  }

  private recommendParachain(session: EnhancedBiometricSession): string {
    const qualityScore = session.aiAnalysis?.qualityScore || 0;
    return qualityScore > 0.8 ? 'acala' : 'moonbeam';
  }

  private calculateMultiChainConsistency(crossChainResults: any): number {
    const scores = [];
    if (crossChainResults.solana?.qualityScoreAcceptable) scores.push(1);
    if (crossChainResults.filecoin?.cidCompatibility) scores.push(1);
    if (crossChainResults.polkadot?.xcmCompatible) scores.push(1);
    return scores.length > 0 ? scores.reduce((a, b) => a + b) / scores.length : 0;
  }

  private recommendOptimalChain(crossChainResults: any): string {
    const scores = {
      near: 1.0, // Always available
      solana: crossChainResults.solana?.qualityScoreAcceptable ? 0.9 : 0,
      filecoin: crossChainResults.filecoin?.cidCompatibility ? 0.8 : 0,
      polkadot: crossChainResults.polkadot?.xcmCompatible ? 0.7 : 0
    };
    
    return Object.keys(scores).reduce((a, b) => scores[a] > scores[b] ? a : b);
  }

  private calculateInteroperabilityScore(crossChainResults: any): number {
    const weights = { solana: 0.3, filecoin: 0.3, polkadot: 0.4 };
    let score = 0;
    
    if (crossChainResults.solana) score += weights.solana * 0.9;
    if (crossChainResults.filecoin) score += weights.filecoin * 0.8;
    if (crossChainResults.polkadot) score += weights.polkadot * 0.7;
    
    return score;
  }

  private generateCrossChainRecommendations(crossChainResults: any): string[] {
    const recommendations = [];
    
    if (!crossChainResults.solana?.qualityScoreAcceptable) {
      recommendations.push('Improve biometric data quality for Solana deployment');
    }
    
    if (!crossChainResults.filecoin?.cidCompatibility) {
      recommendations.push('Optimize data format for Filecoin storage');
    }
    
    if (!crossChainResults.polkadot?.xcmCompatible) {
      recommendations.push('Adjust parameters for Polkadot XCM compatibility');
    }
    
    return recommendations;
  }

  private calculateEnhancedConfidence(baseAnalysis: any, crossChainResults: any): number {
    const baseConfidence = baseAnalysis.confidence || 0.5;
    const consistencyBonus = this.calculateMultiChainConsistency(crossChainResults) * 0.2;
    return Math.min(baseConfidence + consistencyBonus, 1.0);
  }

  private async createSolanaBiometricNFT(session: any, generatedArt: any): Promise<any> {
    // Implementation for Solana NFT creation
    console.log('Creating Solana biometric NFT...');
    return { success: true, chain: 'solana' };
  }

  private async storeFilecoinMetadata(session: any, generatedArt: any): Promise<any> {
    // Implementation for Filecoin metadata storage
    console.log('Storing metadata on Filecoin...');
    return { success: true, chain: 'filecoin' };
  }

  private async createPolkadotBiometricNFT(session: any, generatedArt: any): Promise<any> {
    // Implementation for Polkadot NFT creation
    console.log('Creating Polkadot biometric NFT...');
    return { success: true, chain: 'polkadot' };
  }

  private generateCrossChainTxId(results: any): string {
    const chains = Object.keys(results).sort().join('-');
    const timestamp = Date.now();
    return `crosschain-${chains}-${timestamp}`;
  }

  // Include existing methods from original NEARAIIntegration
  private async initializeContract(): Promise<void> {
    if (!this.wallet) throw new Error('Wallet not initialized');

    this.contract = new Contract(
      this.wallet.account(),
      this.config.contractId,
      {
        viewMethods: ['nft_token', 'nft_tokens_for_owner', 'get_ai_analysis'],
        changeMethods: ['mint_soulbound_ai', 'update_biometric_data', 'store_ai_analysis'],
      }
    );
  }

  async connectWallet(): Promise<void> {
    if (!this.wallet) throw new Error('Wallet not initialized');

    await this.wallet.requestSignIn({
      contractId: this.config.contractId,
      methodNames: ['mint_soulbound_ai', 'update_biometric_data', 'store_ai_analysis'],
    });

    await this.initializeContract();
    this.isConnected = true;
  }

  private async processBiometricSession(session: EnhancedBiometricSession): Promise<any> {
    // Implementation from original NEARAIIntegration
    // This is a simplified version - full implementation would include all original logic
    return {
      sessionId: session.sessionId,
      aiAnalysis: { confidence: 0.8 },
      biometricHash: 'hash_placeholder',
      qualityScore: 0.8
    };
  }

  private async createAIBiometricNFT(session: EnhancedBiometricSession, generatedArt: any): Promise<any> {
    // Implementation from original NEARAIIntegration
    // This is a simplified version - full implementation would include all original logic
    return {
      tokenId: 'token_placeholder',
      transactionHash: 'tx_placeholder',
      metadataCid: 'cid_placeholder',
      metadataUrl: 'url_placeholder',
      aiAnalysis: { confidence: 0.8 }
    };
  }

  getConnectionStatus(): boolean {
    return this.isConnected;
  }

  getWallet(): WalletConnection | null {
    return this.wallet;
  }

  getAccountId(): string | null {
    return this.wallet?.getAccountId() || null;
  }

  getCrossChainStatus(): Map<string, boolean> {
    return this.crossChainConnections;
  }
}

// Factory function for enhanced integration
export function createEnhancedNEARAIIntegration(config: EnhancedNEARAIConfig): EnhancedNEARAIIntegration {
  return new EnhancedNEARAIIntegration(config);
}
import { connect, keyStores, WalletConnection, Contract } from 'near-api-js';
import { WASMMLBridge } from './unified-ai-ml-integration';
import { FilecoinStorageClient } from './filecoin-storage';
import { createHash } from 'crypto';

export interface NEARAIConfig {
  networkId: 'mainnet' | 'testnet';
  contractId: string;
  web3StorageApiKey?: string;
}

export interface BiometricSession {
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
    }>;
    gestures?: any[];
    audio?: Float32Array;
  };
  aiAnalysis?: {
    emotionClassification: any;
    biometricFeatures: number[];
    qualityScore: number;
    predictions: any;
  };
}

export interface NEARAIGeneratedNFT {
  tokenId: string;
  title: string;
  description: string;
  media: Blob;
  attributes: Array<{
    trait_type: string;
    value: string | number;
  }>;
  extra: {
    biometric: {
      sessionId: string;
      hash: string;
      primaryEmotion: {
        valence: number;
        arousal: number;
        dominance: number;
        confidence: number;
      };
    };
    ai: {
      model: string;
      analysis: any;
      confidence: number;
    };
    storage: {
      type: 'filecoin';
      cid: string;
      url: string;
    };
  };
}

export class NEARAIIntegration {
  private wallet: WalletConnection | null = null;
  private contract: Contract | null = null;
  private connection: any = null;
  private aiBridge: WASMMLBridge;
  private storageClient: FilecoinStorageClient | null = null;
  private config: NEARAIConfig;
  private isConnected: boolean = false;

  constructor(config: NEARAIConfig) {
    this.config = config;
    this.aiBridge = new WASMMLBridge();
    
    if (config.web3StorageApiKey) {
      this.storageClient = new FilecoinStorageClient(config.web3StorageApiKey);
    }
  }

  /**
   * Initialize NEAR connection and wallet
   */
  async initialize(): Promise<void> {
    try {
      console.log('🔌 Initializing NEAR AI integration...');
      
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
      this.wallet = new WalletConnection(this.connection, 'near-ai-integration');
      
      if (this.wallet.isSignedIn()) {
        await this.initializeContract();
        this.isConnected = true;
        console.log('✅ NEAR AI integration initialized successfully');
      } else {
        console.log('⚠️ Wallet not signed in');
      }
      
    } catch (error) {
      console.error('❌ NEAR initialization failed:', error);
      throw error;
    }
  }

  /**
   * Initialize contract connection
   */
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

  /**
   * Connect wallet
   */
  async connectWallet(): Promise<void> {
    if (!this.wallet) throw new Error('Wallet not initialized');

    await this.wallet.requestSignIn({
      contractId: this.config.contractId,
      methodNames: ['mint_soulbound_ai', 'update_biometric_data', 'store_ai_analysis'],
    });

    await this.initializeContract();
    this.isConnected = true;
  }

  /**
   * Process biometric session with AI analysis
   */
  async processBiometricSession(session: BiometricSession): Promise<{
    sessionId: string;
    aiAnalysis: any;
    biometricHash: string;
    qualityScore: number;
  }> {
    try {
      console.log('🧠 Processing biometric session with AI analysis...');
      
      // Extract features from biometric data
      const features = this.extractBiometricFeatures(session.biometricData);
      
      // Process with Iron Learn for emotion classification
      const ironLearnResult = await this.aiBridge.processWithIronLearn({
        features,
        labels: session.biometricData.emotions
      }, 'emotion');

      // Store emotion vectors in LanceDB for similarity search
      const emotionVectors = session.biometricData.emotions.map(emotion => ({
        vector: [emotion.valence, emotion.arousal, emotion.dominance, emotion.confidence || 0.8],
        metadata: {
          sessionId: session.sessionId,
          timestamp: emotion.timestamp,
          userId: session.userId,
          type: 'near_biometric'
        }
      }));

      await this.aiBridge.storeEmotionVectors(emotionVectors);

      // Determine primary emotion
      const primaryEmotion = session.biometricData.emotions.reduce((best, current) => {
        if (!best || (current.confidence || 0.8) > (best.confidence || 0.8)) {
          return current;
        }
        return best;
      }, session.biometricData.emotions[0]);

      // Generate biometric hash
      const biometricHash = this.generateBiometricHash(session.biometricData);

      // Calculate quality score
      const qualityScore = this.calculateQualityScore({
        ironLearn: ironLearnResult,
        emotionVectors: emotionVectors.length,
        primaryEmotion,
        confidence: primaryEmotion.confidence || 0.8
      });

      const aiAnalysis = {
        ironLearn: ironLearnResult,
        emotionVectors: emotionVectors.length,
        primaryEmotion,
        confidence: primaryEmotion.confidence || 0.8,
        biometricFeatures: features,
        qualityScore,
        analysisTimestamp: Date.now()
      };

      // Store AI analysis on-chain if contract is available
      if (this.contract) {
        try {
          await this.storeAIAnalysis(session.sessionId, aiAnalysis);
        } catch (error) {
          console.warn('Failed to store AI analysis on-chain:', error);
        }
      }

      return {
        sessionId: session.sessionId,
        aiAnalysis,
        biometricHash,
        qualityScore
      };

    } catch (error) {
      console.error('❌ Biometric session processing failed:', error);
      throw error;
    }
  }

  /**
   * Create AI-powered biometric NFT
   */
  async createAIBiometricNFT(
    session: BiometricSession,
    generatedArt: {
      title: string;
      description: string;
      media: Blob;
      attributes?: Array<{
        trait_type: string;
        value: string | number;
      }>;
    }
  ): Promise<{
    tokenId: string;
    transactionHash: string;
    metadataCid: string;
    metadataUrl: string;
    aiAnalysis: any;
  }> {
    try {
      if (!this.contract) throw new Error('Contract not initialized');
      if (!this.wallet) throw new Error('Wallet not connected');

      console.log('🎨 Creating AI-powered biometric NFT...');

      // Process biometric session with AI
      const processedSession = await this.processBiometricSession(session);

      // Store artwork on Filecoin/IPFS
      let metadataCid = '';
      let metadataUrl = '';
      
      if (this.storageClient) {
        console.log('📤 Storing NFT media on Filecoin...');
        const storageResult = await this.storageClient.storeEmotionalArt({
          canvas: generatedArt.media as any,
          emotionData: processedSession.aiAnalysis.primaryEmotion,
          biometricHash: processedSession.biometricHash,
          aiModel: 'near-ai-integration',
          generationParams: {
            title: generatedArt.title,
            description: generatedArt.description,
            emotionBased: true
          }
        });
        
        metadataCid = storageResult.cid;
        metadataUrl = storageResult.url;
        console.log('✅ Media stored on Filecoin:', metadataCid);
      }

      // Create NFT metadata
      const tokenId = `biometric-ai-${session.sessionId}-${Date.now()}`;
      
      const nftData = {
        token_id: tokenId,
        receiver_id: this.wallet.getAccountId(),
        emotion_data: {
          valence: processedSession.aiAnalysis.primaryEmotion.valence,
          arousal: processedSession.aiAnalysis.primaryEmotion.arousal,
          dominance: processedSession.aiAnalysis.primaryEmotion.dominance,
          confidence: processedSession.aiAnalysis.primaryEmotion.confidence || 0.8,
          timestamp: processedSession.aiAnalysis.primaryEmotion.timestamp
        },
        biometric_hash: processedSession.biometricHash,
        quality_score: processedSession.qualityScore,
        ai_analysis: processedSession.aiAnalysis,
        metadata_uri: metadataUrl,
        attributes: [
          ...(generatedArt.attributes || []),
          {
            trait_type: 'AI Confidence',
            value: Math.round(processedSession.aiAnalysis.confidence * 100)
          },
          {
            trait_type: 'Emotion Vectors',
            value: processedSession.aiAnalysis.emotionVectors
          },
          {
            trait_type: 'Quality Score',
            value: Math.round(processedSession.qualityScore * 100)
          },
          {
            trait_type: 'Storage Type',
            value: 'Filecoin'
          }
        ]
      };

      // Mint the NFT
      console.log('⛏️ Minting AI biometric NFT...');
      const transactionHash = await (this.contract as any).mint_soulbound_ai(nftData);

      console.log('✅ AI-powered biometric NFT created successfully!');

      return {
        tokenId,
        transactionHash,
        metadataCid,
        metadataUrl,
        aiAnalysis: processedSession.aiAnalysis
      };

    } catch (error) {
      console.error('❌ AI biometric NFT creation failed:', error);
      throw error;
    }
  }

  /**
   * Get AI analysis for a specific session
   */
  async getAIAnalysis(sessionId: string): Promise<any> {
    if (!this.contract) throw new Error('Contract not initialized');
    
    try {
      const analysis = await (this.contract as any).get_ai_analysis({ session_id: sessionId });
      return analysis;
    } catch (error) {
      console.error('Failed to get AI analysis:', error);
      throw error;
    }
  }

  /**
   * Store AI analysis on-chain
   */
  private async storeAIAnalysis(sessionId: string, analysis: any): Promise<string> {
    if (!this.contract) throw new Error('Contract not initialized');
    
    try {
      const result = await (this.contract as any).store_ai_analysis({
        session_id: sessionId,
        analysis_data: analysis
      });
      return result;
    } catch (error) {
      console.error('Failed to store AI analysis:', error);
      throw error;
    }
  }

  /**
   * Find similar biometric sessions using LanceDB
   */
  async findSimilarSessions(
    emotionVector: number[],
    limit: number = 5
  ): Promise<any[]> {
    try {
      return await this.aiBridge.searchSimilarEmotions(emotionVector, limit);
    } catch (error) {
      console.error('Failed to search similar sessions:', error);
      throw error;
    }
  }

  /**
   * Get user's biometric NFTs
   */
  async getUserBiometricNFTs(accountId?: string): Promise<any[]> {
    if (!this.contract) throw new Error('Contract not initialized');
    
    const userId = accountId || this.wallet?.getAccountId();
    if (!userId) throw new Error('No account ID available');
    
    try {
      const nfts = await (this.contract as any).nft_tokens_for_owner({ account_id: userId });
      return nfts.filter((nft: any) => nft.metadata?.extra?.includes('biometric'));
    } catch (error) {
      console.error('Failed to get user NFTs:', error);
      throw error;
    }
  }

  /**
   * Extract biometric features for AI processing
   */
  private extractBiometricFeatures(biometricData: any): number[] {
    const features: number[] = [];
    
    // EEG features (if available)
    if (biometricData.eeg && biometricData.eeg.length > 0) {
      const eeg = biometricData.eeg;
      features.push(
        this.calculateAverage(eeg),
        this.calculateVariance(eeg),
        this.calculatePeakToPeak(eeg),
        this.calculateDominantFrequency(eeg)
      );
    } else {
      features.push(0, 0, 0, 0); // Placeholder features
    }

    // Heart rate features (if available)
    if (biometricData.heartRate && biometricData.heartRate.length > 0) {
      const hr = biometricData.heartRate;
      features.push(
        this.calculateAverage(hr),
        this.calculateVariance(hr),
        this.calculateRMSSD(hr), // Heart rate variability
        Math.max(...hr) - Math.min(...hr) // Range
      );
    } else {
      features.push(0, 0, 0, 0); // Placeholder features
    }

    // Emotion features (if available)
    if (biometricData.emotions && biometricData.emotions.length > 0) {
      const latestEmotion = biometricData.emotions[biometricData.emotions.length - 1];
      features.push(
        latestEmotion.valence,
        latestEmotion.arousal,
        latestEmotion.dominance,
        latestEmotion.confidence || 0.8
      );
    } else {
      features.push(0.5, 0.5, 0.5, 0.8); // Neutral emotion
    }

    return features;
  }

  /**
   * Calculate quality score based on AI analysis
   */
  private calculateQualityScore(analysis: any): number {
    try {
      const confidence = analysis.confidence || 0.8;
      const emotionVectors = analysis.emotionVectors || 1;
      
      // Base quality from confidence
      let qualityScore = confidence;
      
      // Bonus for having multiple emotion vectors (more data = better analysis)
      if (emotionVectors > 10) qualityScore += 0.1;
      if (emotionVectors > 50) qualityScore += 0.1;
      
      // Bonus for Iron Learn analysis results
      if (analysis.ironLearn?.predictions) {
        const predictionAccuracy = analysis.ironLearn.predictions.accuracy || 0;
        qualityScore += (predictionAccuracy * 0.2);
      }
      
      return Math.min(qualityScore, 1.0); // Cap at 1.0
    } catch (error) {
      console.error('Error calculating quality score:', error);
      return 0.5; // Default fallback
    }
  }

  /**
   * Generate biometric hash from session data
   */
  private generateBiometricHash(biometricData: any): string {
    const dataString = JSON.stringify({
      eeg: biometricData.eeg?.slice(0, 100),
      heartRate: biometricData.heartRate?.slice(0, 100),
      emotions: biometricData.emotions?.slice(-10),
      timestamp: Date.now()
    });
    
    return createHash('sha256').update(dataString).digest('hex');
  }

  /**
   * Utility functions for biometric feature extraction
   */
  private calculateAverage(data: number[]): number {
    return data.reduce((sum, val) => sum + val, 0) / data.length;
  }

  private calculateVariance(data: number[]): number {
    const avg = this.calculateAverage(data);
    return data.reduce((sum, val) => sum + Math.pow(val - avg, 2), 0) / data.length;
  }

  private calculatePeakToPeak(data: number[]): number {
    return Math.max(...data) - Math.min(...data);
  }

  private calculateDominantFrequency(data: number[]): number {
    // Simplified dominant frequency calculation
    return 10.0; // Placeholder
  }

  private calculateRMSSD(data: number[]): number {
    // Root Mean Square of Successive Differences (heart rate variability)
    if (data.length < 2) return 0;
    
    let sum = 0;
    for (let i = 1; i < data.length; i++) {
      const diff = data[i] - data[i - 1];
      sum += diff * diff;
    }
    
    return Math.sqrt(sum / (data.length - 1));
  }

  /**
   * Get connection status
   */
  getConnectionStatus(): boolean {
    return this.isConnected;
  }

  /**
   * Get wallet instance
   */
  getWallet(): WalletConnection | null {
    return this.wallet;
  }

  /**
   * Get account ID
   */
  getAccountId(): string | null {
    return this.wallet?.getAccountId() || null;
  }
}

// Factory function for easy instantiation
export function createNEARAIIntegration(config: NEARAIConfig): NEARAIIntegration {
  return new NEARAIIntegration(config);
}

// Utility function to validate NEAR AI setup
export function validateNEARAISetup(): {
  near: boolean;
  ai: boolean;
  storage: boolean;
  errors: string[];
} {
  const result = {
    near: false,
    ai: false,
    storage: false,
    errors: [] as string[]
  };

  try {
    // Check if NEAR API is available
    if (typeof connect !== 'undefined') {
      result.near = true;
    } else {
      result.errors.push('NEAR API not available');
    }

    // Check if AI bridge is available
    if (typeof WASMMLBridge !== 'undefined') {
      result.ai = true;
    } else {
      result.errors.push('AI/ML bridge not available');
    }

    // Check if storage client is available
    if (typeof FilecoinStorageClient !== 'undefined') {
      result.storage = true;
    } else {
      result.errors.push('Filecoin storage client not available');
    }

  } catch (error) {
    result.errors.push(`Setup validation failed: ${error}`);
  }

  return result;
}
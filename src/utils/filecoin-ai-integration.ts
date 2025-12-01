import { FilecoinStorageClient } from './filecoin-storage';
import { WASMMLBridge } from './unified-ai-ml-integration';
import { createHash } from 'crypto';

export interface AIGeneratedContent {
  type: 'art' | 'music' | 'text' | 'video';
  data: Blob | File;
  metadata: {
    name: string;
    description: string;
    aiModel: string;
    generationParams: Record<string, any>;
    biometricData?: {
      emotion: {
        valence: number;
        arousal: number;
        dominance: number;
        confidence: number;
      };
      hash: string;
    };
  };
}

export interface CreativeSession {
  sessionId: string;
  userId: string;
  biometricData: {
    eeg?: number[];
    heartRate?: number[];
    facial?: Blob;
    emotions: Array<{
      timestamp: number;
      valence: number;
      arousal: number;
      dominance: number;
      confidence: number;
    }>;
  };
  generatedContent: AIGeneratedContent[];
  timestamp: number;
}

export class FilecoinAIIntegration {
  private storageClient: FilecoinStorageClient;
  private aiBridge: WASMMLBridge;
  private apiKey: string;

  constructor(apiKey: string) {
    this.apiKey = apiKey;
    this.storageClient = new FilecoinStorageClient(apiKey);
    this.aiBridge = new WASMMLBridge();
  }

  /**
   * Process creative session with AI analysis and store on Filecoin
   */
  async processCreativeSession(session: CreativeSession): Promise<{
    sessionCid: string;
    contentCids: string[];
    aiAnalysis: any;
    storageUrls: string[];
  }> {
    try {
      console.log('🧠 Processing creative session with AI analysis...');
      
      // Analyze biometric data with AI
      const aiAnalysis = await this.analyzeCreativeSession(session);
      
      // Generate biometric hash
      const biometricHash = this.generateBiometricHash(session.biometricData);
      
      // Store each generated content piece
      const contentResults = await Promise.all(
        session.generatedContent.map(async (content) => {
          return await this.storeAIGeneratedContent(content, biometricHash);
        })
      );

      // Store session metadata
      const sessionMetadata = {
        sessionId: session.sessionId,
        userId: session.userId,
        timestamp: session.timestamp,
        biometricData: {
          ...session.biometricData,
          hash: biometricHash
        },
        aiAnalysis,
        contentCids: contentResults.map(r => r.cid),
        storageInfo: {
          totalContent: contentResults.length,
          totalSize: contentResults.reduce((sum, r) => sum + r.size, 0)
        }
      };

      const sessionResult = await this.storageClient.storeBiometricData({
        metadata: {
          userId: session.userId,
          sessionId: session.sessionId,
          timestamp: new Date(session.timestamp).toISOString(),
          deviceInfo: 'AI Creative Session'
        },
        eegData: session.biometricData.eeg,
        heartRateData: session.biometricData.heartRate,
        facialData: session.biometricData.facial
      });

      return {
        sessionCid: sessionResult.cid,
        contentCids: contentResults.map(r => r.cid),
        aiAnalysis,
        storageUrls: [sessionResult.url, ...contentResults.map(r => r.url)]
      };

    } catch (error) {
      console.error('Failed to process creative session:', error);
      throw error;
    }
  }

  /**
   * Analyze creative session with AI/ML pipeline
   */
  private async analyzeCreativeSession(session: CreativeSession): Promise<any> {
    try {
      // Process biometric data through Iron Learn
      const biometricFeatures = this.extractBiometricFeatures(session.biometricData);
      
      const ironLearnResult = await this.aiBridge.processWithIronLearn({
        features: biometricFeatures,
        labels: session.biometricData.emotions.map(e => ({
          valence: e.valence,
          arousal: e.arousal,
          dominance: e.dominance,
          confidence: e.confidence
        }))
      }, 'emotion');

      // Store emotion vectors in LanceDB for similarity search
      const emotionVectors = session.biometricData.emotions.map(emotion => ({
        vector: [emotion.valence, emotion.arousal, emotion.dominance, emotion.confidence],
        metadata: {
          sessionId: session.sessionId,
          timestamp: emotion.timestamp,
          userId: session.userId
        }
      }));

      await this.aiBridge.storeEmotionVectors(emotionVectors);

      // Analyze creative patterns
      const creativePatterns = await this.aiBridge.analyzeCreativePatterns({
        biometricData: session.biometricData,
        generatedContent: session.generatedContent
      });

      return {
        ironLearn: ironLearnResult,
        emotionVectors: emotionVectors.length,
        creativePatterns,
        sessionInsights: this.generateSessionInsights(session.biometricData.emotions)
      };

    } catch (error) {
      console.error('AI analysis failed:', error);
      throw error;
    }
  }

  /**
   * Store AI-generated content with proper metadata
   */
  private async storeAIGeneratedContent(
    content: AIGeneratedContent, 
    biometricHash: string
  ): Promise<{
    cid: string;
    url: string;
    size: number;
  }> {
    try {
      // Create enhanced metadata with AI analysis
      const enhancedMetadata = {
        ...content.metadata,
        biometric: {
          hash: biometricHash,
          timestamp: new Date().toISOString()
        },
        ai: {
          model: content.metadata.aiModel,
          parameters: content.metadata.generationParams,
          processing: {
            framework: 'Iron Learn + LanceDB',
            version: '1.0.0'
          }
        },
        content: {
          type: content.type,
          size: content.data.size,
          format: content.data.type
        }
      };

      let result;
      
      if (content.type === 'art') {
        // Store as emotional art
        const emotionData = content.metadata.biometricData?.emotion || {
          valence: 0.5,
          arousal: 0.5,
          dominance: 0.5,
          confidence: 0.8
        };

        result = await this.storageClient.storeEmotionalArt({
          canvas: content.data as any, // Canvas element or image blob
          emotionData,
          biometricHash,
          aiModel: content.metadata.aiModel,
          generationParams: content.metadata.generationParams
        });
      } else {
        // Store as generic NFT data
        result = await this.storageClient.storeNFTData({
          name: content.metadata.name,
          description: content.metadata.description,
          image: content.data as Blob,
          properties: enhancedMetadata
        });
      }

      return {
        cid: result.cid,
        url: result.url,
        size: content.data.size
      };

    } catch (error) {
      console.error('Failed to store AI-generated content:', error);
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
        latestEmotion.confidence
      );
    } else {
      features.push(0.5, 0.5, 0.5, 0.8); // Neutral emotion
    }

    return features;
  }

  /**
   * Generate biometric hash from session data
   */
  private generateBiometricHash(biometricData: any): string {
    const dataString = JSON.stringify({
      eeg: biometricData.eeg?.slice(0, 100), // First 100 samples
      heartRate: biometricData.heartRate?.slice(0, 100),
      emotions: biometricData.emotions?.slice(-10), // Last 10 emotions
      timestamp: Date.now()
    });
    
    return createHash('sha256').update(dataString).digest('hex');
  }

  /**
   * Generate session insights from emotion data
   */
  private generateSessionInsights(emotions: any[]): any {
    if (emotions.length === 0) {
      return { insight: 'No emotion data available' };
    }

    const avgValence = emotions.reduce((sum, e) => sum + e.valence, 0) / emotions.length;
    const avgArousal = emotions.reduce((sum, e) => sum + e.arousal, 0) / emotions.length;
    const avgDominance = emotions.reduce((sum, e) => sum + e.dominance, 0) / emotions.length;

    let emotionalState = '';
    if (avgValence > 0.5) {
      emotionalState = avgArousal > 0.5 ? 'Excited Joy' : 'Calm Happiness';
    } else if (avgValence > 0) {
      emotionalState = avgArousal > 0.5 ? 'Mild Excitement' : 'Neutral Contentment';
    } else {
      emotionalState = avgArousal > 0.5 ? 'Agitated Stress' : 'Calm Melancholy';
    }

    return {
      emotionalState,
      avgValence: Math.round(avgValence * 100) / 100,
      avgArousal: Math.round(avgArousal * 100) / 100,
      avgDominance: Math.round(avgDominance * 100) / 100,
      emotionCount: emotions.length,
      insight: `This session captured ${emotions.length} emotional states, with an overall ${emotionalState} pattern.`
    };
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
    // In practice, this would use FFT
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
   * Search for similar creative sessions using LanceDB
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
   * Get AI-powered recommendations for creative content
   */
  async getCreativeRecommendations(
    userId: string, 
    biometricData: any
  ): Promise<any[]> {
    try {
      // Analyze user's biometric patterns
      const userPatterns = await this.aiBridge.analyzeUserPatterns(userId);
      
      // Get emotion-based recommendations
      const emotionRecs = await this.aiBridge.getEmotionBasedRecommendations(
        biometricData.emotions?.[0] || { valence: 0.5, arousal: 0.5, dominance: 0.5 }
      );

      return [
        ...userPatterns.recommendations,
        ...emotionRecs
      ];
    } catch (error) {
      console.error('Failed to get creative recommendations:', error);
      throw error;
    }
  }
}

// Factory function for easy instantiation
export function createFilecoinAIIntegration(apiKey: string): FilecoinAIIntegration {
  return new FilecoinAIIntegration(apiKey);
}

// Utility function to validate Filecoin AI integration setup
export function validateFilecoinAISetup(): {
  storage: boolean;
  ai: boolean;
  integration: boolean;
  errors: string[];
} {
  const result = {
    storage: false,
    ai: false,
    integration: false,
    errors: [] as string[]
  };

  try {
    // Check if Web3.Storage is available
    if (typeof Web3Storage !== 'undefined') {
      result.storage = true;
    } else {
      result.errors.push('Web3.Storage not available');
    }

    // Check if AI bridge is available
    if (typeof WASMMLBridge !== 'undefined') {
      result.ai = true;
    } else {
      result.errors.push('AI/ML bridge not available');
    }

    // Check if integration can be created
    try {
      const integration = new FilecoinAIIntegration('test-key');
      result.integration = true;
    } catch (error) {
      result.errors.push(`Integration creation failed: ${error}`);
    }

  } catch (error) {
    result.errors.push(`Setup validation failed: ${error}`);
  }

  return result;
}
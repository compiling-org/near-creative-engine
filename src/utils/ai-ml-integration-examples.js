/**
 * AI/ML Integration Example for Biometric NFT Pipeline
 * 
 * Demonstrates how to integrate Iron Learn, LanceDB, and Candle frameworks
 * with the existing biometric NFT analysis and cross-chain bridge
 */

import { unifiedAIMLPipeline } from './unified-ai-ml-integration.js';
import { createHash } from 'crypto';

/**
 * Example: Process creative session data through AI/ML pipeline
 */
export async function processCreativeSessionWithAI(creativeSessionData) {
  console.log('🎨 Processing creative session with AI/ML pipeline...');
  
  try {
    // Step 1: Prepare biometric data from creative session
    const biometricData = prepareBiometricDataFromSession(creativeSessionData);
    
    // Step 2: Process through unified AI/ML pipeline
    const aiResults = await unifiedAIMLPipeline.processBiometricData(biometricData, {
      modelType: 'emotion',
      includeSimilaritySearch: true
    });
    
    // Step 3: Generate biometric hash for NFT metadata
    const biometricHash = unifiedAIMLPipeline.generateBiometricHash(aiResults);
    
    // Step 4: Create enhanced NFT metadata with AI analysis
    const nftMetadata = createEnhancedNFTMetadata(creativeSessionData, aiResults, biometricHash);
    
    // Step 5: Store in vector database for future similarity searches
    await storeCreativeSessionInVectorDB(creativeSessionData, aiResults);
    
    return {
      success: true,
      ai_analysis: aiResults,
      biometric_hash: biometricHash,
      nft_metadata: nftMetadata,
      timestamp: new Date().toISOString()
    };
    
  } catch (error) {
    console.error('❌ Creative session AI processing failed:', error);
    throw error;
  }
}

/**
 * Example: Find similar creative sessions based on emotional patterns
 */
export async function findSimilarCreativeSessions(emotionalQuery) {
  console.log('🔍 Finding similar creative sessions...');
  
  try {
    // Prepare query data
    const queryData = {
      emotions: emotionalQuery.states,
      signals: emotionalQuery.biometric_signals || {},
      metadata: {
        creative_type: emotionalQuery.creative_type,
        session_duration: emotionalQuery.duration
      }
    };
    
    // Search for similar patterns
    const searchResults = await unifiedAIMLPipeline.searchSimilarBiometricPatterns(queryData, {
      limit: 10,
      similarity_threshold: 0.75
    });
    
    return {
      success: true,
      similar_sessions: searchResults.search_results,
      query_analysis: searchResults.query_results,
      found_count: searchResults.search_results.length
    };
    
  } catch (error) {
    console.error('❌ Similarity search failed:', error);
    throw error;
  }
}

/**
 * Example: Train custom emotion classification model
 */
export async function trainCustomEmotionModel(trainingData) {
  console.log('🧠 Training custom emotion classification model...');
  
  try {
    // Prepare training data for Iron Learn
    const preparedTrainingData = {
      features: trainingData.samples.map(sample => [
        sample.emotions.valence,
        sample.emotions.arousal,
        sample.emotions.dominance,
        sample.biometric_confidence || 0.8
      ]),
      labels: trainingData.samples.map(sample => sample.emotion_label),
      feature_names: ['valence', 'arousal', 'dominance', 'confidence']
    };
    
    // Train model using Iron Learn integration
    const modelResults = await unifiedAIMLPipeline.wasmBridge.processWithIronLearn(
      preparedTrainingData,
      'custom_emotion'
    );
    
    return {
      success: true,
      model: modelResults.model,
      training_metrics: modelResults.metrics,
      model_id: createHash('sha256').update(JSON.stringify(modelResults.model)).digest('hex')
    };
    
  } catch (error) {
    console.error('❌ Model training failed:', error);
    throw error;
  }
}

/**
 * Example: Real-time biometric monitoring during creative sessions
 */
export class RealTimeBiometricMonitor {
  constructor(options = {}) {
    this.pipeline = unifiedAIMLPipeline;
    this.monitoring = false;
    this.sessionData = [];
    this.callbacks = options.callbacks || {};
    this.samplingRate = options.samplingRate || 1000; // 1Hz default
  }

  /**
   * Start real-time biometric monitoring
   */
  async startMonitoring(sessionId) {
    console.log(`📊 Starting real-time biometric monitoring for session: ${sessionId}`);
    
    this.sessionId = sessionId;
    this.monitoring = true;
    this.sessionData = [];
    
    // Initialize pipeline
    await this.pipeline.initialize();
    
    // Set up monitoring interval
    this.monitorInterval = setInterval(async () => {
      if (!this.monitoring) return;
      
      try {
        // Simulate biometric data collection (in production, this would read from actual sensors)
        const biometricSnapshot = await this.collectBiometricSnapshot();
        
        // Process through AI pipeline
        const aiResults = await this.pipeline.processBiometricData(biometricSnapshot, {
          modelType: 'real_time_emotion'
        });
        
        // Store session data
        this.sessionData.push({
          timestamp: new Date().toISOString(),
          biometric_data: biometricSnapshot,
          ai_analysis: aiResults
        });
        
        // Trigger callbacks
        if (this.callbacks.onAnalysis) {
          this.callbacks.onAnalysis(aiResults);
        }
        
        // Check for significant emotional changes
        if (this.detectSignificantEmotionalChange(aiResults)) {
          if (this.callbacks.onEmotionalChange) {
            this.callbacks.onEmotionalChange(aiResults);
          }
        }
        
      } catch (error) {
        console.error('❌ Real-time monitoring error:', error);
        if (this.callbacks.onError) {
          this.callbacks.onError(error);
        }
      }
    }, this.samplingRate);
    
    console.log('✅ Real-time monitoring started');
  }

  /**
   * Stop real-time monitoring
   */
  stopMonitoring() {
    console.log('⏹️ Stopping real-time biometric monitoring');
    
    this.monitoring = false;
    
    if (this.monitorInterval) {
      clearInterval(this.monitorInterval);
      this.monitorInterval = null;
    }
    
    // Generate session summary
    const sessionSummary = this.generateSessionSummary();
    
    if (this.callbacks.onSessionEnd) {
      this.callbacks.onSessionEnd(sessionSummary);
    }
    
    return sessionSummary;
  }

  /**
   * Collect biometric snapshot (simulated)
   */
  async collectBiometricSnapshot() {
    // In production, this would read from actual biometric sensors
    return {
      eeg: {
        epochs: [{
          timestamp: Date.now(),
          signals: Array(256).fill(0).map(() => Math.random() * 100),
          signalQuality: 0.8 + Math.random() * 0.2
        }]
      },
      emg: {
        epochs: [{
          timestamp: Date.now(),
          muscleActivity: Math.random(),
          signalQuality: 0.7 + Math.random() * 0.3
        }]
      },
      ecg: {
        epochs: [{
          timestamp: Date.now(),
          signals: Array(100).fill(0).map(() => 60 + Math.random() * 40),
          signalQuality: 0.9 + Math.random() * 0.1
        }]
      },
      emotions: {
        states: [{
          valence: 0.5 + Math.random() * 0.4 - 0.2,
          arousal: 0.5 + Math.random() * 0.4 - 0.2,
          dominance: 0.5 + Math.random() * 0.4 - 0.2,
          confidence: 0.8 + Math.random() * 0.2,
          timestamp: Date.now()
        }]
      }
    };
  }

  /**
   * Detect significant emotional changes
   */
  detectSignificantEmotionalChange(currentResults) {
    if (this.sessionData.length < 2) return false;
    
    const previousAnalysis = this.sessionData[this.sessionData.length - 2].ai_analysis;
    const currentAnalysis = currentResults;
    
    // Check for significant changes in emotional dimensions
    const prevEmotion = previousAnalysis.iron_learn.predictions[0];
    const currEmotion = currentAnalysis.iron_learn.predictions[0];
    
    const valenceChange = Math.abs(currEmotion.valence - prevEmotion.valence);
    const arousalChange = Math.abs(currEmotion.arousal - prevEmotion.arousal);
    const dominanceChange = Math.abs(currEmotion.dominance - prevEmotion.dominance);
    
    // Significant change if any dimension changes by more than 0.3
    return valenceChange > 0.3 || arousalChange > 0.3 || dominanceChange > 0.3;
  }

  /**
   * Generate session summary
   */
  generateSessionSummary() {
    if (this.sessionData.length === 0) {
      return { error: 'No session data collected' };
    }
    
    const emotions = this.sessionData.map(entry => entry.ai_analysis.iron_learn.predictions[0]);
    const avgEmotion = {
      valence: emotions.reduce((sum, e) => sum + e.valence, 0) / emotions.length,
      arousal: emotions.reduce((sum, e) => sum + e.arousal, 0) / emotions.length,
      dominance: emotions.reduce((sum, e) => sum + e.dominance, 0) / emotions.length
    };
    
    const emotionChanges = this.sessionData.filter((entry, index) => {
      if (index === 0) return false;
      return this.detectSignificantEmotionalChange(entry.ai_analysis);
    }).length;
    
    return {
      session_id: this.sessionId,
      duration_ms: this.sessionData.length * this.samplingRate,
      data_points: this.sessionData.length,
      average_emotion: avgEmotion,
      emotion_changes: emotionChanges,
      dominant_emotion: this.getDominantEmotion(emotions),
      emotional_stability: this.calculateEmotionalStability(emotions),
      session_data: this.sessionData
    };
  }

  /**
   * Get dominant emotion from session
   */
  getDominantEmotion(emotions) {
    const emotionCounts = {};
    emotions.forEach(emotion => {
      const label = emotion.predicted_emotion || 'neutral';
      emotionCounts[label] = (emotionCounts[label] || 0) + 1;
    });
    
    return Object.keys(emotionCounts).reduce((a, b) => 
      emotionCounts[a] > emotionCounts[b] ? a : b
    );
  }

  /**
   * Calculate emotional stability
   */
  calculateEmotionalStability(emotions) {
    if (emotions.length < 2) return 1.0;
    
    const changes = [];
    for (let i = 1; i < emotions.length; i++) {
      const change = Math.abs(emotions[i].valence - emotions[i-1].valence) +
                     Math.abs(emotions[i].arousal - emotions[i-1].arousal) +
                     Math.abs(emotions[i].dominance - emotions[i-1].dominance);
      changes.push(change);
    }
    
    const avgChange = changes.reduce((sum, change) => sum + change, 0) / changes.length;
    return Math.max(0, 1 - avgChange); // Higher stability = less change
  }
}

/**
 * Helper functions
 */

function prepareBiometricDataFromSession(sessionData) {
  return {
    eeg: sessionData.biometric_signals?.eeg || null,
    emg: sessionData.biometric_signals?.emg || null,
    ecg: sessionData.biometric_signals?.ecg || null,
    emotions: {
      states: sessionData.emotional_states || [{
        valence: sessionData.valence || 0.5,
        arousal: sessionData.arousal || 0.5,
        dominance: sessionData.dominance || 0.5,
        confidence: sessionData.confidence || 0.8,
        timestamp: Date.now()
      }]
    },
    metadata: {
      session_id: sessionData.session_id,
      creative_type: sessionData.creative_type,
      duration: sessionData.duration,
      complexity_score: sessionData.complexity_score
    }
  };
}

function createEnhancedNFTMetadata(sessionData, aiResults, biometricHash) {
  return {
    name: `Biometric NFT - ${sessionData.creative_type} Session`,
    description: `AI-analyzed creative session with emotional fingerprint`,
    image: sessionData.preview_image || 'ipfs://default-creative-image',
    attributes: [
      {
        trait_type: 'Creative Type',
        value: sessionData.creative_type
      },
      {
        trait_type: 'Dominant Emotion',
        value: aiResults.iron_learn.predictions[0]?.predicted_emotion || 'unknown'
      },
      {
        trait_type: 'Emotional Valence',
        value: aiResults.iron_learn.predictions[0]?.valence || 0.5,
        display_type: 'number'
      },
      {
        trait_type: 'Emotional Arousal', 
        value: aiResults.iron_learn.predictions[0]?.arousal || 0.5,
        display_type: 'number'
      },
      {
        trait_type: 'Emotional Dominance',
        value: aiResults.iron_learn.predictions[0]?.dominance || 0.5,
        display_type: 'number'
      },
      {
        trait_type: 'AI Model Accuracy',
        value: aiResults.iron_learn.model.training_metrics.accuracy,
        display_type: 'number'
      },
      {
        trait_type: 'Biometric Hash',
        value: biometricHash
      }
    ],
    biometric_analysis: {
      hash: biometricHash,
      model_metrics: aiResults.iron_learn.model.training_metrics,
      similar_patterns_found: aiResults.similar_patterns?.length || 0,
      processing_timestamp: aiResults.processing_timestamp
    }
  };
}

async function storeCreativeSessionInVectorDB(sessionData, aiResults) {
  const vectorData = {
    emotions: aiResults.iron_learn.predictions,
    signals: sessionData.biometric_signals || {},
    metadata: {
      session_id: sessionData.session_id,
      creative_type: sessionData.creative_type,
      duration: sessionData.duration,
      complexity_score: sessionData.complexity_score,
      ai_model_accuracy: aiResults.iron_learn.model.training_metrics.accuracy
    }
  };
  
  return await unifiedAIMLPipeline.processWithLanceDB(vectorData, 'store');
}

// Export usage examples
export const AIIntegrationExamples = {
  processCreativeSessionWithAI,
  findSimilarCreativeSessions,
  trainCustomEmotionModel,
  RealTimeBiometricMonitor
};
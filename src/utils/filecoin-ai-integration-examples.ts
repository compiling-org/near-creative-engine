import { 
  FilecoinAIIntegration, 
  createFilecoinAIIntegration,
  AIGeneratedContent,
  CreativeSession 
} from './filecoin-ai-integration';
import { WASMMLBridge } from './unified-ai-ml-integration';

/**
 * Example: Complete Filecoin AI Integration Workflow
 * This demonstrates real-world usage of the Filecoin + AI/ML integration
 */

// Initialize the integration (use real Web3.Storage API key in production)
const filecoinAI = createFilecoinAIIntegration(process.env.WEB3_STORAGE_API_KEY || 'your-api-key');

/**
 * Example 1: Process a complete creative session with AI analysis
 */
export async function exampleCreativeSessionWorkflow() {
  console.log('🎨 Starting creative session workflow...');

  try {
    // Create a sample creative session
    const creativeSession: CreativeSession = {
      sessionId: `session-${Date.now()}`,
      userId: 'user-123',
      timestamp: Date.now(),
      biometricData: {
        eeg: generateSampleEEGData(1000), // 1000 samples
        heartRate: generateSampleHeartRateData(1000), // 1000 samples
        emotions: generateSampleEmotionData(50), // 50 emotion readings
        facial: null // Could include facial recognition data
      },
      generatedContent: [
        {
          type: 'art',
          data: await generateSampleArtwork(), // Canvas or image blob
          metadata: {
            name: 'Emotional Landscape #001',
            description: 'AI-generated artwork based on biometric emotional data',
            aiModel: 'stable-diffusion-v2.1',
            generationParams: {
              prompt: 'emotional landscape, abstract, vibrant colors',
              style: 'abstract',
              emotion: 'joyful'
            },
            biometricData: {
              emotion: {
                valence: 0.8,
                arousal: 0.6,
                dominance: 0.7,
                confidence: 0.9
              },
              hash: 'sample-biometric-hash'
            }
          }
        },
        {
          type: 'music',
          data: await generateSampleMusic(), // Audio blob
          metadata: {
            name: 'Heart Rate Symphony #001',
            description: 'AI-generated music synchronized with biometric data',
            aiModel: 'musicgen-large',
            generationParams: {
              tempo: 120,
              key: 'C major',
              emotion: 'uplifting'
            }
          }
        }
      ]
    };

    // Process the session with AI analysis and store on Filecoin
    console.log('🧠 Processing session with AI analysis...');
    const result = await filecoinAI.processCreativeSession(creativeSession);

    console.log('✅ Session processed successfully!');
    console.log('📊 Results:');
    console.log(`   Session CID: ${result.sessionCid}`);
    console.log(`   Content CIDs: ${result.contentCids.join(', ')}`);
    console.log(`   Storage URLs: ${result.storageUrls.join(', ')}`);
    console.log(`   AI Analysis:`, result.aiAnalysis);

    return result;

  } catch (error) {
    console.error('❌ Creative session workflow failed:', error);
    throw error;
  }
}

/**
 * Example 2: Find similar creative sessions using emotion vectors
 */
export async function exampleSimilaritySearch() {
  console.log('🔍 Searching for similar creative sessions...');

  try {
    // Search for sessions with similar emotional patterns
    const emotionVector = [0.8, 0.6, 0.7, 0.9]; // [valence, arousal, dominance, confidence]
    const similarSessions = await filecoinAI.findSimilarSessions(emotionVector, 5);

    console.log('✅ Found similar sessions:');
    similarSessions.forEach((session, index) => {
      console.log(`   ${index + 1}. Session ID: ${session.metadata.sessionId}`);
      console.log(`      Similarity Score: ${session.score}`);
      console.log(`      Emotion Vector: [${session.vector.join(', ')}]`);
      console.log(`      Timestamp: ${new Date(session.metadata.timestamp).toLocaleString()}`);
    });

    return similarSessions;

  } catch (error) {
    console.error('❌ Similarity search failed:', error);
    throw error;
  }
}

/**
 * Example 3: Get AI-powered creative recommendations
 */
export async function exampleCreativeRecommendations() {
  console.log('💡 Getting AI-powered creative recommendations...');

  try {
    const userId = 'user-123';
    const biometricData = {
      emotions: [{
        valence: 0.8,
        arousal: 0.6,
        dominance: 0.7,
        confidence: 0.9,
        timestamp: Date.now()
      }],
      eeg: generateSampleEEGData(100),
      heartRate: generateSampleHeartRateData(100)
    };

    const recommendations = await filecoinAI.getCreativeRecommendations(userId, biometricData);

    console.log('✅ Creative recommendations:');
    recommendations.forEach((rec, index) => {
      console.log(`   ${index + 1}. ${rec.title}`);
      console.log(`      Description: ${rec.description}`);
      console.log(`      Confidence: ${rec.confidence}`);
      console.log(`      Type: ${rec.type}`);
    });

    return recommendations;

  } catch (error) {
    console.error('❌ Recommendations failed:', error);
    throw error;
  }
}

/**
 * Example 4: Real-time creative session monitoring
 */
export async function exampleRealTimeMonitoring() {
  console.log('📡 Starting real-time creative session monitoring...');

  const sessionId = `realtime-${Date.now()}`;
  const userId = 'user-123';
  
  // Simulate real-time biometric data collection
  const monitoringInterval = setInterval(async () => {
    try {
      const currentBiometric = {
        eeg: generateSampleEEGData(10), // 10 samples per interval
        heartRate: generateSampleHeartRateData(10),
        emotions: [generateRandomEmotion()],
        timestamp: Date.now()
      };

      console.log('📊 Current biometric reading:', {
        emotion: currentBiometric.emotions[0],
        heartRateAvg: currentBiometric.heartRate.reduce((a, b) => a + b) / currentBiometric.heartRate.length,
        eegActivity: currentBiometric.eeg.reduce((a, b) => a + b) / currentBiometric.eeg.length
      });

      // You could store this real-time data or use it for immediate AI processing
      // For demo purposes, we'll just log it

    } catch (error) {
      console.error('❌ Real-time monitoring error:', error);
    }
  }, 5000); // Every 5 seconds

  // Stop monitoring after 30 seconds
  setTimeout(() => {
    clearInterval(monitoringInterval);
    console.log('✅ Real-time monitoring completed');
  }, 30000);
}

/**
 * Example 5: Batch process multiple creative sessions
 */
export async function exampleBatchProcessing() {
  console.log('📦 Starting batch processing of multiple sessions...');

  try {
    const sessions = [];
    
    // Generate 5 sample sessions
    for (let i = 0; i < 5; i++) {
      sessions.push({
        sessionId: `batch-session-${i}-${Date.now()}`,
        userId: `user-${i % 3}`, // Rotate between 3 users
        timestamp: Date.now() - (i * 3600000), // Stagger by 1 hour
        biometricData: {
          eeg: generateSampleEEGData(500),
          heartRate: generateSampleHeartRateData(500),
          emotions: generateSampleEmotionData(25)
        },
        generatedContent: [{
          type: 'art' as const,
          data: await generateSampleArtwork(),
          metadata: {
            name: `Batch Artwork #${i}`,
            description: `AI-generated artwork from batch session ${i}`,
            aiModel: 'stable-diffusion-v2.1',
            generationParams: { style: 'abstract', emotion: 'varied' }
          }
        }]
      });
    }

    // Process all sessions in parallel
    const results = await Promise.all(
      sessions.map(session => filecoinAI.processCreativeSession(session))
    );

    console.log('✅ Batch processing completed!');
    console.log(`   Processed ${results.length} sessions`);
    console.log(`   Total CIDs generated: ${results.reduce((sum, r) => sum + r.contentCids.length, 0)}`);
    console.log(`   Average processing time per session: ${30 / results.length}s`);

    return results;

  } catch (error) {
    console.error('❌ Batch processing failed:', error);
    throw error;
  }
}

/**
 * Helper functions for generating sample data
 */

function generateSampleEEGData(length: number): number[] {
  const data = [];
  for (let i = 0; i < length; i++) {
    // Simulate EEG data with some realistic patterns
    const base = Math.sin(i * 0.1) * 50;
    const noise = (Math.random() - 0.5) * 20;
    const alpha = Math.sin(i * 0.05) * 10; // Alpha wave simulation
    data.push(base + noise + alpha);
  }
  return data;
}

function generateSampleHeartRateData(length: number): number[] {
  const data = [];
  const baseHR = 70; // Base heart rate
  for (let i = 0; i < length; i++) {
    const variation = Math.sin(i * 0.02) * 10; // Heart rate variability
    const noise = (Math.random() - 0.5) * 5;
    data.push(baseHR + variation + noise);
  }
  return data;
}

function generateSampleEmotionData(length: number): any[] {
  const emotions = [];
  for (let i = 0; i < length; i++) {
    emotions.push({
      timestamp: Date.now() - (length - i) * 1000, // 1 second intervals
      valence: Math.random() * 2 - 1, // -1 to 1
      arousal: Math.random(), // 0 to 1
      dominance: Math.random(), // 0 to 1
      confidence: 0.7 + Math.random() * 0.3 // 0.7 to 1.0
    });
  }
  return emotions;
}

function generateRandomEmotion(): any {
  return {
    timestamp: Date.now(),
    valence: Math.random() * 2 - 1,
    arousal: Math.random(),
    dominance: Math.random(),
    confidence: 0.7 + Math.random() * 0.3
  };
}

async function generateSampleArtwork(): Promise<Blob> {
  // Create a simple canvas-based artwork for demonstration
  const canvas = document.createElement('canvas');
  canvas.width = 512;
  canvas.height = 512;
  const ctx = canvas.getContext('2d')!;
  
  // Generate abstract art based on random parameters
  const colors = ['#FF6B6B', '#4ECDC4', '#45B7D1', '#96CEB4', '#FFEAA7'];
  
  // Background
  ctx.fillStyle = colors[Math.floor(Math.random() * colors.length)];
  ctx.fillRect(0, 0, canvas.width, canvas.height);
  
  // Abstract shapes
  for (let i = 0; i < 10; i++) {
    ctx.fillStyle = colors[Math.floor(Math.random() * colors.length)];
    ctx.globalAlpha = 0.7;
    ctx.beginPath();
    ctx.arc(
      Math.random() * canvas.width,
      Math.random() * canvas.height,
      Math.random() * 100 + 20,
      0,
      Math.PI * 2
    );
    ctx.fill();
  }
  
  return new Promise((resolve) => {
    canvas.toBlob((blob) => {
      resolve(blob!);
    }, 'image/png');
  });
}

async function generateSampleMusic(): Promise<Blob> {
  // Generate a simple audio blob for demonstration
  // In a real implementation, this would use an AI music generation model
  const sampleRate = 44100;
  const duration = 5; // 5 seconds
  const samples = sampleRate * duration;
  
  const audioData = new Float32Array(samples);
  
  // Generate simple sine wave with varying frequency
  for (let i = 0; i < samples; i++) {
    const time = i / sampleRate;
    const frequency = 440 + Math.sin(time * 2) * 100; // Varying frequency
    audioData[i] = Math.sin(2 * Math.PI * frequency * time) * 0.3;
  }
  
  // Convert to WAV format (simplified)
  const wavData = createWavFile(audioData, sampleRate);
  return new Blob([wavData], { type: 'audio/wav' });
}

function createWavFile(audioData: Float32Array, sampleRate: number): ArrayBuffer {
  const buffer = new ArrayBuffer(44 + audioData.length * 2);
  const view = new DataView(buffer);
  
  // WAV header
  const writeString = (offset: number, string: string) => {
    for (let i = 0; i < string.length; i++) {
      view.setUint8(offset + i, string.charCodeAt(i));
    }
  };
  
  writeString(0, 'RIFF');
  view.setUint32(4, 36 + audioData.length * 2, true);
  writeString(8, 'WAVE');
  writeString(12, 'fmt ');
  view.setUint32(16, 16, true);
  view.setUint16(20, 1, true);
  view.setUint16(22, 1, true);
  view.setUint32(24, sampleRate, true);
  view.setUint32(28, sampleRate * 2, true);
  view.setUint16(32, 2, true);
  view.setUint16(34, 16, true);
  writeString(36, 'data');
  view.setUint32(40, audioData.length * 2, true);
  
  // Convert float to 16-bit PCM
  let offset = 44;
  for (let i = 0; i < audioData.length; i++) {
    const sample = Math.max(-1, Math.min(1, audioData[i]));
    view.setInt16(offset, sample * 0x7FFF, true);
    offset += 2;
  }
  
  return buffer;
}

/**
 * Run all examples
 */
export async function runAllExamples() {
  console.log('🚀 Running all Filecoin AI integration examples...\n');

  try {
    // Example 1: Creative Session Workflow
    console.log('--- Example 1: Creative Session Workflow ---');
    await exampleCreativeSessionWorkflow();
    console.log('\n');

    // Example 2: Similarity Search
    console.log('--- Example 2: Similarity Search ---');
    await exampleSimilaritySearch();
    console.log('\n');

    // Example 3: Creative Recommendations
    console.log('--- Example 3: Creative Recommendations ---');
    await exampleCreativeRecommendations();
    console.log('\n');

    // Example 4: Real-time Monitoring (runs for 30 seconds)
    console.log('--- Example 4: Real-time Monitoring ---');
    await exampleRealTimeMonitoring();
    console.log('\n');

    // Example 5: Batch Processing
    console.log('--- Example 5: Batch Processing ---');
    await exampleBatchProcessing();
    console.log('\n');

    console.log('✅ All examples completed successfully!');

  } catch (error) {
    console.error('❌ Examples failed:', error);
  }
}

// Export for use in other modules
export {
  exampleCreativeSessionWorkflow,
  exampleSimilaritySearch,
  exampleCreativeRecommendations,
  exampleRealTimeMonitoring,
  exampleBatchProcessing,
  runAllExamples
};
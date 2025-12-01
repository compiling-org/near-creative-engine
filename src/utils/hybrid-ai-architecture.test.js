// Simple test for hybrid AI architecture
const { HybridAIManager } = require('./hybrid-ai-architecture.js');

async function testHybridAI() {
  console.log('🧠 Testing Hybrid AI Architecture...');
  
  try {
    const aiManager = new HybridAIManager();
    
    console.log('✅ HybridAIManager created successfully');
    
    // Initialize the AI manager
    await aiManager.initialize();
    console.log('✅ Hybrid AI initialized');
    
    // Test biometric stream processing
    console.log('🎭 Testing biometric stream processing...');
    const streamData = [
      {
        eeg: { alpha: 0.5, beta: 0.3, theta: 0.2, delta: 0.1 },
        audio: { frequency: 440, amplitude: 0.8 },
        timestamp: Date.now()
      },
      {
        eeg: { alpha: 0.6, beta: 0.4, theta: 0.3, delta: 0.2 },
        audio: { frequency: 880, amplitude: 0.6 },
        timestamp: Date.now() + 1000
      }
    ];
    
    const result = await aiManager.processBiometricData(streamData);
    console.log('✅ Biometric data processed:', {
      emotions: result.emotions.length,
      biometric_hash: result.biometric_hash,
      timestamp: result.processing_timestamp
    });
    
    console.log('🎉 All AI tests passed successfully!');
    
  } catch (error) {
    console.error('❌ AI test failed:', error);
    process.exit(1);
  }
}

// Run the test
testHybridAI();
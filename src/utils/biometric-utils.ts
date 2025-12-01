// Utility functions for biometric data processing

/**
 * Generate a biometric hash from input data
 * This simulates real biometric processing - in production, this would use
 * actual biometric sensors and processing algorithms
 */
export async function generateBiometricHash(data: string): Promise<Uint8Array> {
  // Simulate biometric processing by creating a hash from the input
  const encoder = new TextEncoder();
  const dataBuffer = encoder.encode(data);
  
  // Simple hash function (in production, use proper cryptographic hashing)
  const hash = new Uint8Array(32);
  for (let i = 0; i < dataBuffer.length; i++) {
    hash[i % 32] = (hash[i % 32] + dataBuffer[i]) % 256;
  }
  
  // Add some entropy to simulate biometric variability
  const timestamp = Date.now();
  const entropy = new Uint8Array([
    (timestamp >> 24) & 0xFF,
    (timestamp >> 16) & 0xFF,
    (timestamp >> 8) & 0xFF,
    timestamp & 0xFF
  ]);
  
  for (let i = 0; i < 4; i++) {
    hash[i * 8] = (hash[i * 8] + entropy[i]) % 256;
  }
  
  return hash;
}

/**
 * Process EEG data to extract emotional features
 * This is a simulation - real implementation would use signal processing
 */
export function processEEGData(_eegData: number[]): {
  alphaPower: number;
  betaPower: number;
  thetaPower: number;
  emotionalState: {
    valence: number;
    arousal: number;
    dominance: number;
  };
} {
  // Simulate EEG frequency analysis
  const alphaPower = Math.random() * 100 + 50; // 8-12 Hz
  const betaPower = Math.random() * 80 + 30;  // 13-30 Hz
  const thetaPower = Math.random() * 60 + 20;  // 4-7 Hz
  
  // Simple emotional state estimation (real implementation would be much more complex)
  const valence = (alphaPower - 50) / 100 * 2 - 1; // -1 to 1
  const arousal = (betaPower - 30) / 80 * 2 - 1;   // -1 to 1
  const dominance = (thetaPower - 20) / 60 * 2 - 1; // -1 to 1
  
  return {
    alphaPower,
    betaPower,
    thetaPower,
    emotionalState: {
      valence: Math.max(-1, Math.min(1, valence)),
      arousal: Math.max(-1, Math.min(1, arousal)),
      dominance: Math.max(-1, Math.min(1, dominance))
    }
  };
}

/**
 * Process heart rate variability data
 */
export function processHeartRateData(heartRateData: number[]): {
  meanHR: number;
  hrv: number;
  stressLevel: number;
} {
  const meanHR = heartRateData.reduce((sum, hr) => sum + hr, 0) / heartRateData.length;
  
  // Calculate HRV (simplified)
  const differences = [];
  for (let i = 1; i < heartRateData.length; i++) {
    differences.push(Math.abs(heartRateData[i] - heartRateData[i-1]));
  }
  const hrv = differences.reduce((sum, diff) => sum + diff, 0) / differences.length;
  
  // Estimate stress level (0-100)
  const stressLevel = Math.min(100, Math.max(0, (hrv - 10) * 2));
  
  return {
    meanHR,
    hrv,
    stressLevel
  };
}

/**
 * Generate a unique biometric signature combining multiple data sources
 */
export async function generateBiometricSignature(
  eegData: number[],
  heartRateData: number[],
  additionalData?: string
): Promise<{
  hash: Uint8Array;
  emotionalState: {
    valence: number;
    arousal: number;
    dominance: number;
  };
  confidence: number;
}> {
  const eegResults = processEEGData(eegData);
  const hrResults = processHeartRateData(heartRateData);
  
  // Combine all data for hashing
  const combinedData = JSON.stringify({
    eeg: eegResults,
    heartRate: hrResults,
    additional: additionalData,
    timestamp: Date.now()
  });
  
  const hash = await generateBiometricHash(combinedData);
  
  // Calculate confidence based on data quality
  const confidence = Math.min(100, Math.max(0, 
    (eegResults.alphaPower + hrResults.hrv) / 2
  ));
  
  return {
    hash,
    emotionalState: eegResults.emotionalState,
    confidence
  };
}

/**
 * Validate biometric data quality
 */
export function validateBiometricData(data: {
  eeg?: number[];
  heartRate?: number[];
  additional?: string;
}): {
  isValid: boolean;
  issues: string[];
  qualityScore: number;
} {
  const issues: string[] = [];
  let qualityScore = 100;
  
  if (data.eeg) {
    if (data.eeg.length < 100) {
      issues.push('EEG data too short (minimum 100 samples required)');
      qualityScore -= 20;
    }
    if (data.eeg.some(sample => Math.abs(sample) > 200)) {
      issues.push('EEG data contains outliers');
      qualityScore -= 15;
    }
  }
  
  if (data.heartRate) {
    if (data.heartRate.length < 30) {
      issues.push('Heart rate data too short (minimum 30 samples required)');
      qualityScore -= 20;
    }
    if (data.heartRate.some(hr => hr < 40 || hr > 200)) {
      issues.push('Heart rate data contains unrealistic values');
      qualityScore -= 15;
    }
  }
  
  if (!data.eeg && !data.heartRate) {
    issues.push('No biometric data provided');
    qualityScore -= 50;
  }
  
  return {
    isValid: qualityScore >= 60,
    issues,
    qualityScore: Math.max(0, qualityScore)
  };
}
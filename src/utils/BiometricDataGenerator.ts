// Realistic biometric data generator for testing biometric integration
// Based on actual EEG patterns, gesture recognition, and audio analysis

export interface EEGPattern {
  delta: number;    // 0.5-4 Hz - deep sleep
  theta: number;    // 4-8 Hz - drowsiness, meditation
  alpha: number;    // 8-13 Hz - relaxed awareness
  beta: number;     // 13-30 Hz - active thinking
  gamma: number;    // 30-100 Hz - higher mental activity
}

export interface GesturePattern {
  handPosition: { x: number; y: number; z: number };
  handRotation: { pitch: number; yaw: number; roll: number };
  gestureType: 'pointing' | 'swiping' | 'grabbing' | 'rotating' | 'idle';
  confidence: number;
  velocity: { x: number; y: number; z: number };
}

export interface AudioPattern {
  fundamentalFreq: number;
  amplitude: number;
  emotion: 'calm' | 'excited' | 'angry' | 'sad' | 'happy' | 'neutral';
  confidence: number;
  spectralCentroid: number;
  zeroCrossingRate: number;
}

export interface BiometricSample {
  timestamp: number;
  eeg: EEGPattern;
  gesture: GesturePattern;
  audio: AudioPattern;
  attention: number;      // 0-100 (MindWave-like)
  meditation: number;    // 0-100 (MindWave-like)
  signalQuality: number; // 0-1
  emotionalState: {
    valence: number;     // -1 to 1 (positive/negative)
    arousal: number;     // -1 to 1 (energy level)
    dominance: number;   // 0 to 1 (control level)
  };
}

export class BiometricDataGenerator {
  private baseFrequency: number;
  private sampleRate: number;
  private time: number;
  private emotionalState: string;
  private gestureState: string;
  private noiseLevel: number;
  
  constructor(sampleRate: number = 256) {
    this.baseFrequency = sampleRate;
    this.sampleRate = sampleRate;
    this.time = 0;
    this.emotionalState = 'neutral';
    this.gestureState = 'idle';
    this.noiseLevel = 0.1;
  }
  
  // Generate realistic EEG patterns based on mental states
  generateEEGPattern(state: string = 'neutral'): EEGPattern {
    const patterns: Record<string, EEGPattern> = {
      'focused': {
        delta: 0.1 + Math.random() * 0.1,
        theta: 0.2 + Math.random() * 0.1,
        alpha: 0.3 + Math.random() * 0.2,
        beta: 0.8 + Math.random() * 0.2,  // High beta for focus
        gamma: 0.4 + Math.random() * 0.2
      },
      'relaxed': {
        delta: 0.2 + Math.random() * 0.1,
        theta: 0.6 + Math.random() * 0.2,  // High theta for relaxation
        alpha: 0.9 + Math.random() * 0.1,  // High alpha for relaxed awareness
        beta: 0.3 + Math.random() * 0.1,
        gamma: 0.2 + Math.random() * 0.1
      },
      'meditation': {
        delta: 0.3 + Math.random() * 0.2,
        theta: 0.8 + Math.random() * 0.2,  // Very high theta
        alpha: 0.7 + Math.random() * 0.2,
        beta: 0.2 + Math.random() * 0.1,
        gamma: 0.1 + Math.random() * 0.1
      },
      'excited': {
        delta: 0.1 + Math.random() * 0.1,
        theta: 0.2 + Math.random() * 0.1,
        alpha: 0.4 + Math.random() * 0.2,
        beta: 0.9 + Math.random() * 0.1,  // Very high beta
        gamma: 0.8 + Math.random() * 0.2   // High gamma for excitement
      },
      'neutral': {
        delta: 0.2 + Math.random() * 0.1,
        theta: 0.3 + Math.random() * 0.2,
        alpha: 0.5 + Math.random() * 0.2,
        beta: 0.5 + Math.random() * 0.2,
        gamma: 0.3 + Math.random() * 0.2
      }
    };
    
    return patterns[state] || patterns['neutral'];
  }
  
  // Generate realistic gesture patterns
  generateGesturePattern(state: string = 'idle'): GesturePattern {
    const time = this.time * 0.001;
    
    const patterns: Record<string, () => GesturePattern> = {
      'idle': () => ({
        handPosition: {
          x: Math.sin(time * 0.5) * 10 + Math.random() * 5,
          y: Math.cos(time * 0.3) * 8 + Math.random() * 3,
          z: Math.sin(time * 0.7) * 5 + Math.random() * 2
        },
        handRotation: {
          pitch: Math.sin(time * 0.4) * 5,
          yaw: Math.cos(time * 0.6) * 3,
          roll: Math.sin(time * 0.8) * 2
        },
        gestureType: 'idle',
        confidence: 0.95,
        velocity: { x: 0, y: 0, z: 0 }
      }),
      
      'pointing': () => ({
        handPosition: {
          x: Math.sin(time * 0.8) * 50 + 100,
          y: Math.cos(time * 0.6) * 30 + 80,
          z: Math.sin(time * 1.2) * 40 + 60
        },
        handRotation: {
          pitch: -20 + Math.sin(time * 0.9) * 10,
          yaw: Math.cos(time * 0.7) * 15,
          roll: Math.sin(time * 1.1) * 5
        },
        gestureType: 'pointing',
        confidence: 0.88,
        velocity: {
          x: Math.sin(time * 0.8) * 20,
          y: Math.cos(time * 0.6) * 15,
          z: Math.sin(time * 1.2) * 18
        }
      }),
      
      'swiping': () => ({
        handPosition: {
          x: Math.sin(time * 2.5) * 120,
          y: 50 + Math.cos(time * 1.8) * 20,
          z: 30 + Math.sin(time * 2.2) * 15
        },
        handRotation: {
          pitch: Math.sin(time * 2.8) * 25,
          yaw: Math.cos(time * 2.1) * 30,
          roll: Math.sin(time * 3.2) * 10
        },
        gestureType: 'swiping',
        confidence: 0.82,
        velocity: {
          x: Math.cos(time * 2.5) * 80,
          y: -Math.sin(time * 1.8) * 25,
          z: Math.cos(time * 2.2) * 20
        }
      }),
      
      'grabbing': () => ({
        handPosition: {
          x: Math.sin(time * 1.2) * 30 + 60,
          y: Math.cos(time * 0.9) * 25 + 70,
          z: Math.sin(time * 1.5) * 35 + 50
        },
        handRotation: {
          pitch: Math.sin(time * 1.3) * 15 + 10,
          yaw: Math.cos(time * 1.1) * 20,
          roll: Math.sin(time * 1.6) * 8
        },
        gestureType: 'grabbing',
        confidence: 0.79,
        velocity: {
          x: Math.sin(time * 1.2) * 15,
          y: Math.cos(time * 0.9) * 12,
          z: Math.sin(time * 1.5) * 18
        }
      })
    };
    
    const generator = patterns[state] || patterns['idle'];
    return generator();
  }
  
  // Generate realistic audio patterns
  generateAudioPattern(emotion: string = 'neutral'): AudioPattern {
    const time = this.time * 0.001;
    
    const patterns: Record<string, () => AudioPattern> = {
      'calm': () => ({
        fundamentalFreq: 120 + Math.sin(time * 0.5) * 20 + Math.random() * 10,
        amplitude: 0.3 + Math.sin(time * 0.3) * 0.1 + Math.random() * 0.05,
        emotion: 'calm',
        confidence: 0.85,
        spectralCentroid: 800 + Math.sin(time * 0.4) * 100,
        zeroCrossingRate: 0.05 + Math.sin(time * 0.6) * 0.02
      }),
      
      'excited': () => ({
        fundamentalFreq: 180 + Math.sin(time * 2.5) * 40 + Math.random() * 20,
        amplitude: 0.8 + Math.sin(time * 3.2) * 0.2 + Math.random() * 0.1,
        emotion: 'excited',
        confidence: 0.78,
        spectralCentroid: 1500 + Math.sin(time * 2.8) * 300,
        zeroCrossingRate: 0.15 + Math.sin(time * 3.1) * 0.05
      }),
      
      'angry': () => ({
        fundamentalFreq: 140 + Math.sin(time * 1.8) * 30 + Math.random() * 15,
        amplitude: 0.9 + Math.sin(time * 2.1) * 0.15 + Math.random() * 0.08,
        emotion: 'angry',
        confidence: 0.82,
        spectralCentroid: 1200 + Math.sin(time * 2.3) * 250,
        zeroCrossingRate: 0.12 + Math.sin(time * 1.9) * 0.04
      }),
      
      'sad': () => ({
        fundamentalFreq: 100 + Math.sin(time * 0.8) * 15 + Math.random() * 8,
        amplitude: 0.4 + Math.sin(time * 0.6) * 0.1 + Math.random() * 0.05,
        emotion: 'sad',
        confidence: 0.88,
        spectralCentroid: 600 + Math.sin(time * 0.7) * 80,
        zeroCrossingRate: 0.03 + Math.sin(time * 0.5) * 0.015
      }),
      
      'happy': () => ({
        fundamentalFreq: 160 + Math.sin(time * 1.5) * 35 + Math.random() * 18,
        amplitude: 0.7 + Math.sin(time * 2.8) * 0.25 + Math.random() * 0.12,
        emotion: 'happy',
        confidence: 0.84,
        spectralCentroid: 1100 + Math.sin(time * 1.8) * 200,
        zeroCrossingRate: 0.08 + Math.sin(time * 2.2) * 0.03
      }),
      
      'neutral': () => ({
        fundamentalFreq: 130 + Math.sin(time * 0.9) * 25 + Math.random() * 12,
        amplitude: 0.5 + Math.sin(time * 1.1) * 0.15 + Math.random() * 0.08,
        emotion: 'neutral',
        confidence: 0.9,
        spectralCentroid: 900 + Math.sin(time * 0.8) * 150,
        zeroCrossingRate: 0.06 + Math.sin(time * 1.0) * 0.02
      })
    };
    
    const generator = patterns[emotion] || patterns['neutral'];
    return generator();
  }
  
  // Calculate attention index (MindWave-like) based on beta/theta ratio
  calculateAttention(eeg: EEGPattern): number {
    const betaThetaRatio = eeg.beta / (eeg.theta + 0.01); // Avoid division by zero
    const attention = 50 + (betaThetaRatio - 1) * 25;
    return Math.max(0, Math.min(100, attention));
  }
  
  // Calculate meditation index (MindWave-like) based on alpha power
  calculateMeditation(eeg: EEGPattern): number {
    const meditation = eeg.alpha * 100;
    return Math.max(0, Math.min(100, meditation));
  }
  
  // Calculate emotional state using Valence-Arousal-Dominance model
  calculateEmotionalState(eeg: EEGPattern, gesture: GesturePattern, audio: AudioPattern) {
    // EEG-based emotional indicators
    const eegValence = (eeg.alpha - eeg.beta) / (eeg.alpha + eeg.beta + 0.01);
    const eegArousal = (eeg.beta + eeg.gamma) / (eeg.theta + eeg.alpha + 0.01);
    
    // Gesture-based emotional indicators
    const gestureEnergy = Math.sqrt(
      gesture.velocity.x ** 2 + gesture.velocity.y ** 2 + gesture.velocity.z ** 2
    );
    const gestureArousal = Math.min(1, gestureEnergy / 100);
    
    // Audio-based emotional indicators
    const audioArousal = audio.amplitude;
    const audioValence = this.getAudioValence(audio.emotion);
    
    // Combined emotional state
    const valence = (eegValence * 0.4 + audioValence * 0.6);
    const arousal = (eegArousal * 0.3 + gestureArousal * 0.3 + audioArousal * 0.4);
    const dominance = 0.5 + (eeg.alpha - eeg.theta) * 0.5; // Alpha dominance indicates control
    
    return {
      valence: Math.max(-1, Math.min(1, valence)),
      arousal: Math.max(-1, Math.min(1, arousal)),
      dominance: Math.max(0, Math.min(1, dominance))
    };
  }
  
  private getAudioValence(emotion: string): number {
    const valenceMap: Record<string, number> = {
      'happy': 0.8,
      'excited': 0.6,
      'calm': 0.3,
      'neutral': 0.0,
      'sad': -0.6,
      'angry': -0.8
    };
    return valenceMap[emotion] || 0.0;
  }
  
  // Calculate signal quality based on multiple metrics
  calculateSignalQuality(eeg: EEGPattern, gesture: GesturePattern, audio: AudioPattern): number {
    // EEG quality metrics
    const eegTotalPower = eeg.delta + eeg.theta + eeg.alpha + eeg.beta + eeg.gamma;
    const eegBalance = 1 - Math.abs(eegTotalPower - 1); // Should sum to ~1
    
    // Gesture quality metrics
    const gestureQuality = gesture.confidence;
    
    // Audio quality metrics
    const audioQuality = audio.confidence;
    
    // Combined quality score
    const quality = (eegBalance * 0.4 + gestureQuality * 0.3 + audioQuality * 0.3);
    return Math.max(0, Math.min(1, quality));
  }
  
  // Generate a complete biometric sample
  generateSample(state?: string): BiometricSample {
    this.time += 1000 / this.sampleRate; // Advance time
    
    const eeg = this.generateEEGPattern(state || this.emotionalState);
    const gesture = this.generateGesturePattern(this.gestureState);
    const audio = this.generateAudioPattern(this.emotionalState);
    
    const attention = this.calculateAttention(eeg);
    const meditation = this.calculateMeditation(eeg);
    const emotionalState = this.calculateEmotionalState(eeg, gesture, audio);
    const signalQuality = this.calculateSignalQuality(eeg, gesture, audio);
    
    return {
      timestamp: Date.now(),
      eeg,
      gesture,
      audio,
      attention,
      meditation,
      signalQuality,
      emotionalState
    };
  }
  
  // Generate time-series EEG data (like MindWave output)
  generateEEGTimeSeries(duration: number = 1.0, state?: string): Float32Array {
    const samples = Math.floor(duration * this.sampleRate);
    const data = new Float32Array(samples);
    
    const eeg = this.generateEEGPattern(state || this.emotionalState);
    
    for (let i = 0; i < samples; i++) {
      const t = i / this.sampleRate;
      
      // Generate realistic EEG signal with multiple frequency components
      const deltaWave = Math.sin(2 * Math.PI * 2 * t) * eeg.delta * 50;
      const thetaWave = Math.sin(2 * Math.PI * 6 * t) * eeg.theta * 40;
      const alphaWave = Math.sin(2 * Math.PI * 10 * t) * eeg.alpha * 30;
      const betaWave = Math.sin(2 * Math.PI * 20 * t) * eeg.beta * 25;
      const gammaWave = Math.sin(2 * Math.PI * 40 * t) * eeg.gamma * 15;
      
      // Add some noise and temporal variation
      const noise = (Math.random() - 0.5) * this.noiseLevel * 20;
      const temporalVariation = Math.sin(t * 0.5) * 10;
      
      data[i] = deltaWave + thetaWave + alphaWave + betaWave + gammaWave + noise + temporalVariation;
    }
    
    return data;
  }
  
  // Set emotional state for consistent pattern generation
  setEmotionalState(state: string) {
    this.emotionalState = state;
  }
  
  // Set gesture state
  setGestureState(state: string) {
    this.gestureState = state;
  }
  
  // Set noise level (0-1)
  setNoiseLevel(level: number) {
    this.noiseLevel = Math.max(0, Math.min(1, level));
  }
  
  // Get current generator state
  getState() {
    return {
      emotionalState: this.emotionalState,
      gestureState: this.gestureState,
      noiseLevel: this.noiseLevel,
      time: this.time
    };
  }
}

// Factory function for common use cases
export const createBiometricGenerator = (type: 'meditation' | 'focus' | 'excited' | 'calm' | 'neutral' = 'neutral') => {
  const generator = new BiometricDataGenerator();
  
  switch (type) {
    case 'meditation':
      generator.setEmotionalState('meditation');
      generator.setGestureState('idle');
      generator.setNoiseLevel(0.05);
      break;
    case 'focus':
      generator.setEmotionalState('focused');
      generator.setGestureState('pointing');
      generator.setNoiseLevel(0.08);
      break;
    case 'excited':
      generator.setEmotionalState('excited');
      generator.setGestureState('swiping');
      generator.setNoiseLevel(0.12);
      break;
    case 'calm':
      generator.setEmotionalState('relaxed');
      generator.setGestureState('idle');
      generator.setNoiseLevel(0.06);
      break;
    default:
      generator.setEmotionalState('neutral');
      generator.setGestureState('idle');
      generator.setNoiseLevel(0.1);
  }
  
  return generator;
};

// Real-time biometric data stream generator
export class BiometricDataStream {
  private generator: BiometricDataGenerator;
  private intervalId: NodeJS.Timeout | null = null;
  private callbacks: ((sample: BiometricSample) => void)[] = [];
  private sampleRate: number;
  
  constructor(sampleRate: number = 30) { // 30 Hz update rate
    this.generator = new BiometricDataGenerator();
    this.sampleRate = sampleRate;
  }
  
  // Start the biometric data stream
  start() {
    if (this.intervalId) return; // Already running
    
    const interval = 1000 / this.sampleRate;
    
    this.intervalId = setInterval(() => {
      const sample = this.generator.generateSample();
      this.callbacks.forEach(callback => callback(sample));
    }, interval);
    
    console.log(`✅ Biometric data stream started at ${this.sampleRate} Hz`);
  }
  
  // Stop the biometric data stream
  stop() {
    if (this.intervalId) {
      clearInterval(this.intervalId);
      this.intervalId = null;
      console.log('✅ Biometric data stream stopped');
    }
  }
  
  // Add callback for new samples
  onSample(callback: (sample: BiometricSample) => void) {
    this.callbacks.push(callback);
  }
  
  // Remove callback
  offSample(callback: (sample: BiometricSample) => void) {
    this.callbacks = this.callbacks.filter(cb => cb !== callback);
  }
  
  // Update generator state
  updateState(updates: Partial<{ emotionalState: string; gestureState: string; noiseLevel: number }>) {
    if (updates.emotionalState) {
      this.generator.setEmotionalState(updates.emotionalState);
    }
    if (updates.gestureState) {
      this.generator.setGestureState(updates.gestureState);
    }
    if (updates.noiseLevel !== undefined) {
      this.generator.setNoiseLevel(updates.noiseLevel);
    }
  }
  
  // Get current stream state
  getState() {
    return {
      isRunning: this.intervalId !== null,
      sampleRate: this.sampleRate,
      generatorState: this.generator.getState(),
      callbackCount: this.callbacks.length
    };
  }
}

export default BiometricDataGenerator;
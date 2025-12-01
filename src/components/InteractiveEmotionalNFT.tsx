import React, { useState, useEffect, useRef } from 'react';
import { crossChainAI } from '../utils/cross-chain-ai-integration';
import { Connection, PublicKey } from '@solana/web3.js';
import { Program, AnchorProvider, web3 } from '@project-serum/anchor';
import * as faceapi from 'face-api.js';
import * as tf from '@tensorflow/tfjs';

interface EmotionalNFTProps {
  className?: string;
  onEmotionDetected?: (emotion: EmotionData) => void;
  onBiometricVerified?: (verified: boolean) => void;
  interactive?: boolean;
  soulbound?: boolean;
}

interface EmotionData {
  type: string;
  intensity: number;
  confidence: number;
  vectorHash: string;
  timestamp: number;
  biometricData?: BiometricData;
}

interface BiometricData {
  facialFeatures: any[];
  fingerprint?: string;
  voicePattern?: string;
  behavioralPattern?: string;
}

interface NFTMetadata {
  name: string;
  description: string;
  image: string;
  attributes: Array<{
    trait_type: string;
    value: string | number;
  }>;
  emotional_history: EmotionData[];
  soulbound_data?: {
    owner: string;
    creation_date: number;
    biometric_hash: string;
    emotional_signature: string;
  };
}

export const InteractiveEmotionalNFT: React.FC<EmotionalNFTProps> = ({
  className = '',
  onEmotionDetected,
  onBiometricVerified,
  interactive = true,
  soulbound = false
}) => {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isProcessing, setIsProcessing] = useState(false);
  const [currentEmotion, setCurrentEmotion] = useState<EmotionData | null>(null);
  const [emotionalHistory, setEmotionalHistory] = useState<EmotionData[]>([]);
  const [biometricVerified, setBiometricVerified] = useState(false);
  const [nftMetadata, setNFTMetadata] = useState<NFTMetadata | null>(null);
  const [isMinting, setIsMinting] = useState(false);
  
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const animationRef = useRef<number | null>(null);

  // Initialize AI models and cross-chain integration
  useEffect(() => {
    const initialize = async () => {
      try {
        console.log('🚀 Initializing Interactive Emotional NFT System...');
        
        // Load face-api.js models
        await loadFaceAPIModels();
        
        // Initialize cross-chain AI integration
        await crossChainAI.initialize();
        
        // Load or create NFT metadata
        await loadNFTMetadata();
        
        setIsInitialized(true);
        console.log('✅ Interactive Emotional NFT System initialized!');
        
      } catch (error) {
        console.error('❌ Failed to initialize Interactive Emotional NFT:', error);
      }
    };

    initialize();

    return () => {
      cleanup();
    };
  }, []);

  const loadFaceAPIModels = async () => {
    const MODEL_URL = '/models';
    
    await faceapi.nets.tinyFaceDetector.loadFromUri(MODEL_URL);
    await faceapi.nets.faceLandmark68Net.loadFromUri(MODEL_URL);
    await faceapi.nets.faceRecognitionNet.loadFromUri(MODEL_URL);
    await faceapi.nets.faceExpressionNet.loadFromUri(MODEL_URL);
    
    console.log('✅ Face API models loaded');
  };

  const loadNFTMetadata = async () => {
    // Load existing metadata or create new one
    const metadata: NFTMetadata = {
      name: "Interactive Emotional NFT",
      description: "An NFT that responds to and records emotional interactions",
      image: "https://ipfs.io/ipfs/QmYbEe8fkyJz3Domaxj1goXAKf5epFndDVP7bgpbcZJhh5",
      attributes: [
        { trait_type: "Type", value: "Emotional" },
        { trait_type: "Interactive", value: interactive ? "Yes" : "No" },
        { trait_type: "Soulbound", value: soulbound ? "Yes" : "No" },
        { trait_type: "AI_Powered", value: "Yes" },
        { trait_type: "Cross_Chain", value: "Filecoin+NEAR+Solana" }
      ],
      emotional_history: [],
      soulbound_data: soulbound ? {
        owner: "",
        creation_date: Date.now(),
        biometric_hash: "",
        emotional_signature: ""
      } : undefined
    };
    
    setNFTMetadata(metadata);
  };

  const startEmotionDetection = async () => {
    if (!isInitialized || !interactive) return;
    
    try {
      setIsProcessing(true);
      
      // Request camera access
      const stream = await navigator.mediaDevices.getUserMedia({ 
        video: { width: 640, height: 480 }, 
        audio: false 
      });
      
      streamRef.current = stream;
      
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
        await videoRef.current.play();
      }
      
      // Start emotion detection loop
      detectEmotions();
      
    } catch (error) {
      console.error('❌ Failed to start emotion detection:', error);
      setIsProcessing(false);
    }
  };

  const detectEmotions = async () => {
    if (!videoRef.current || !canvasRef.current) return;
    
    const canvas = canvasRef.current;
    const ctx = canvas.getContext('2d');
    const video = videoRef.current;
    
    // Set canvas size to match video
    canvas.width = video.videoWidth;
    canvas.height = video.videoHeight;
    
    const detect = async () => {
      try {
        // Draw current video frame to canvas
        ctx.drawImage(video, 0, 0);
        
        // Detect faces and emotions
        const detections = await faceapi
          .detectAllFaces(video, new faceapi.TinyFaceDetectorOptions())
          .withFaceLandmarks()
          .withFaceExpressions();
        
        if (detections.length > 0) {
          const face = detections[0];
          const expressions = face.expressions;
          
          // Get dominant emotion
          const dominantEmotion = Object.keys(expressions).reduce((a, b) => 
            expressions[a] > expressions[b] ? a : b
          );
          
          const confidence = expressions[dominantEmotion];
          const intensity = Math.round(confidence * 100);
          
          // Process with cross-chain AI for real inference
          const aiResult = await crossChainAI.processAIData(
            `nft_${Date.now()}`,
            'emotion',
            { imageData: canvas, useBiometric: soulbound },
            { streamId: 'emotional-nft-stream' }
          );
          
          const emotionData: EmotionData = {
            type: dominantEmotion,
            intensity,
            confidence: Math.round(confidence * 100),
            vectorHash: aiResult.vectorHash,
            timestamp: Date.now(),
            biometricData: aiResult.biometricData
          };
          
          setCurrentEmotion(emotionData);
          setEmotionalHistory(prev => [...prev, emotionData]);
          
          // Update NFT metadata
          await updateNFTMetadata(emotionData);
          
          // Callback to parent component
          if (onEmotionDetected) {
            onEmotionDetected(emotionData);
          }
          
          // Verify biometric data for soulbound NFTs
          if (soulbound && emotionData.biometricData) {
            await verifyBiometricData(emotionData.biometricData);
          }
          
          // Draw emotion visualization
          drawEmotionVisualization(ctx, face, emotionData);
        }
        
      } catch (error) {
        console.error('❌ Emotion detection error:', error);
      }
      
      // Continue detection loop
      animationRef.current = requestAnimationFrame(detect);
    };
    
    detect();
  };

  const verifyBiometricData = async (biometricData: BiometricData) => {
    try {
      const verificationResult = await crossChainAI.processAIData(
        `biometric_${Date.now()}`,
        'biometric',
        { biometricData, userId: 'nft_owner' },
        { streamId: 'biometric-verification-stream' }
      );
      
      const isVerified = verificationResult.prediction === 'authenticated';
      setBiometricVerified(isVerified);
      
      if (onBiometricVerified) {
        onBiometricVerified(isVerified);
      }
      
      console.log(`🔐 Biometric verification: ${isVerified ? 'PASSED' : 'FAILED'}`);
      
    } catch (error) {
      console.error('❌ Biometric verification failed:', error);
    }
  };

  const updateNFTMetadata = async (emotionData: EmotionData) => {
    if (!nftMetadata) return;
    
    try {
      // Update emotional history
      const updatedMetadata = {
        ...nftMetadata,
        emotional_history: [...emotionalHistory, emotionData]
      };
      
      // Store emotional metadata on-chain
      await crossChainAI.storeEmotionalMetadata(
        'emotional-nft-stream',
        {
          emotionType: emotionData.type,
          intensity: emotionData.intensity,
          vectorHash: emotionData.vectorHash,
          tags: ['nft', 'interactive', 'emotional']
        }
      );
      
      setNFTMetadata(updatedMetadata);
      
      // Sync across chains if enabled
      if (crossChainAI.activeStreams.size > 0) {
        await crossChainAI.synchronizeCrossChain('emotional-nft-stream', ['near', 'solana']);
      }
      
    } catch (error) {
      console.error('❌ Failed to update NFT metadata:', error);
    }
  };

  const drawEmotionVisualization = (ctx: CanvasRenderingContext2D, face: any, emotionData: EmotionData) => {
    const { x, y, width, height } = face.detection.box;
    
    // Draw bounding box with emotion color
    const emotionColors = {
      angry: '#FF4444',
      disgust: '#8B4513',
      fear: '#800080',
      happy: '#FFD700',
      sad: '#4169E1',
      surprise: '#FF69B4',
      neutral: '#808080'
    };
    
    ctx.strokeStyle = emotionColors[emotionData.type] || '#808080';
    ctx.lineWidth = 3;
    ctx.strokeRect(x, y, width, height);
    
    // Draw emotion label
    ctx.fillStyle = emotionColors[emotionData.type] || '#808080';
    ctx.font = '16px Arial';
    ctx.fillText(
      `${emotionData.type.toUpperCase()} (${emotionData.intensity}%)`,
      x,
      y - 10
    );
    
    // Draw intensity bar
    const barWidth = width;
    const barHeight = 10;
    const barY = y + height + 10;
    
    // Background
    ctx.fillStyle = '#333333';
    ctx.fillRect(x, barY, barWidth, barHeight);
    
    // Intensity fill
    ctx.fillStyle = emotionColors[emotionData.type] || '#808080';
    ctx.fillRect(x, barY, (barWidth * emotionData.intensity) / 100, barHeight);
    
    // Draw facial landmarks if available
    if (face.landmarks) {
      ctx.fillStyle = '#00FF00';
      face.landmarks.positions.forEach((point: any) => {
        ctx.beginPath();
        ctx.arc(point.x, point.y, 2, 0, 2 * Math.PI);
        ctx.fill();
      });
    }
  };

  const mintEmotionalNFT = async () => {
    if (!nftMetadata || isMinting) return;
    
    try {
      setIsMinting(true);
      
      console.log('🎨 Minting Interactive Emotional NFT...');
      
      // Create cross-chain data stream for NFT metadata
      const streamId = await crossChainAI.createDataStream(
        'near', // source chain
        'solana', // target chain
        {
          metadata: nftMetadata,
          emotional_history: emotionalHistory,
          soulbound_data: nftMetadata.soulbound_data
        },
        {
          type: 'interactive_nft',
          emotional: true,
          biometric: soulbound
        }
      );
      
      // Store on IPFS
      const ipfsHash = await crossChainAI.uploadToIPFS({
        metadata: nftMetadata,
        emotional_history: emotionalHistory
      });
      
      console.log(`✅ Emotional NFT minted with stream ID: ${streamId}`);
      console.log(`📁 Metadata stored on IPFS: ${ipfsHash}`);
      
      // Update NFT with minting information
      const updatedMetadata = {
        ...nftMetadata,
        attributes: [
          ...nftMetadata.attributes,
          { trait_type: "Stream_ID", value: streamId },
          { trait_type: "IPFS_Hash", value: ipfsHash },
          { trait_type: "Minted", value: "Yes" },
          { trait_type: "Cross_Chain_ID", value: streamId }
        ]
      };
      
      setNFTMetadata(updatedMetadata);
      
    } catch (error) {
      console.error('❌ Failed to mint Emotional NFT:', error);
    } finally {
      setIsMinting(false);
    }
  };

  const stopEmotionDetection = () => {
    if (animationRef.current) {
      cancelAnimationFrame(animationRef.current);
      animationRef.current = null;
    }
    
    if (streamRef.current) {
      streamRef.current.getTracks().forEach(track => track.stop());
      streamRef.current = null;
    }
    
    if (videoRef.current) {
      videoRef.current.srcObject = null;
    }
    
    setIsProcessing(false);
  };

  const cleanup = () => {
    stopEmotionDetection();
  };

  const getEmotionColor = (emotion: string) => {
    const colors = {
      angry: 'bg-red-500',
      disgust: 'bg-amber-700',
      fear: 'bg-purple-600',
      happy: 'bg-yellow-400',
      sad: 'bg-blue-600',
      surprise: 'bg-pink-500',
      neutral: 'bg-gray-500'
    };
    return colors[emotion] || 'bg-gray-500';
  };

  if (!isInitialized) {
    return (
      <div className={`flex items-center justify-center p-8 ${className}`}>
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
          <p className="text-gray-600">Initializing Interactive Emotional NFT...</p>
        </div>
      </div>
    );
  }

  return (
    <div className={`bg-gradient-to-br from-purple-900 via-blue-900 to-indigo-900 rounded-xl p-6 shadow-2xl ${className}`}>
      <div className="text-center mb-6">
        <h2 className="text-2xl font-bold text-white mb-2">Interactive Emotional NFT</h2>
        <p className="text-purple-200">Experience AI-powered emotional interaction</p>
      </div>

      {/* Video and Canvas for Emotion Detection */}
      <div className="relative mb-6 rounded-lg overflow-hidden">
        <video
          ref={videoRef}
          className="w-full h-64 object-cover"
          autoPlay
          muted
          playsInline
          style={{ display: isProcessing ? 'block' : 'none' }}
        />
        <canvas
          ref={canvasRef}
          className="w-full h-64"
          style={{ display: isProcessing ? 'block' : 'none' }}
        />
        
        {!isProcessing && (
          <div className="w-full h-64 bg-gray-800 flex items-center justify-center">
            <div className="text-center text-gray-400">
              <div className="text-6xl mb-4">🎭</div>
              <p>Click "Start Interaction" to begin emotional detection</p>
            </div>
          </div>
        )}
      </div>

      {/* Current Emotion Display */}
      {currentEmotion && (
        <div className="mb-6 p-4 bg-black bg-opacity-30 rounded-lg">
          <h3 className="text-lg font-semibold text-white mb-2">Current Emotion</h3>
          <div className="flex items-center space-x-4">
            <div className={`w-16 h-16 rounded-full ${getEmotionColor(currentEmotion.type)} flex items-center justify-center text-2xl`}>
              {getEmotionEmoji(currentEmotion.type)}
            </div>
            <div>
              <p className="text-white font-medium capitalize">{currentEmotion.type}</p>
              <p className="text-purple-200">Intensity: {currentEmotion.intensity}%</p>
              <p className="text-purple-200">Confidence: {currentEmotion.confidence}%</p>
            </div>
          </div>
        </div>
      )}

      {/* Biometric Verification Status */}
      {soulbound && (
        <div className="mb-6 p-4 bg-black bg-opacity-30 rounded-lg">
          <h3 className="text-lg font-semibold text-white mb-2">Biometric Verification</h3>
          <div className="flex items-center space-x-2">
            <div className={`w-4 h-4 rounded-full ${biometricVerified ? 'bg-green-500' : 'bg-red-500'}`}></div>
            <span className="text-white">
              {biometricVerified ? '✅ Verified' : '❌ Not Verified'}
            </span>
          </div>
        </div>
      )}

      {/* Emotional History */}
      {emotionalHistory.length > 0 && (
        <div className="mb-6 p-4 bg-black bg-opacity-30 rounded-lg">
          <h3 className="text-lg font-semibold text-white mb-2">Emotional History</h3>
          <div className="max-h-32 overflow-y-auto space-y-2">
            {emotionalHistory.slice(-5).map((emotion, index) => (
              <div key={index} className="flex items-center justify-between text-sm">
                <div className="flex items-center space-x-2">
                  <span className="text-purple-200">{getEmotionEmoji(emotion.type)}</span>
                  <span className="text-white capitalize">{emotion.type}</span>
                </div>
                <div className="text-purple-200">
                  {emotion.intensity}% • {new Date(emotion.timestamp).toLocaleTimeString()}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Control Buttons */}
      <div className="flex space-x-4">
        {!isProcessing ? (
          <button
            onClick={startEmotionDetection}
            className="flex-1 bg-purple-600 hover:bg-purple-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
          >
            🎭 Start Interaction
          </button>
        ) : (
          <button
            onClick={stopEmotionDetection}
            className="flex-1 bg-red-600 hover:bg-red-700 text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
          >
            ⏹️ Stop Interaction
          </button>
        )}
        
        <button
          onClick={mintEmotionalNFT}
          disabled={isMinting || emotionalHistory.length === 0}
          className="flex-1 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-semibold py-3 px-6 rounded-lg transition-colors duration-200"
        >
          {isMinting ? '🔄 Minting...' : '🎨 Mint NFT'}
        </button>
      </div>

      {/* NFT Metadata Display */}
      {nftMetadata && (
        <div className="mt-6 p-4 bg-black bg-opacity-20 rounded-lg">
          <h4 className="text-md font-semibold text-white mb-2">NFT Properties</h4>
          <div className="grid grid-cols-2 gap-2 text-sm">
            {nftMetadata.attributes.map((attr, index) => (
              <div key={index} className="flex justify-between">
                <span className="text-purple-200">{attr.trait_type}:</span>
                <span className="text-white">{attr.value}</span>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
};

// Helper functions
const getEmotionEmoji = (emotion: string) => {
  const emojis = {
    angry: '😠',
    disgust: '🤢',
    fear: '😨',
    happy: '😊',
    sad: '😢',
    surprise: '😲',
    neutral: '😐'
  };
  return emojis[emotion] || '😐';
};

export default InteractiveEmotionalNFT;
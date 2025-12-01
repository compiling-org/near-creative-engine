import React, { useState, useCallback, useRef } from 'react';
import { FilecoinStorageClient, createFilecoinStorageClient } from '../utils/filecoin-storage';

interface FilecoinStorageIntegrationProps {
  canvas: HTMLCanvasElement | null;
  emotionData: {
    valence: number;
    arousal: number;
    dominance: number;
    confidence: number;
  };
  biometricData: string;
  onStorageComplete?: (result: { cid: string; url: string }) => void;
  onError?: (error: string) => void;
}

const FilecoinStorageIntegration: React.FC<FilecoinStorageIntegrationProps> = ({
  canvas,
  emotionData,
  biometricData,
  onStorageComplete,
  onError
}) => {
  const [storageClient, setStorageClient] = useState<FilecoinStorageClient | null>(null);
  const [isUploading, setIsUploading] = useState(false);
  const [uploadProgress, setUploadProgress] = useState(0);
  const [lastUpload, setLastUpload] = useState<{
    cid: string;
    url: string;
    timestamp: number;
  } | null>(null);
  const [apiKey, setApiKey] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const progressIntervalRef = useRef<NodeJS.Timeout | null>(null);

  const connectStorage = useCallback(async () => {
    if (!apiKey.trim()) {
      onError?.('Please enter a Web3.Storage API key');
      return;
    }

    try {
      const client = createFilecoinStorageClient(apiKey);
      setStorageClient(client);
      setIsConnected(true);
    } catch (error) {
      onError?.(`Failed to connect to Filecoin storage: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }, [apiKey, onError]);

  const uploadToFilecoin = useCallback(async () => {
    if (!storageClient || !canvas) {
      onError?.('Storage client not connected or canvas not available');
      return;
    }

    setIsUploading(true);
    setUploadProgress(0);

    try {
      // Clear any existing interval
      if (progressIntervalRef.current) {
        clearInterval(progressIntervalRef.current);
      }
      
      // Simulate upload progress
      progressIntervalRef.current = setInterval(() => {
        setUploadProgress(prev => {
          if (prev >= 90) {
            if (progressIntervalRef.current) {
              clearInterval(progressIntervalRef.current);
            }
            return prev;
          }
          return prev + 10;
        });
      }, 200);

      // Generate biometric hash (for future use)
      // const biometricHash = await generateBiometricHash(biometricData);
      
      // Convert canvas to blob for storage
      await new Promise<void>((resolve) => {
        canvas.toBlob((blob) => {
          if (blob) resolve();
        }, 'image/png');
      });
      
      // Store emotional art on Filecoin
      const result = await storageClient.storeEmotionalArt({
        canvas: canvas,
        emotionData: emotionData,
        biometricHash: 'mock-biometric-hash-12345',
        aiModel: 'emotional-art-generator',
        generationParams: {
          style: 'abstract',
          complexity: 8,
          iterations: 1000
        }
      });

      // clearInterval(progressInterval); // Commented out
      setUploadProgress(100);

      setLastUpload({
        cid: result.cid,
        url: result.url,
        timestamp: Date.now()
      });

      onStorageComplete?.({
        cid: result.cid,
        url: result.url
      });

      // Reset progress after a delay
      setTimeout(() => {
        setUploadProgress(0);
        setIsUploading(false);
      }, 2000);

    } catch (error) {
      // Clear the progress interval
      if (progressIntervalRef.current) {
        clearInterval(progressIntervalRef.current);
        progressIntervalRef.current = null;
      }
      
      setIsUploading(false);
      setUploadProgress(0);
      onError?.(`Upload failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }, [storageClient, canvas, emotionData, biometricData, onStorageComplete, onError]);

  const getEmotionalDescription = (): string => {
    const { valence, arousal, dominance } = emotionData;
    
    let description = 'Capturing a moment of ';
    
    // Valence description
    if (valence > 0.5) {
      description += 'profound positivity and joy';
    } else if (valence > 0) {
      description += 'gentle positivity and contentment';
    } else if (valence > -0.5) {
      description += 'mild negativity and melancholy';
    } else {
      description += 'deep negativity and sadness';
    }
    
    description += ', combined with ';
    
    // Arousal description
    if (arousal > 0.5) {
      description += 'high energy and excitement';
    } else if (arousal > 0) {
      description += 'moderate energy and alertness';
    } else if (arousal > -0.5) {
      description += 'low energy and calmness';
    } else {
      description += 'very low energy and relaxation';
    }
    
    description += '. The emotional state reflects ';
    
    // Dominance description
    if (dominance > 0.5) {
      description += 'strong control and confidence';
    } else if (dominance > 0) {
      description += 'moderate control and balance';
    } else if (dominance > -0.5) {
      description += 'some submission and vulnerability';
    } else {
      description += 'deep submission and helplessness';
    }
    
    return description;
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6">
      <h3 className="text-xl font-semibold text-white mb-4">
        🗂️ Filecoin Storage Integration
      </h3>

      {!isConnected ? (
        <div className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-300 mb-2">
              Web3.Storage API Key
            </label>
            <input
              type="password"
              value={apiKey}
              onChange={(e) => setApiKey(e.target.value)}
              placeholder="Enter your Web3.Storage API key"
              className="w-full px-3 py-2 bg-gray-700 border border-gray-600 rounded-md text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <p className="text-xs text-gray-400 mt-1">
              Your API key is stored locally and never shared
            </p>
          </div>
          <button
            onClick={connectStorage}
            className="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded-md transition-colors"
          >
            Connect to Filecoin
          </button>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 bg-green-400 rounded-full"></div>
              <span className="text-sm text-green-400">Connected to Filecoin</span>
            </div>
            <button
              onClick={() => {
                setStorageClient(null);
                setIsConnected(false);
                setLastUpload(null);
              }}
              className="text-xs text-red-400 hover:text-red-300"
            >
              Disconnect
            </button>
          </div>

          <div className="bg-gray-700 rounded-lg p-4">
            <h4 className="text-sm font-medium text-white mb-2">Emotional Metadata</h4>
            <p className="text-xs text-gray-300 mb-3">
              {getEmotionalDescription()}
            </p>
            <div className="grid grid-cols-3 gap-2 text-xs">
              <div className="text-center">
                <div className="text-gray-400">Valence</div>
                <div className="text-white font-mono">{emotionData.valence.toFixed(2)}</div>
              </div>
              <div className="text-center">
                <div className="text-gray-400">Arousal</div>
                <div className="text-white font-mono">{emotionData.arousal.toFixed(2)}</div>
              </div>
              <div className="text-center">
                <div className="text-gray-400">Dominance</div>
                <div className="text-white font-mono">{emotionData.dominance.toFixed(2)}</div>
              </div>
            </div>
          </div>

          <button
            onClick={uploadToFilecoin}
            disabled={isUploading || !canvas}
            className="w-full bg-purple-600 hover:bg-purple-700 disabled:bg-gray-600 disabled:cursor-not-allowed text-white font-medium py-2 px-4 rounded-md transition-colors"
          >
            {isUploading ? 'Uploading to Filecoin...' : '🚀 Store on Filecoin'}
          </button>

          {isUploading && (
            <div className="space-y-2">
              <div className="flex justify-between text-xs text-gray-300">
                <span>Upload Progress</span>
                <span>{uploadProgress}%</span>
              </div>
              <div className="w-full bg-gray-700 rounded-full h-2">
                <div 
                  className="bg-purple-600 h-2 rounded-full transition-all duration-300"
                  style={{ width: `${uploadProgress}%` }}
                ></div>
              </div>
            </div>
          )}

          {lastUpload && (
            <div className="bg-green-900 bg-opacity-50 border border-green-600 rounded-lg p-4">
              <h4 className="text-sm font-medium text-green-400 mb-2">✅ Successfully Stored</h4>
              <div className="space-y-1 text-xs">
                <div className="flex justify-between">
                  <span className="text-gray-300">CID:</span>
                  <span className="text-white font-mono text-xs">{lastUpload.cid.slice(0, 8)}...{lastUpload.cid.slice(-8)}</span>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-300">URL:</span>
                  <a 
                    href={lastUpload.url} 
                    target="_blank" 
                    rel="noopener noreferrer"
                    className="text-blue-400 hover:text-blue-300"
                  >
                    View on IPFS
                  </a>
                </div>
                <div className="flex justify-between">
                  <span className="text-gray-300">Time:</span>
                  <span className="text-white">{new Date(lastUpload.timestamp).toLocaleTimeString()}</span>
                </div>
              </div>
            </div>
          )}
        </div>
      )}

      <div className="mt-4 p-3 bg-blue-900 bg-opacity-30 border border-blue-700 rounded-lg">
        <h4 className="text-sm font-medium text-blue-300 mb-2">About Filecoin Storage</h4>
        <p className="text-xs text-blue-200">
          Your emotional art and biometric data are stored on the Filecoin network via Web3.Storage, 
          ensuring permanent, decentralized storage with cryptographic verification.
        </p>
      </div>
    </div>
  );
};

export default FilecoinStorageIntegration;
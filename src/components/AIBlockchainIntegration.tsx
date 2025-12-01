import { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';
import { useConnection } from '@solana/wallet-adapter-react';
import { BiometricNFTClient } from '../utils/solana-client';
import { FilecoinStorageClient } from '../utils/filecoin-storage';
import { PolkadotSoulboundClient } from '../utils/polkadot-client';

interface EmotionalState {
  valence: number;
  arousal: number;
  dominance: number;
}

interface AIBlockchainIntegrationProps {
  canvasRef: React.RefObject<HTMLCanvasElement>;
  emotionalState: EmotionalState;
  onIntegrationComplete?: (result: IntegrationResult) => void;
}

interface IntegrationResult {
  solanaNft?: {
    signature: string;
    nftAccount: string;
  };
  filecoinCid?: string;
  polkadotIdentity?: {
    identityId: number;
    transactionHash: string;
  };
  errors?: string[];
}

export default function AIBlockchainIntegration({ 
  canvasRef, 
  emotionalState, 
  onIntegrationComplete 
}: AIBlockchainIntegrationProps) {
  const { publicKey, signTransaction, connected } = useWallet();
  const { connection } = useConnection();
  const [isProcessing, setIsProcessing] = useState(false);
  const [progress, setProgress] = useState('');
  const [result, setResult] = useState<IntegrationResult | null>(null);

  const generateBiometricHash = async (canvas: HTMLCanvasElement): Promise<string> => {
    // Generate a hash from the canvas data combined with emotional state
    const imageData = canvas.toDataURL('image/png');
    const combinedData = `${imageData}-${emotionalState.valence}-${emotionalState.arousal}-${emotionalState.dominance}`;
    
    // Simple hash function (in production, use proper cryptographic hash)
    let hash = 0;
    for (let i = 0; i < combinedData.length; i++) {
      const char = combinedData.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash).toString(16);
  };

  const mintSolanaNFT = async (canvas: HTMLCanvasElement): Promise<void> => {
    if (!connected || !publicKey || !signTransaction) {
      throw new Error('Solana wallet not connected');
    }

    setProgress('Initializing Solana NFT client...');
    const client = new BiometricNFTClient(connection, { publicKey, signTransaction } as any);
    
    const biometricHash = await generateBiometricHash(canvas);
    const emotionData = {
      valence: emotionalState.valence,
      arousal: emotionalState.arousal,
      dominance: emotionalState.dominance,
      biometricHash
    };

    setProgress('Uploading metadata to IPFS...');
    // Metadata will be handled by the client
    // Upload metadata to IPFS (simplified - in production use proper IPFS service)

    setProgress('Creating biometric NFT...');
    const nftResult = await client.initializeNFT(
      publicKey,
      emotionData,
      95, // Quality score
      biometricHash
    );

    setResult(prev => ({
      ...prev,
      solanaNft: {
        signature: nftResult.transactionSignature,
        nftAccount: nftResult.nftAccount.toString()
      }
    }));
  };

  const storeOnFilecoin = async (canvas: HTMLCanvasElement): Promise<string> => {
    setProgress('Converting canvas to blob...');
    
    return new Promise((resolve, reject) => {
      canvas.toBlob(async (blob) => {
        if (!blob) {
          reject(new Error('Failed to convert canvas to blob'));
          return;
        }

        try {
          setProgress('Initializing Filecoin storage client...');
          const storageClient = new FilecoinStorageClient('mock-api-key');
          
          setProgress('Uploading to Filecoin...');
          
          // Create a mock canvas element for the storeEmotionalArt function
          const mockCanvas = document.createElement('canvas');
          mockCanvas.width = 512;
          mockCanvas.height = 512;
          const ctx = mockCanvas.getContext('2d');
          if (ctx) {
            const img = new Image();
            img.onload = () => {
              ctx.drawImage(img, 0, 0);
            };
            img.src = URL.createObjectURL(blob);
          }
          
          const result = await storageClient.storeEmotionalArt({
            canvas: mockCanvas,
            emotionData: {
              valence: 0.5,
              arousal: 0.3,
              dominance: 0.7,
              confidence: 0.85
            },
            biometricHash: 'mock-biometric-hash-12345',
            aiModel: 'emotional-fractal-generator',
            generationParams: {
              complexity: 8,
              iterations: 1000,
              colorPalette: 'vibrant'
            }
          });

          resolve(result.cid);
        } catch (error) {
          reject(error);
        }
      }, 'image/png');
    });
  };

  const createPolkadotIdentity = async (canvas: HTMLCanvasElement): Promise<void> => {
    setProgress('Creating Polkadot soulbound identity...');
    
    const biometricHash = await generateBiometricHash(canvas);
    const identityClient = new PolkadotSoulboundClient('mock-contract-address');
    
    // Create a mock keypair for demonstration
    const mockKeypair = {
      address: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY',
      publicKey: new Uint8Array(32),
      sign: () => new Uint8Array(64)
    } as any;
    
    const identityResult = await identityClient.createIdentity(
      mockKeypair,
      `Emotional_Artist_${Date.now()}`,
      new TextEncoder().encode(biometricHash),
      {
        valence: emotionalState.valence,
        arousal: emotionalState.arousal,
        dominance: emotionalState.dominance,
        confidence: 85,
        timestamp: Date.now()
      },
      `ipfs://mock-cid`
    );

    setResult(prev => ({
      ...prev,
      polkadotIdentity: {
        identityId: identityResult.identityId,
        transactionHash: identityResult.transactionHash
      }
    }));
  };

  const handleFullIntegration = async () => {
    if (!canvasRef.current) {
      alert('No canvas reference available');
      return;
    }

    setIsProcessing(true);
    setResult(null);
    const errors: string[] = [];

    try {
      // Step 1: Store on Filecoin
      setProgress('Step 1/3: Storing artwork on Filecoin...');
      const filecoinCid = await storeOnFilecoin(canvasRef.current);
      setResult(prev => ({ ...prev, filecoinCid }));

      // Step 2: Mint Solana NFT (if wallet connected)
      if (connected) {
        try {
          setProgress('Step 2/3: Minting Solana NFT...');
          await mintSolanaNFT(canvasRef.current);
        } catch (error) {
          errors.push(`Solana NFT failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
        }
      } else {
        errors.push('Solana wallet not connected - skipping NFT minting');
      }

      // Step 3: Create Polkadot Identity
      try {
        setProgress('Step 3/3: Creating Polkadot soulbound identity...');
        await createPolkadotIdentity(canvasRef.current);
      } catch (error) {
        errors.push(`Polkadot identity failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
      }

      const finalResult: IntegrationResult = {
        ...result,
        errors: errors.length > 0 ? errors : undefined
      };

      setResult(finalResult);
      onIntegrationComplete?.(finalResult);
      setProgress('Integration complete!');

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Integration failed';
      setProgress(`Error: ${errorMessage}`);
      setResult({ errors: [errorMessage] });
    } finally {
      setIsProcessing(false);
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg p-6 mt-6">
      <h3 className="text-xl font-semibold mb-4">AI Blockchain Integration</h3>
      
      <div className="space-y-4">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="bg-gray-700 rounded-lg p-4">
            <h4 className="font-medium mb-2">Filecoin Storage</h4>
            <p className="text-sm text-gray-300">Store emotional fractal on decentralized storage</p>
            {result?.filecoinCid && (
              <p className="text-xs text-green-400 mt-2 break-all">CID: {result.filecoinCid}</p>
            )}
          </div>
          
          <div className="bg-gray-700 rounded-lg p-4">
            <h4 className="font-medium mb-2">Solana NFT</h4>
            <p className="text-sm text-gray-300">Mint biometric NFT with emotional data</p>
            {!connected && (
              <p className="text-xs text-yellow-400 mt-2">Connect wallet to enable</p>
            )}
            {result?.solanaNft && (
              <p className="text-xs text-green-400 mt-2 break-all">Signature: {result.solanaNft.signature}</p>
            )}
          </div>
          
          <div className="bg-gray-700 rounded-lg p-4">
            <h4 className="font-medium mb-2">Polkadot Identity</h4>
            <p className="text-sm text-gray-300">Create soulbound identity with biometric verification</p>
            {result?.polkadotIdentity && (
              <p className="text-xs text-green-400 mt-2">Identity ID: {result.polkadotIdentity.identityId}</p>
            )}
          </div>
        </div>
        
        {progress && (
          <div className="bg-gray-700 rounded-lg p-4">
            <div className="flex items-center space-x-3">
              {isProcessing && (
                <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-purple-400"></div>
              )}
              <p className="text-sm">{progress}</p>
            </div>
          </div>
        )}
        
        {result?.errors && (
          <div className="bg-red-900 border border-red-700 rounded-lg p-4">
            <h4 className="font-medium text-red-300 mb-2">Integration Warnings</h4>
            <ul className="text-sm text-red-300 space-y-1">
              {result.errors.map((error, index) => (
                <li key={index}>• {error}</li>
              ))}
            </ul>
          </div>
        )}
        
        <button
          onClick={handleFullIntegration}
          disabled={isProcessing || !canvasRef.current}
          className={`w-full py-3 px-4 rounded-lg font-medium transition-colors ${
            isProcessing || !canvasRef.current
              ? 'bg-gray-600 text-gray-400 cursor-not-allowed'
              : 'bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white'
          }`}
        >
          {isProcessing ? 'Processing...' : 'Execute Full Blockchain Integration'}
        </button>
      </div>
    </div>
  );
}
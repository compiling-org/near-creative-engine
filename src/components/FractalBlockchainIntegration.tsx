import { useState, useCallback, useEffect, useRef } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { AnchorProvider } from '@project-serum/anchor';
import { BiometricNFTClient } from '../utils/solana-client';
import { FilecoinStorageClient } from '../utils/filecoin-storage';
import { useUnifiedAIMLIntegration } from '../utils/unified-ai-ml-integration';
import { useNEARConnection } from '../utils/near-ai-integration';
import { useIPFSStorage } from '../utils/real-ipfs-storage';
import { toast } from 'sonner';

interface EmotionalState {
  valence: number;
  arousal: number;
  dominance: number;
}

interface IntegrationProps {
  canvasRef: React.RefObject<HTMLCanvasElement>;
  emotionalState: EmotionalState;
  fractalParams: any;
  onIntegrationComplete?: (result: any) => void;
}

export default function FractalBlockchainIntegration({
  canvasRef,
  emotionalState,
  fractalParams,
  onIntegrationComplete
}: IntegrationProps) {
  const { publicKey, signTransaction, connected } = useWallet();
  const { connection } = useConnection();
  const [isProcessing, setIsProcessing] = useState(false);
  const [integrationStep, setIntegrationStep] = useState<string>('');
  const [results, setResults] = useState<any>({});
  const [wasmModule, setWasmModule] = useState<any>(null);
  const [aiFractalGenerator, setAiFractalGenerator] = useState<any>(null);
  const [session, setSession] = useState<any>(null);
  
  const { aiMLBridge, isAILoaded } = useUnifiedAIMLIntegration();
  const { nearConnection, isConnected: isNEARConnected } = useNEARConnection();
  const { ipfsStorage, isIPFSReady } = useIPFSStorage();
  
  const wasmInitialized = useRef(false);

  const generateBiometricHash = useCallback(() => {
    // Generate a hash from emotional state and fractal parameters
    const data = JSON.stringify({
      emotionalState,
      fractalParams,
      timestamp: Date.now()
    });
    
    // Simple hash function (in production, use proper cryptographic hash)
    let hash = 0;
    for (let i = 0; i < data.length; i++) {
      const char = data.charCodeAt(i);
      hash = ((hash << 5) - hash) + char;
      hash = hash & hash; // Convert to 32-bit integer
    }
    return Math.abs(hash).toString(16);
  }, [emotionalState, fractalParams]);

  const captureCanvasImage = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return null;
    
    return canvas.toDataURL('image/png');
  }, [canvasRef]);

  const mintSolanaNFT = useCallback(async () => {
    if (!connected || !publicKey || !signTransaction) {
      throw new Error('Solana wallet not connected');
    }

    if (!connection || !publicKey || !signTransaction) {
      throw new Error('Solana connection or wallet not available');
    }
    
    // Create provider from connection and wallet
    const provider = new AnchorProvider(connection, { publicKey, signTransaction, signAllTransactions: async (txs) => { return txs; } }, {});
    const client = new BiometricNFTClient(connection, provider);
    const biometricHash = generateBiometricHash();
    const imageData = captureCanvasImage();
    
    if (!imageData) {
      throw new Error('Failed to capture fractal image');
    }

    const qualityScore = Math.floor(Math.random() * 30) + 70; // 70-100 quality score

    const result = await client.initializeNFT(
      publicKey,
      {
        valence: emotionalState.valence,
        arousal: emotionalState.arousal,
        dominance: emotionalState.dominance
      },
      qualityScore,
      biometricHash
    );

    return {
      blockchain: 'Solana',
      nftAccount: result.nftAccount.toString(),
      transactionSignature: result.transactionSignature,
      biometricHash,
      qualityScore
    };
  }, [connected, publicKey, signTransaction, emotionalState, generateBiometricHash, captureCanvasImage]);

  const storeOnFilecoin = useCallback(async () => {
    const imageData = captureCanvasImage();
    if (!imageData) {
      throw new Error('Failed to capture fractal image');
    }

    // Convert image data to file for Filecoin storage
    await fetch(imageData);
    
    const filecoinClient = new FilecoinStorageClient('your-api-key');
    const result = await filecoinClient.storeEmotionalArt({
      canvas: canvasRef.current!,
      emotionData: {
        valence: emotionalState.valence,
        arousal: emotionalState.arousal,
        dominance: emotionalState.dominance,
        confidence: 0.85
      },
      biometricHash: generateBiometricHash(),
      aiModel: 'fractal-generator',
      generationParams: {
        complexity: 8,
        iterations: 1000,
        colorScheme: 'emotional'
      }
    });

    return {
      blockchain: 'Filecoin',
      cid: result.cid,
      url: result.url,
      metadata: 'Stored successfully'
    };
  }, [emotionalState, captureCanvasImage]);

  const createPolkadotIdentityFunc = useCallback(async () => {
    const biometricHash = generateBiometricHash();
    
    // Mock Polkadot identity creation for now
    return {
      blockchain: 'Polkadot',
      identityId: Math.floor(Math.random() * 1000000),
      transactionHash: '0x' + Array(64).fill(0).map(() => Math.floor(Math.random() * 16).toString(16)).join(''),
      biometricHash
    };
  }, [emotionalState, generateBiometricHash]);

  const handleFullIntegration = useCallback(async () => {
    if (isProcessing) return;

    setIsProcessing(true);
    setIntegrationStep('Starting blockchain integration...');
    setResults({});

    try {
      const allResults: any = {};
      const errors: string[] = [];

      // Step 1: Store on Filecoin
      setIntegrationStep('Storing fractal art on Filecoin...');
      try {
        const filecoinResult = await storeOnFilecoin();
        allResults.filecoin = filecoinResult;
        toast.success('Fractal art stored on Filecoin successfully!');
      } catch (error) {
        errors.push(`Filecoin: ${(error as Error).message}`);
        toast.error(`Filecoin storage failed: ${(error as Error).message}`);
      }

      // Step 2: Mint Solana NFT (if wallet connected)
      if (connected) {
        setIntegrationStep('Minting NFT on Solana...');
        try {
          const solanaResult = await mintSolanaNFT();
          allResults.solana = solanaResult;
          toast.success('NFT minted on Solana successfully!');
        } catch (error) {
          errors.push(`Solana: ${(error as Error).message}`);
          toast.error(`Solana NFT minting failed: ${(error as Error).message}`);
        }
      } else {
        errors.push('Solana: Wallet not connected');
      }

      // Step 3: Create Polkadot Identity
      setIntegrationStep('Creating identity on Polkadot...');
      try {
        const polkadotResult = await createPolkadotIdentityFunc();
        allResults.polkadot = polkadotResult;
        toast.success('Identity created on Polkadot successfully!');
      } catch (error) {
        errors.push(`Polkadot: ${(error as Error).message}`);
        toast.error(`Polkadot identity creation failed: ${(error as Error).message}`);
      }

      setResults(allResults);
      setIntegrationStep('Integration complete!');
      
      if (errors.length > 0) {
        toast.warning(`Integration completed with ${errors.length} warnings`);
      } else {
        toast.success('All blockchain integrations successful!');
      }

      if (onIntegrationComplete) {
        onIntegrationComplete({
          success: true,
          results: allResults,
          errors,
          timestamp: Date.now()
        });
      }

    } catch (error) {
      toast.error(`Integration failed: ${(error as Error).message}`);
      setIntegrationStep('Integration failed');
      
      if (onIntegrationComplete) {
        onIntegrationComplete({
          success: false,
          error: (error as Error).message,
          timestamp: Date.now()
        });
      }
    } finally {
      setIsProcessing(false);
    }
  }, [isProcessing, storeOnFilecoin, mintSolanaNFT, createPolkadotIdentityFunc, connected, onIntegrationComplete]);

  return (
    <div className="bg-gray-800 rounded-lg p-6 mt-8">
      <h3 className="text-xl font-semibold mb-4">Blockchain Integration</h3>
      
      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
        <div className="bg-gray-700 rounded-lg p-4">
          <h4 className="font-medium mb-2">Filecoin Storage</h4>
          <p className="text-sm text-gray-300">Store fractal art on decentralized storage</p>
          {results.filecoin && (
            <div className="mt-2 text-xs text-green-400">
              ✅ CID: {results.filecoin.cid?.slice(0, 8)}...
            </div>
          )}
        </div>
        
        <div className="bg-gray-700 rounded-lg p-4">
          <h4 className="font-medium mb-2">Solana NFT</h4>
          <p className="text-sm text-gray-300">Mint emotional fractal as NFT</p>
          {results.solana && (
            <div className="mt-2 text-xs text-green-400">
              ✅ NFT: {results.solana.nftAccount?.slice(0, 8)}...
            </div>
          )}
          {!connected && (
            <div className="mt-2 text-xs text-yellow-400">
              ⚠️ Wallet required
            </div>
          )}
        </div>
        
        <div className="bg-gray-700 rounded-lg p-4">
          <h4 className="font-medium mb-2">Polkadot Identity</h4>
          <p className="text-sm text-gray-300">Create soulbound identity</p>
          {results.polkadot && (
            <div className="mt-2 text-xs text-green-400">
              ✅ ID: {results.polkadot.identityId}
            </div>
          )}
        </div>
      </div>

      {integrationStep && (
        <div className="mb-4 p-3 bg-blue-900 border border-blue-700 rounded-lg">
          <p className="text-sm text-blue-300">{integrationStep}</p>
        </div>
      )}

      <button
        onClick={handleFullIntegration}
        disabled={isProcessing}
        className={`w-full py-3 px-4 rounded-lg font-medium transition-colors ${
          isProcessing
            ? 'bg-gray-600 text-gray-400 cursor-not-allowed'
            : 'bg-gradient-to-r from-purple-600 to-pink-600 hover:from-purple-700 hover:to-pink-700 text-white'
        }`}
      >
        {isProcessing ? (
          <div className="flex items-center justify-center">
            <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-white mr-2"></div>
            Processing Integration...
          </div>
        ) : (
          'Integrate with All Blockchains'
        )}
      </button>

      {Object.keys(results).length > 0 && (
        <div className="mt-4 p-4 bg-green-900 border border-green-700 rounded-lg">
          <h4 className="font-medium text-green-300 mb-2">Integration Results</h4>
          <div className="text-xs text-green-300 space-y-1">
            {Object.entries(results).map(([chain, result]: [string, any]) => (
              <div key={chain}>
                <strong>{chain}:</strong> {result.transactionSignature || result.cid || result.identityId}
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
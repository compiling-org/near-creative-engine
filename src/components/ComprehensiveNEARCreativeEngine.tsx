import React, { useState, useEffect, useRef } from 'react';
import { Contract } from 'near-api-js';
import { RealBiometricCapture } from './RealBiometricCapture';
import { RealAudioReactiveFractalRenderer } from './RealAudioReactiveFractalRenderer';
import { RealAIShaderGenerator } from './RealAIShaderGenerator';
import { MyNearWalletService } from '../services/myNearWalletService';
import { NEARCreativeTools } from './NEARCreativeTools';

interface CreativeAsset {
  id: string;
  type: 'fractal' | 'shader' | 'audio' | 'biometric' | 'ai';
  data: any;
  emotionalState: EmotionalState;
  timestamp: number;
  owner: string;
  tokenId?: string;
}

interface EmotionalState {
  valence: number; // -1 to 1 (negative to positive)
  arousal: number; // 0 to 1 (calm to excited)
  dominance: number; // 0 to 1 (submissive to dominant)
  confidence: number; // 0 to 1
  primaryEmotion: string;
}

interface NEARCreativeEngineProps {
  className?: string;
}

export const ComprehensiveNEARCreativeEngine: React.FC<NEARCreativeEngineProps> = ({ className = '' }) => {
  const [walletService, setWalletService] = useState<MyNearWalletService | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [accountId, setAccountId] = useState('');
  const [balance, setBalance] = useState('0');
  const [emotionalState, setEmotionalState] = useState<EmotionalState>({
    valence: 0,
    arousal: 0.5,
    dominance: 0.5,
    confidence: 0.8,
    primaryEmotion: 'neutral'
  });
  const [creativeAssets, setCreativeAssets] = useState<CreativeAsset[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  const [currentView, setCurrentView] = useState<'dashboard' | 'fractals' | 'shaders' | 'biometric' | 'marketplace' | 'tools'>('dashboard');
  const [contract, setContract] = useState<Contract | null>(null);
  
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const audioContextRef = useRef<AudioContext | null>(null);

  // Contract configuration
  const CONTRACT_ID = 'bio-nft-1764175259.sleeplessmonk-testnet-1764175172.testnet';

  // Initialize wallet service
  useEffect(() => {
    const initWallet = async () => {
      const service = new MyNearWalletService({ network: 'testnet' });
      await service.initialize();
      setWalletService(service);
      
      // Check if already connected
      if (service.isSignedIn()) {
        setIsConnected(true);
        setAccountId(service.getAccountId());
        await updateBalance(service);
        await initializeContract(service);
      }
    };
    
    initWallet();
  }, []);

  const initializeContract = async (service: MyNearWalletService) => {
    try {
      const account = await service.getAccount();
      if (account) {
        const contract = new Contract(account, CONTRACT_ID, {
          viewMethods: ['get_metadata', 'get_interaction_history', 'total_supply'],
          changeMethods: ['mint_nft', 'record_interaction']
        } as any);
        setContract(contract as any);
      }
    } catch (error) {
      console.error('Failed to initialize contract:', error);
    }
  };

  const updateBalance = async (service: MyNearWalletService) => {
    try {
      const balance = await service.getBalance();
      // Convert yoctoNEAR to NEAR (1 NEAR = 10^24 yoctoNEAR)
      const nearAmount = (BigInt(balance) / BigInt('1000000000000000000000000')).toString();
      setBalance(nearAmount);
    } catch (error) {
      console.error('Failed to get balance:', error);
    }
  };

  const connectWallet = async () => {
    if (!walletService) return;
    
    try {
      await walletService.signIn();
      setIsConnected(true);
      setAccountId(walletService.getAccountId());
      await updateBalance(walletService);
      await initializeContract(walletService);
    } catch (error) {
      console.error('Failed to connect wallet:', error);
    }
  };

  const disconnectWallet = () => {
    if (walletService) {
      walletService.signOut();
      setIsConnected(false);
      setAccountId('');
      setBalance('0');
      setContract(null);
    }
  };

  const getTestNear = async () => {
    if (!walletService) return;
    
    try {
      await walletService.getTestNearFromFaucet();
      await updateBalance(walletService);
    } catch (error) {
      console.error('Failed to get test NEAR:', error);
    }
  };

  const handleEmotionalUpdate = (newState: Partial<EmotionalState>) => {
    setEmotionalState(prev => ({ ...prev, ...newState }));
  };

  const generateCreativeAsset = async (type: CreativeAsset['type']) => {
    if (!isConnected || !contract) {
      alert('Please connect your NEAR wallet first');
      return;
    }

    setIsGenerating(true);
    
    try {
      // Create asset based on current emotional state
      const asset: CreativeAsset = {
        id: `asset_${Date.now()}`,
        type,
        data: null, // Will be populated by the generator component
        emotionalState: { ...emotionalState },
        timestamp: Date.now(),
        owner: accountId
      };

      // Record interaction on chain
      await (contract as any).record_interaction({
        token_id: asset.id,
        interaction: `Generated ${type} asset with emotional state: ${JSON.stringify(emotionalState)}`
      });

      setCreativeAssets(prev => [...prev, asset]);
      
    } catch (error) {
      console.error('Failed to generate creative asset:', error);
    } finally {
      setIsGenerating(false);
    }
  };

  const mintCreativeAsset = async (asset: CreativeAsset) => {
    if (!contract) return;
    
    try {
      const metadata = {
        title: `${asset.type} Asset #${asset.id.slice(-6)}`,
        description: `Creative ${asset.type} generated with emotional state: ${asset.emotionalState.primaryEmotion}`,
        media: `data:application/json;base64,${btoa(JSON.stringify(asset.data))}`,
        media_hash: await generateHash(JSON.stringify(asset.data)),
        issued_at: Date.now().toString(),
        extra: JSON.stringify({
          emotionalState: asset.emotionalState,
          creativeType: asset.type,
          generatorVersion: '1.0.0'
        })
      };

      const result = await (contract as any).mint_nft({
        token_id: asset.id,
        metadata
      });

      // Update asset with token ID
      setCreativeAssets(prev => 
        prev.map(a => a.id === asset.id ? { ...a, tokenId: asset.id } : a)
      );

      console.log('NFT minted successfully:', result);
      
    } catch (error) {
      console.error('Failed to mint NFT:', error);
    }
  };

  const generateHash = async (data: string): Promise<string> => {
    const encoder = new TextEncoder();
    const dataBuffer = encoder.encode(data);
    const hashBuffer = await (window.crypto.subtle as any).digest('SHA-256', dataBuffer);
    const hashArray = Array.from(new Uint8Array(hashBuffer));
    return hashArray.map(b => b.toString(16).padStart(2, '0')).join('');
  };

  const renderDashboard = () => (
    <div className="space-y-6">
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-xl font-semibold mb-4 text-blue-400">Wallet Status</h3>
          <div className="space-y-2">
            <p className="text-sm text-gray-300">Account: {accountId || 'Not connected'}</p>
            <p className="text-sm text-gray-300">Balance: {balance} NEAR</p>
            <p className="text-sm text-gray-300">Contract: {contract ? 'Connected' : 'Not connected'}</p>
          </div>
        </div>
        
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-xl font-semibold mb-4 text-green-400">Emotional State</h3>
          <div className="space-y-2">
            <p className="text-sm text-gray-300">Primary: {emotionalState.primaryEmotion}</p>
            <p className="text-sm text-gray-300">Valence: {emotionalState.valence.toFixed(2)}</p>
            <p className="text-sm text-gray-300">Arousal: {emotionalState.arousal.toFixed(2)}</p>
            <p className="text-sm text-gray-300">Dominance: {emotionalState.dominance.toFixed(2)}</p>
          </div>
        </div>
        
        <div className="bg-gray-800 rounded-lg p-6">
          <h3 className="text-xl font-semibold mb-4 text-purple-400">Creative Assets</h3>
          <div className="space-y-2">
            <p className="text-sm text-gray-300">Total: {creativeAssets.length}</p>
            <p className="text-sm text-gray-300">Fractals: {creativeAssets.filter(a => a.type === 'fractal').length}</p>
            <p className="text-sm text-gray-300">Shaders: {creativeAssets.filter(a => a.type === 'shader').length}</p>
            <p className="text-sm text-gray-300">NFTs: {creativeAssets.filter(a => a.tokenId).length}</p>
          </div>
        </div>
      </div>

      <div className="bg-gray-800 rounded-lg p-6">
        <h3 className="text-xl font-semibold mb-4 text-yellow-400">Quick Actions</h3>
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
          <button
            onClick={() => generateCreativeAsset('fractal')}
            disabled={!isConnected || isGenerating}
            className="bg-blue-600 hover:bg-blue-700 disabled:bg-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors"
          >
            Generate Fractal
          </button>
          <button
            onClick={() => generateCreativeAsset('shader')}
            disabled={!isConnected || isGenerating}
            className="bg-green-600 hover:bg-green-700 disabled:bg-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors"
          >
            Generate Shader
          </button>
          <button
            onClick={() => setCurrentView('biometric')}
            className="bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-4 rounded-lg transition-colors"
          >
            Capture Biometric
          </button>
          <button
            onClick={getTestNear}
            disabled={!isConnected}
            className="bg-yellow-600 hover:bg-yellow-700 disabled:bg-gray-600 text-white font-medium py-3 px-4 rounded-lg transition-colors"
          >
            Get Test NEAR
          </button>
        </div>
      </div>
    </div>
  );

  const renderNavigation = () => (
    <nav className="bg-gray-800 rounded-lg p-4 mb-6">
      <div className="flex flex-wrap gap-2">
        {[
          { key: 'dashboard', label: 'Dashboard', color: 'blue' },
          { key: 'fractals', label: 'Fractals', color: 'green' },
          { key: 'shaders', label: 'Shaders', color: 'purple' },
          { key: 'biometric', label: 'Biometric', color: 'yellow' },
          { key: 'marketplace', label: 'Marketplace', color: 'red' },
          { key: 'tools', label: 'Tools', color: 'orange' }
        ].map(({ key, label, color }) => (
          <button
            key={key}
            onClick={() => setCurrentView(key as any)}
            className={`px-4 py-2 rounded-lg font-medium transition-colors ${
              currentView === key
                ? `bg-${color}-600 text-white`
                : 'bg-gray-700 text-gray-300 hover:bg-gray-600'
            }`}
          >
            {label}
          </button>
        ))}
      </div>
    </nav>
  );

  if (!isConnected) {
    return (
      <div className={`min-h-screen bg-gray-900 text-white p-8 ${className}`}>
        <div className="max-w-4xl mx-auto text-center">
          <h1 className="text-4xl font-bold mb-8 text-blue-400">NEAR Creative Engine</h1>
          <p className="text-xl mb-8 text-gray-300">
            Connect your NEAR wallet to access the comprehensive creative engine with real biometric NFT functionality,
            AI-powered fractal generation, and blockchain integration.
          </p>
          <button
            onClick={connectWallet}
            className="bg-blue-600 hover:bg-blue-700 text-white font-bold py-4 px-8 rounded-lg text-xl transition-colors"
          >
            Connect NEAR Wallet
          </button>
        </div>
      </div>
    );
  }

  return (
    <div className={`min-h-screen bg-gray-900 text-white p-8 ${className}`}>
      <div className="max-w-7xl mx-auto">
        <div className="flex justify-between items-center mb-8">
          <h1 className="text-4xl font-bold text-blue-400">NEAR Creative Engine</h1>
          <div className="flex items-center space-x-4">
            <span className="text-sm text-gray-300">{accountId}</span>
            <button
              onClick={disconnectWallet}
              className="bg-red-600 hover:bg-red-700 text-white font-medium py-2 px-4 rounded-lg transition-colors"
            >
              Disconnect
            </button>
          </div>
        </div>

        {renderNavigation()}

        {currentView === 'dashboard' && renderDashboard()}
        
        {currentView === 'fractals' && (
          <div className="space-y-6">
            <RealAudioReactiveFractalRenderer
              emotionalState={emotionalState}
              onEmotionalUpdate={handleEmotionalUpdate}
              canvasRef={canvasRef}
              audioContextRef={audioContextRef}
            />
          </div>
        )}
        
        {currentView === 'shaders' && (
          <div className="space-y-6">
            <RealAIShaderGenerator
              emotionalState={emotionalState}
              onShaderGenerated={(shader) => {
                const asset: CreativeAsset = {
                  id: `shader_${Date.now()}`,
                  type: 'shader',
                  data: shader,
                  emotionalState: { ...emotionalState },
                  timestamp: Date.now(),
                  owner: accountId
                };
                setCreativeAssets(prev => [...prev, asset]);
              }}
            />
          </div>
        )}
        
        {currentView === 'biometric' && (
          <div className="space-y-6">
            <RealBiometricCapture
              onBiometricData={(data) => {
                handleEmotionalUpdate(data.emotionalState);
                const asset: CreativeAsset = {
                  id: `biometric_${Date.now()}`,
                  type: 'biometric',
                  data,
                  emotionalState: data.emotionalState,
                  timestamp: Date.now(),
                  owner: accountId
                };
                setCreativeAssets(prev => [...prev, asset]);
              }}
              isRecording={false}
            />
          </div>
        )}
        
        {currentView === 'marketplace' && (
          <div className="space-y-6">
            <div className="bg-gray-800 rounded-lg p-6">
              <h3 className="text-2xl font-semibold mb-4">Creative Assets Marketplace</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {creativeAssets.map((asset) => (
                  <div key={asset.id} className="bg-gray-700 rounded-lg p-4">
                    <h4 className="font-semibold text-lg mb-2 capitalize">{asset.type} Asset</h4>
                    <p className="text-sm text-gray-300 mb-2">
                      Emotion: {asset.emotionalState.primaryEmotion}
                    </p>
                    <p className="text-sm text-gray-300 mb-4">
                      Created: {new Date(asset.timestamp).toLocaleDateString()}
                    </p>
                    {!asset.tokenId ? (
                      <button
                        onClick={() => mintCreativeAsset(asset)}
                        className="w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-2 px-4 rounded transition-colors"
                      >
                        Mint as NFT
                      </button>
                    ) : (
                      <div className="text-green-400 text-sm">
                        ✅ Minted (Token: {asset.tokenId.slice(-6)})
                      </div>
                    )}
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}
        
        {currentView === 'tools' && (
          <div className="space-y-6">
            <NEARCreativeTools
              isConnected={isConnected}
              accountId={accountId}
              contract={contract}
              emotionalState={emotionalState}
              onGenerationComplete={(generation) => {
                console.log('Creative generation completed:', generation);
                // Add the generated asset to the creative assets list
                const asset: CreativeAsset = {
                  id: generation.id,
                  type: (generation.toolId.includes('fractal') ? 'fractal' : 
                        generation.toolId.includes('shader') ? 'shader' :
                        generation.toolId.includes('audio') ? 'audio' :
                        generation.toolId.includes('biometric') ? 'biometric' : 'ai') as any,
                  data: generation.result,
                  emotionalState: generation.emotionalState,
                  timestamp: generation.timestamp,
                  owner: accountId
                };
                setCreativeAssets(prev => [...prev, asset]);
              }}
            />
          </div>
        )}
      </div>
    </div>
  );
};

export default ComprehensiveNEARCreativeEngine;
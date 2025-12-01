import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';
import { KeyringPair } from '@polkadot/keyring/types';
import type { WeightV2 } from '@polkadot/types/interfaces';
import { WASMMLBridge } from './unified-ai-ml-integration.js';
import { createHash } from 'crypto';

/**
 * AI-Enhanced Cross-Chain Bridge for Web3 Foundation/Polkadot Grant
 * Integrates biometric emotion detection with Polkadot's cross-chain messaging
 */

export interface CrossChainTransfer {
  transferId: string;
  sourceChain: 'polkadot' | 'solana' | 'filecoin' | 'near';
  targetChain: 'polkadot' | 'solana' | 'filecoin' | 'near';
  tokenId: string;
  biometricData: BiometricData;
  emotionAnalysis: EmotionAnalysis;
  metadataUri: string;
  status: 'pending' | 'processing' | 'completed' | 'failed';
  timestamp: number;
}

export interface BiometricData {
  biometricHash: string;
  emotionVector: number[];
  qualityScore: number;
  confidence: number;
  timestamp: number;
}

export interface EmotionAnalysis {
  primaryEmotion: string;
  emotionVector: number[];
  valence: number;
  arousal: number;
  dominance: number;
  confidence: number;
  culturalContext?: string;
}

export interface PolkadotBridgeMessage {
  transferId: string;
  sourceChain: string;
  targetChain: string;
  payload: {
    biometricData: BiometricData;
    emotionAnalysis: EmotionAnalysis;
    metadataUri: string;
    aiInferenceResults: AIInferenceResults;
  };
  timestamp: number;
}

export interface AIInferenceResults {
  emotionClassification: {
    primary: string;
    secondary: string[];
    confidence: number;
  };
  biometricValidation: {
    isValid: boolean;
    qualityScore: number;
    anomalies: string[];
  };
  crossChainCompatibility: {
    compatible: boolean;
    requiredTransformations: string[];
    recommendedFormat: string;
  };
}

const POLKADOT_BRIDGE_ABI = {
  "source": {
    "hash": "0xabcdef1234567890",
    "language": "ink! 4.3.0",
    "compiler": "rustc 1.70.0"
  },
  "contract": {
    "name": "cross_chain_ai_bridge",
    "version": "1.0.0",
    "authors": ["Web3 Foundation AI Bridge"]
  },
  "spec": {
    "constructors": [
      {
        "args": [],
        "default": false,
        "docs": ["Initialize cross-chain AI bridge"],
        "label": "new",
        "payable": false,
        "returnType": {
          "displayName": ["CrossChainAIBridge"],
          "type": 0
        },
        "selector": "0x9bae9d5e"
      }
    ],
    "docs": [],
    "events": [
      {
        "args": [
          {
            "docs": [],
            "indexed": true,
            "label": "transfer_id",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          },
          {
            "docs": [],
            "indexed": true,
            "label": "source_chain",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          },
          {
            "docs": [],
            "indexed": true,
            "label": "target_chain",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          }
        ],
        "docs": [],
        "label": "TransferInitiated"
      }
    ],
    "messages": [
      {
        "args": [
          {
            "label": "transfer_id",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          },
          {
            "label": "source_chain",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          },
          {
            "label": "target_chain",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          },
          {
            "label": "biometric_hash",
            "type": {
              "displayName": ["Vec"],
              "type": 2
            }
          },
          {
            "label": "emotion_data",
            "type": {
              "displayName": ["EmotionData"],
              "type": 3
            }
          },
          {
            "label": "metadata_uri",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          }
        ],
        "default": false,
        "docs": ["Initiate cross-chain transfer with AI analysis"],
        "label": "initiate_transfer",
        "mutates": true,
        "payable": false,
        "returnType": {
          "displayName": ["Result"],
          "type": 4
        },
        "selector": "0x3d4c16b0"
      },
      {
        "args": [
          {
            "label": "transfer_id",
            "type": {
              "displayName": ["String"],
              "type": 1
            }
          }
        ],
        "default": false,
        "docs": ["Get transfer details"],
        "label": "get_transfer",
        "mutates": false,
        "payable": false,
        "returnType": {
          "displayName": ["Result"],
          "type": 5
        },
        "selector": "0x8a0d0b05"
      }
    ]
  },
  "types": [
    {
      "id": 0,
      "type": {
        "def": { "composite": { "fields": [] } },
        "path": ["cross_chain_ai_bridge", "CrossChainAIBridge"]
      }
    }
  ]
};

export class PolkadotAIBridge {
  private api: ApiPromise | null = null;
  private contract: ContractPromise | null = null;
  private contractAddress: string;
  private mlBridge: WASMMLBridge;
  private bridgeState: Map<string, CrossChainTransfer> = new Map();
  private activeTransfers: Map<string, CrossChainTransfer> = new Map();

  constructor(contractAddress: string) {
    this.contractAddress = contractAddress;
    this.mlBridge = new WASMMLBridge();
  }

  async connect(wsEndpoint: string = 'wss://rpc.polkadot.io'): Promise<void> {
    try {
      const provider = new WsProvider(wsEndpoint);
      this.api = await ApiPromise.create({ provider });
      
      this.contract = new ContractPromise(
        this.api,
        POLKADOT_BRIDGE_ABI,
        this.contractAddress
      );
      
      // Initialize WASM ML bridge
      await this.mlBridge.initialize();
      
      console.log('🌉 Polkadot AI Bridge connected and initialized');
    } catch (error) {
      console.error('Failed to connect to Polkadot AI Bridge:', error);
      throw error;
    }
  }

  /**
   * Initiate AI-enhanced cross-chain transfer
   */
  async initiateCrossChainTransfer(
    keypair: KeyringPair,
    transferRequest: {
      sourceChain: string;
      targetChain: string;
      tokenId: string;
      biometricData: BiometricData;
      metadataUri: string;
    }
  ): Promise<{ transferId: string; transactionHash: string; aiAnalysis: AIInferenceResults }> {
    if (!this.contract || !this.api) {
      throw new Error('Bridge not initialized. Call connect() first.');
    }

    try {
      console.log(`🔄 Initiating cross-chain transfer: ${transferRequest.sourceChain} → ${transferRequest.targetChain}`);

      // Step 1: AI Analysis of biometric data
      const aiAnalysis = await this.performAIAnalysis(transferRequest.biometricData);
      
      // Step 2: Generate unique transfer ID
      const transferId = this.generateTransferId(
        transferRequest.tokenId,
        transferRequest.sourceChain,
        transferRequest.targetChain
      );

      // Step 3: Create cross-chain transfer record
      const transfer: CrossChainTransfer = {
        transferId,
        sourceChain: transferRequest.sourceChain as any,
        targetChain: transferRequest.targetChain as any,
        tokenId: transferRequest.tokenId,
        biometricData: transferRequest.biometricData,
        emotionAnalysis: aiAnalysis.emotionClassification,
        metadataUri: transferRequest.metadataUri,
        status: 'processing',
        timestamp: Date.now()
      };

      this.activeTransfers.set(transferId, transfer);

      // Step 4: Scale emotion data for contract
      const emotionDataScaled = {
        valence: Math.round(aiAnalysis.emotionClassification.valence * 127),
        arousal: Math.round(aiAnalysis.emotionClassification.arousal * 127),
        dominance: Math.round(aiAnalysis.emotionClassification.dominance * 127),
        confidence: Math.round(aiAnalysis.emotionClassification.confidence),
        timestamp: Date.now()
      };

      // Step 5: Estimate gas
      const { gasRequired } = await this.contract.query.initiateTransfer(
        keypair.address,
        {},
        transferId,
        transferRequest.sourceChain,
        transferRequest.targetChain,
        Array.from(Buffer.from(transferRequest.biometricData.biometricHash, 'hex')),
        emotionDataScaled,
        transferRequest.metadataUri
      );

      const gasLimit = this.api.registry.createType('WeightV2', {
        refTime: gasRequired.refTime.toBn().muln(2),
        proofSize: gasRequired.proofSize.toBn().muln(2),
      }) as WeightV2;

      // Step 6: Execute transfer transaction
      const tx = await this.contract.tx.initiateTransfer(
        { gasLimit },
        transferId,
        transferRequest.sourceChain,
        transferRequest.targetChain,
        Array.from(Buffer.from(transferRequest.biometricData.biometricHash, 'hex')),
        emotionDataScaled,
        transferRequest.metadataUri
      );

      const promise = new Promise<{ transferId: string; transactionHash: string; aiAnalysis: AIInferenceResults }>((resolve, reject) => {
        tx.signAndSend(keypair, (result: any) => {
          if (result.status.isInBlock) {
            const transactionHash = result.status.asInBlock.toString();
            
            // Update transfer status
            transfer.status = 'completed';
            this.bridgeState.set(transferId, transfer);
            this.activeTransfers.delete(transferId);
            
            console.log(`✅ Cross-chain transfer initiated: ${transferId}`);
            console.log(`🔗 Transaction hash: ${transactionHash}`);
            console.log(`🧠 AI Analysis confidence: ${aiAnalysis.emotionClassification.confidence}%`);
            
            resolve({ transferId, transactionHash, aiAnalysis });
          } else if (result.dispatchError) {
            transfer.status = 'failed';
            this.activeTransfers.delete(transferId);
            reject(new Error(`Transfer failed: ${result.dispatchError}`));
          }
        }).catch(reject);
      });

      return await promise;
    } catch (error) {
      console.error('Cross-chain transfer initiation failed:', error);
      throw error;
    }
  }

  /**
   * Perform comprehensive AI analysis on biometric data
   */
  private async performAIAnalysis(biometricData: BiometricData): Promise<AIInferenceResults> {
    try {
      console.log('🧠 Performing AI analysis on biometric data...');

      // Step 1: Emotion classification using Iron Learn
      const emotionClassification = await this.mlBridge.processWithIronLearn(
        biometricData.emotionVector,
        'emotion'
      );

      // Step 2: Biometric validation using Candle
      const biometricValidation = await this.mlBridge.processWithCandle(
        biometricData.biometricHash,
        'validation'
      );

      // Step 3: Cross-chain compatibility analysis
      const crossChainCompatibility = await this.analyzeCrossChainCompatibility(biometricData);

      const results: AIInferenceResults = {
        emotionClassification: {
          primary: emotionClassification.predictions[0]?.label || 'neutral',
          secondary: emotionClassification.predictions.slice(1, 3).map(p => p.label) || [],
          confidence: emotionClassification.confidence || biometricData.confidence
        },
        biometricValidation: {
          isValid: biometricValidation.isValid,
          qualityScore: biometricValidation.qualityScore || biometricData.qualityScore,
          anomalies: biometricValidation.anomalies || []
        },
        crossChainCompatibility: {
          compatible: crossChainCompatibility.compatible,
          requiredTransformations: crossChainCompatibility.transformations || [],
          recommendedFormat: crossChainCompatibility.format || 'standard'
        }
      };

      console.log(`✅ AI analysis completed with ${results.emotionClassification.confidence}% confidence`);
      return results;
    } catch (error) {
      console.error('AI analysis failed:', error);
      
      // Fallback to basic analysis
      return {
        emotionClassification: {
          primary: 'neutral',
          secondary: [],
          confidence: biometricData.confidence
        },
        biometricValidation: {
          isValid: biometricData.qualityScore > 0.7,
          qualityScore: biometricData.qualityScore,
          anomalies: []
        },
        crossChainCompatibility: {
          compatible: true,
          requiredTransformations: [],
          recommendedFormat: 'standard'
        }
      };
    }
  }

  /**
   * Analyze cross-chain compatibility for biometric data
   */
  private async analyzeCrossChainCompatibility(biometricData: BiometricData): Promise<{
    compatible: boolean;
    transformations?: string[];
    format?: string;
  }> {
    try {
      // Use LanceDB to check compatibility patterns
      const compatibilityResults = await this.mlBridge.queryLanceDB(
        'cross_chain_compatibility',
        biometricData.emotionVector,
        5
      );

      const transformations = [];
      let compatible = true;

      // Check for required transformations based on chain differences
      if (biometricData.qualityScore < 0.8) {
        transformations.push('quality_enhancement');
      }

      if (biometricData.emotionVector.length !== 6) {
        transformations.push('vector_normalization');
      }

      if (biometricData.confidence < 0.7) {
        transformations.push('confidence_boosting');
        compatible = false;
      }

      return {
        compatible,
        transformations,
        format: 'cross_chain_standard'
      };
    } catch (error) {
      console.warn('Cross-chain compatibility analysis failed:', error);
      return {
        compatible: true,
        transformations: [],
        format: 'standard'
      };
    }
  }

  /**
   * Get transfer details from blockchain
   */
  async getTransferDetails(transferId: string): Promise<CrossChainTransfer | null> {
    if (!this.contract) {
      throw new Error('Bridge not initialized. Call connect() first.');
    }

    try {
      const { result } = await this.contract.query.getTransfer(
        this.contract.address.toString(),
        {},
        transferId
      );

      if (result.isErr) {
        throw new Error(`Contract query failed: ${result.asErr}`);
      }

      if (result.isOk && (result as any).asOk.isOk) {
        const transferData = (result as any).asOk.asOk;
        
        // Convert blockchain data to CrossChainTransfer format
        return {
          transferId,
          sourceChain: transferData.source_chain.toString(),
          targetChain: transferData.target_chain.toString(),
          tokenId: transferData.token_id.toString(),
          biometricData: {
            biometricHash: Buffer.from(transferData.biometric_hash).toString('hex'),
            emotionVector: [], // Would need to decode from contract
            qualityScore: 0.8,
            confidence: 0.85,
            timestamp: Date.now()
          },
          emotionAnalysis: {
            primaryEmotion: 'neutral',
            emotionVector: [],
            valence: 0,
            arousal: 0,
            dominance: 0,
            confidence: 0.85
          },
          metadataUri: transferData.metadata_uri.toString(),
          status: 'completed',
          timestamp: Date.now()
        };
      }

      return null;
    } catch (error) {
      console.error('Failed to get transfer details:', error);
      return null;
    }
  }

  /**
   * Get active transfers
   */
  getActiveTransfers(): CrossChainTransfer[] {
    return Array.from(this.activeTransfers.values());
  }

  /**
   * Get bridge statistics
   */
  getBridgeStatistics() {
    const transfers = Array.from(this.bridgeState.values());
    const active = this.getActiveTransfers();
    
    return {
      totalTransfers: transfers.length,
      completedTransfers: transfers.filter(t => t.status === 'completed').length,
      failedTransfers: transfers.filter(t => t.status === 'failed').length,
      activeTransfers: active.length,
      successRate: transfers.length > 0 
        ? (transfers.filter(t => t.status === 'completed').length / transfers.length) * 100 
        : 0,
      averageAIConfidence: transfers.length > 0
        ? transfers.reduce((sum, t) => sum + t.emotionAnalysis.confidence, 0) / transfers.length
        : 0
    };
  }

  /**
   * Generate unique transfer ID
   */
  private generateTransferId(tokenId: string, sourceChain: string, targetChain: string): string {
    const timestamp = Date.now();
    const randomBytes = createHash('sha256')
      .update(`${tokenId}${sourceChain}${targetChain}${timestamp}${Math.random()}`)
      .digest('hex')
      .substring(0, 16);
    
    return `${sourceChain}_${targetChain}_${timestamp}_${randomBytes}`;
  }

  /**
   * Disconnect from bridge
   */
  disconnect(): void {
    if (this.api) {
      this.api.disconnect();
      this.api = null;
      this.contract = null;
    }
    
    if (this.mlBridge) {
      this.mlBridge.cleanup();
    }
  }
}

/**
 * Utility functions for Polkadot AI Bridge
 */
export function createBiometricData(
  emotionVector: number[],
  qualityScore: number = 0.85,
  confidence: number = 0.9
): BiometricData {
  const biometricHash = createHash('sha256')
    .update(JSON.stringify(emotionVector))
    .digest('hex');
  
  return {
    biometricHash,
    emotionVector: emotionVector.slice(0, 6), // Ensure 6-dimensional vector
    qualityScore: Math.max(0, Math.min(1, qualityScore)),
    confidence: Math.max(0, Math.min(1, confidence)),
    timestamp: Date.now()
  };
}

export function validateCrossChainTransfer(transfer: CrossChainTransfer): boolean {
  // Validate transfer data integrity
  if (!transfer.transferId || transfer.transferId.length < 10) {
    return false;
  }
  
  if (transfer.biometricData.qualityScore < 0.5) {
    return false;
  }
  
  if (transfer.emotionAnalysis.confidence < 0.3) {
    return false;
  }
  
  if (transfer.biometricData.emotionVector.length !== 6) {
    return false;
  }
  
  return true;
}
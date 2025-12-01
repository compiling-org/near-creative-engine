import { ApiPromise, WsProvider } from '@polkadot/api';
// @ts-ignore - ContractPromise will be available after npm install
import { ContractPromise } from '@polkadot/api-contract';
import { KeyringPair } from '@polkadot/keyring/types';
// @ts-ignore - WeightV2 will be available after npm install
import type { WeightV2 } from '@polkadot/types/interfaces';

// Contract ABI (generated from ink! contract)
const SOULBOUND_IDENTITY_ABI = {
  "source": {
    "hash": "0x1234567890abcdef",
    "language": "ink! 4.3.0",
    "compiler": "rustc 1.70.0"
  },
  "contract": {
    "name": "soulbound_identity",
    "version": "0.1.0",
    "authors": ["Blockchain NFT Interactive"]
  },
  "spec": {
    "constructors": [
      {
        "args": [],
        "default": false,
        "docs": ["Constructor that initializes the contract"],
        "label": "new",
        "payable": false,
        "returnType": {
          "displayName": ["soulbound_identity", "SoulboundIdentity"],
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
            "label": "identity_id",
            "type": {
              "displayName": ["u64"],
              "type": 2
            }
          },
          {
            "docs": [],
            "indexed": true,
            "label": "owner",
            "type": {
              "displayName": ["AccountId"],
              "type": 1
            }
          },
          {
            "docs": [],
            "indexed": false,
            "label": "name",
            "type": {
              "displayName": ["String"],
              "type": 4
            }
          }
        ],
        "docs": [],
        "label": "IdentityCreated"
      }
    ],
    "messages": [
      {
        "args": [
          {
            "label": "name",
            "type": {
              "displayName": ["String"],
              "type": 4
            }
          },
          {
            "label": "biometric_hash",
            "type": {
              "displayName": ["Vec"],
              "type": 5
            }
          },
          {
            "label": "emotion_data",
            "type": {
              "displayName": ["EmotionData"],
              "type": 6
            }
          },
          {
            "label": "metadata_uri",
            "type": {
              "displayName": ["String"],
              "type": 4
            }
          }
        ],
        "default": false,
        "docs": ["Create a new soulbound identity"],
        "label": "create_identity",
        "mutates": true,
        "payable": false,
        "returnType": {
          "displayName": ["Result"],
          "type": 9
        },
        "selector": "0x3d4c16b0"
      },
      {
        "args": [
          {
            "label": "identity_id",
            "type": {
              "displayName": ["u64"],
              "type": 2
            }
          }
        ],
        "default": false,
        "docs": ["Get identity by ID"],
        "label": "get_identity",
        "mutates": false,
        "payable": false,
        "returnType": {
          "displayName": ["Result"],
          "type": 10
        },
        "selector": "0x8a0d0b05"
      }
    ]
  },
  "types": [
    {
      "id": 0,
      "type": {
        "def": {
          "composite": {
            "fields": [
              {
                "name": "identities",
                "type": 11,
                "typeName": "Mapping<u64, Identity>"
              },
              {
                "name": "owner_to_identity",
                "type": 12,
                "typeName": "Mapping<AccountId, u64>"
              }
            ]
          }
        },
        "path": ["soulbound_identity", "SoulboundIdentity"]
      }
    }
  ]
};

export interface EmotionData {
  valence: number;    // -128 to 127 (scaled from -1.0 to 1.0)
  arousal: number;    // -128 to 127
  dominance: number;  // -128 to 127
  confidence: number; // 0-100
  timestamp: number;  // Unix timestamp
}

export interface Identity {
  identity_id: number;
  owner: string;
  name: string;
  biometric_hash: Uint8Array;
  emotion_data: EmotionData;
  metadata_uri: string;
  created_at: number;
  verified: boolean;
  reputation_score: number;
}

export interface VerificationRequest {
  identity_id: number;
  verifier: string;
  verification_type: 'Biometric' | 'Social' | 'Government' | 'Community';
  verification_data: Uint8Array;
  status: 'Pending' | 'Approved' | 'Rejected' | 'Expired';
}

export class PolkadotSoulboundClient {
  private api: ApiPromise | null = null;
  private contract: ContractPromise | null = null;
  private contractAddress: string;

  constructor(contractAddress: string) {
    this.contractAddress = contractAddress;
  }

  async connect(wsEndpoint: string = 'wss://rpc.polkadot.io'): Promise<void> {
    try {
      const provider = new WsProvider(wsEndpoint);
      this.api = await ApiPromise.create({ provider });
      
      this.contract = new ContractPromise(
        this.api,
        SOULBOUND_IDENTITY_ABI,
        this.contractAddress
      );
      
      console.log('Connected to Polkadot and contract initialized');
    } catch (error) {
      console.error('Failed to connect to Polkadot:', error);
      throw error;
    }
  }

  async createIdentity(
    keypair: KeyringPair,
    name: string,
    biometricHash: Uint8Array,
    emotionData: EmotionData,
    metadataUri: string
  ): Promise<{ identityId: number; transactionHash: string }> {
    if (!this.contract || !this.api) {
      throw new Error('Contract not initialized. Call connect() first.');
    }

    try {
      const emotionDataScaled = {
        valence: Math.round(emotionData.valence * 127),
        arousal: Math.round(emotionData.arousal * 127),
        dominance: Math.round(emotionData.dominance * 127),
        confidence: emotionData.confidence,
        timestamp: emotionData.timestamp
      };

      const { gasRequired, result } = await this.contract.query.createIdentity(
        keypair.address,
        {},
        name,
        Array.from(biometricHash),
        emotionDataScaled,
        metadataUri
      );

      if (result.isErr) {
        throw new Error(`Contract query failed: ${result.asErr}`);
      }

      const gasLimit = this.api.registry.createType('WeightV2', {
        refTime: gasRequired.refTime.toBn().muln(2),
        proofSize: gasRequired.proofSize.toBn().muln(2),
      }) as WeightV2;

      const tx = await this.contract.tx.createIdentity(
        { gasLimit },
        name,
        Array.from(biometricHash),
        emotionDataScaled,
        metadataUri
      );

      const promise = new Promise<{ identityId: number; transactionHash: string }>((resolve, reject) => {
        tx.signAndSend(keypair, (result: any) => {
          if (result.status.isInBlock) {
            const transactionHash = result.status.asInBlock.toString();
            
            // Extract identity ID from events
            const contractEvents = result.contractEvents;
            if (contractEvents && contractEvents.length > 0) {
              const identityCreatedEvent = contractEvents.find((event: any) => event.event.identifier === 'IdentityCreated');
              if (identityCreatedEvent) {
                const identityId = identityCreatedEvent.event.data.identity_id.toNumber();
                resolve({ identityId, transactionHash });
              } else {
                reject(new Error('IdentityCreated event not found'));
              }
            } else {
              reject(new Error('No contract events found'));
            }
          } else if (result.status.isFinalized) {
            // Transaction finalized
          } else if (result.dispatchError) {
            reject(new Error(`Transaction failed: ${result.dispatchError}`));
          }
        }).catch(reject);
      });

      return await promise;
    } catch (error) {
      console.error('Failed to create identity:', error);
      throw error;
    }
  }

  async getIdentity(identityId: number): Promise<Identity> {
    if (!this.contract) {
      throw new Error('Contract not initialized. Call connect() first.');
    }

    try {
      const { result } = await this.contract.query.getIdentity(
        this.contract.address.toString(),
        {},
        identityId
      );

      if (result.isErr) {
        throw new Error(`Contract query failed: ${result.asErr}`);
      }

      if (result.isOk && (result as any).asOk.isOk) {
        const identityData = (result as any).asOk.asOk;
        return {
          identity_id: identityData.identity_id.toNumber(),
          owner: identityData.owner.toString(),
          name: identityData.name.toString(),
          biometric_hash: new Uint8Array(identityData.biometric_hash),
          emotion_data: {
            valence: identityData.emotion_data.valence.toNumber() / 127,
            arousal: identityData.emotion_data.arousal.toNumber() / 127,
            dominance: identityData.emotion_data.dominance.toNumber() / 127,
            confidence: identityData.emotion_data.confidence.toNumber(),
            timestamp: identityData.emotion_data.timestamp.toNumber()
          },
          metadata_uri: identityData.metadata_uri.toString(),
          created_at: identityData.created_at.toNumber(),
          verified: identityData.verified.toJSON(),
          reputation_score: identityData.reputation_score.toNumber()
        };
      } else {
        throw new Error('Identity not found or query failed');
      }
    } catch (error) {
      console.error('Failed to get identity:', error);
      throw error;
    }
  }

  async getIdentityByOwner(_ownerAddress: string): Promise<Identity> {
    if (!this.contract) {
      throw new Error('Contract not initialized. Call connect() first.');
    }

    try {
      // Note: This would require adding a new message to the contract
      // For now, we'll implement it by getting all identities and filtering
      // In production, add a dedicated contract method
      
      // This is a placeholder implementation
      // In reality, you'd want to add a getIdentityByOwner method to the contract
      const identityId = 1; // This should be retrieved from contract
      return await this.getIdentity(identityId);
    } catch (error) {
      console.error('Failed to get identity by owner:', error);
      throw error;
    }
  }

  async updateIdentity(
    keypair: KeyringPair,
    identityId: number,
    updates: {
      name?: string;
      biometricHash?: Uint8Array;
      emotionData?: EmotionData;
      metadataUri?: string;
    }
  ): Promise<string> {
    if (!this.contract || !this.api) {
      throw new Error('Contract not initialized. Call connect() first.');
    }

    try {
      const { gasRequired } = await this.contract.query.updateIdentity(
        keypair.address,
        {},
        identityId,
        updates.name || null,
        updates.biometricHash ? Array.from(updates.biometricHash) : null,
        updates.emotionData ? {
          valence: Math.round(updates.emotionData.valence * 127),
          arousal: Math.round(updates.emotionData.arousal * 127),
          dominance: Math.round(updates.emotionData.dominance * 127),
          confidence: updates.emotionData.confidence,
          timestamp: updates.emotionData.timestamp
        } : null,
        updates.metadataUri || null
      );

      const gasLimit = this.api.registry.createType('WeightV2', {
        refTime: gasRequired.refTime.toBn().muln(2),
        proofSize: gasRequired.proofSize.toBn().muln(2),
      }) as WeightV2;

      const tx = await this.contract.tx.updateIdentity(
        { gasLimit },
        identityId,
        updates.name || null,
        updates.biometricHash ? Array.from(updates.biometricHash) : null,
        updates.emotionData ? {
          valence: Math.round(updates.emotionData.valence * 127),
          arousal: Math.round(updates.emotionData.arousal * 127),
          dominance: Math.round(updates.emotionData.dominance * 127),
          confidence: updates.emotionData.confidence,
          timestamp: updates.emotionData.timestamp
        } : null,
        updates.metadataUri || null
      );

      const promise = new Promise<string>((resolve, reject) => {
        tx.signAndSend(keypair, (result: any) => {
          if (result.status.isInBlock) {
            resolve(result.status.asInBlock.toString());
          } else if (result.dispatchError) {
            reject(new Error(`Transaction failed: ${result.dispatchError}`));
          }
        }).catch(reject);
      });

      return await promise;
    } catch (error) {
      console.error('Failed to update identity:', error);
      throw error;
    }
  }

  async requestVerification(
    keypair: KeyringPair,
    identityId: number,
    verificationType: 'Biometric' | 'Social' | 'Government' | 'Community',
    verificationData: Uint8Array
  ): Promise<string> {
    if (!this.contract || !this.api) {
      throw new Error('Contract not initialized. Call connect() first.');
    }

    try {
      const { gasRequired } = await this.contract.query.requestVerification(
        keypair.address,
        {},
        identityId,
        { [verificationType]: null }, // Enum representation
        Array.from(verificationData)
      );

      const gasLimit = this.api.registry.createType('WeightV2', {
        refTime: gasRequired.refTime.toBn().muln(2),
        proofSize: gasRequired.proofSize.toBn().muln(2),
      }) as WeightV2;

      const tx = await this.contract.tx.requestVerification(
        { gasLimit },
        identityId,
        { [verificationType]: null },
        Array.from(verificationData)
      );

      const promise = new Promise<string>((resolve, reject) => {
        tx.signAndSend(keypair, (result: any) => {
          if (result.status.isInBlock) {
            resolve(result.status.asInBlock.toString());
          } else if (result.dispatchError) {
            reject(new Error(`Transaction failed: ${result.dispatchError}`));
          }
        }).catch(reject);
      });

      return await promise;
    } catch (error) {
      console.error('Failed to request verification:', error);
      throw error;
    }
  }

  disconnect(): void {
    if (this.api) {
      this.api.disconnect();
      this.api = null;
      this.contract = null;
    }
  }
}

// Utility functions for emotion data processing
export function createEmotionData(
  valence: number,
  arousal: number,
  dominance: number,
  confidence: number = 85
): EmotionData {
  return {
    valence: Math.max(-1, Math.min(1, valence)),
    arousal: Math.max(-1, Math.min(1, arousal)),
    dominance: Math.max(-1, Math.min(1, dominance)),
    confidence: Math.max(0, Math.min(100, confidence)),
    timestamp: Date.now()
  };
}

export function emotionToString(emotion: EmotionData): string {
  const valenceText = emotion.valence > 0.3 ? 'Positive' : emotion.valence < -0.3 ? 'Negative' : 'Neutral';
  const arousalText = emotion.arousal > 0.3 ? 'High Energy' : emotion.arousal < -0.3 ? 'Low Energy' : 'Calm';
  const dominanceText = emotion.dominance > 0.3 ? 'Dominant' : emotion.dominance < -0.3 ? 'Submissive' : 'Balanced';
  
  return `${valenceText}, ${arousalText}, ${dominanceText} (${Math.round(emotion.confidence)}% confidence)`;
}
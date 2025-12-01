import { connect, keyStores, WalletConnection } from 'near-api-js';
import { WASMMLBridge } from './unified-ai-ml-integration.js';
import { createHash } from 'crypto';

/**
 * Bitte Protocol AI Agent Integration - Enhanced with Unified ML Pipeline
 * Integrates Iron Learn, LanceDB, and Candle frameworks for advanced AI capabilities
 */

export interface BitteAIAgent {
  id: string;
  name: string;
  capabilities: string[];
  personality: {
    tone: 'friendly' | 'professional' | 'creative' | 'analytical';
    creativity: number;
    empathy: number;
    intelligence: number;
  };
  emotionVector: number[];
  reputation: number;
  lastActive: number;
}

export interface AIInteraction {
  id: string;
  agentId: string;
  userId: string;
  type: 'emotion_recognition' | 'biometric_verification' | 'cross_chain_analysis' | 'governance_prediction' | 'federated_learning' | 'nft_interaction' | 'soulbound_verification';
  input: any;
  output: any;
  emotionContext: {
    userEmotion: number[];
    agentEmotion: number[];
    emotionalCompatibility: number;
  };
  timestamp: number;
  chain: string;
  metadataUri?: string;
}

export interface FederatedLearningSession {
  id: string;
  modelType: 'emotion' | 'biometric' | 'cross_chain' | 'governance';
  participants: string[];
  updates: Array<{
    participantId: string;
    update: any;
    emotionVector: number[];
    timestamp: number;
  }>;
  globalModel: any;
  round: number;
  status: 'active' | 'completed' | 'failed';
  aiAnalysis: any;
}

export interface CrossChainAIAnalysis {
  chains: string[];
  dataType: 'biometric' | 'emotion' | 'nft' | 'governance';
  analysisType: 'compatibility' | 'optimization' | 'prediction' | 'anomaly_detection';
  aiResults: {
    compatibility: number;
    recommendations: string[];
    riskAssessment: number;
    emotionalContext: number[];
  };
  timestamp: number;
}

export class BitteProtocolAIEnhanced {
  private near: any;
  private wallet: any;
  private mlBridge: WASMMLBridge;
  private agents: Map<string, BitteAIAgent> = new Map();
  private interactions: Map<string, AIInteraction> = new Map();
  private federatedSessions: Map<string, FederatedLearningSession> = new Map();
  private config: any;

  constructor(config: any) {
    this.config = config;
    this.mlBridge = new WASMMLBridge();
  }

  async initialize(): Promise<void> {
    try {
      // Initialize NEAR connection
      const nearConfig = {
        networkId: this.config.networkId || 'testnet',
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: this.config.nodeUrl || 'https://rpc.testnet.near.org',
        walletUrl: this.config.walletUrl || 'https://wallet.testnet.near.org',
        helperUrl: this.config.helperUrl || 'https://helper.testnet.near.org',
        explorerUrl: this.config.explorerUrl || 'https://explorer.testnet.near.org',
      };

      this.near = await connect(nearConfig);
      this.wallet = new WalletConnection(this.near, 'bitte-ai-enhanced');

      // Initialize unified ML pipeline
      await this.mlBridge.initialize();

      // Load existing AI agents
      await this.loadAIAgents();

      console.log('🤖 Bitte Protocol AI Enhanced initialized successfully');
    } catch (error) {
      console.error('Failed to initialize Bitte Protocol AI Enhanced:', error);
      throw error;
    }
  }

  /**
   * Create AI agent with emotion-based personality
   */
  async createAIAgent(
    name: string,
    emotionVector: number[],
    capabilities: string[]
  ): Promise<BitteAIAgent> {
    try {
      // Analyze emotion vector to determine personality traits
      const personalityAnalysis = await this.analyzeEmotionPersonality(emotionVector);
      
      const agent: BitteAIAgent = {
        id: `agent_${createHash('sha256').update(name + Date.now()).digest('hex').substring(0, 16)}`,
        name,
        capabilities,
        personality: personalityAnalysis,
        emotionVector,
        reputation: 0.5, // Start with neutral reputation
        lastActive: Date.now()
      };

      this.agents.set(agent.id, agent);
      
      // Store agent on NEAR blockchain
      await this.storeAgentOnChain(agent);
      
      console.log(`🤖 Created AI agent: ${name} with personality: ${personalityAnalysis.tone}`);
      return agent;
    } catch (error) {
      console.error('Failed to create AI agent:', error);
      throw error;
    }
  }

  /**
   * Analyze emotion vector to determine personality traits
   */
  private async analyzeEmotionPersonality(emotionVector: number[]): Promise<BitteAIAgent['personality']> {
    try {
      // Use Iron Learn to analyze emotion patterns
      const analysis = await this.mlBridge.processWithIronLearn(emotionVector, 'personality');
      
      // Map emotion analysis to personality traits
      const [happiness, sadness, anger, fear, surprise, neutral] = emotionVector;
      
      let tone: BitteAIAgent['personality']['tone'] = 'friendly';
      if (happiness > 0.7) tone = 'friendly';
      else if (anger > 0.5) tone = 'professional';
      else if (surprise > 0.6) tone = 'creative';
      else if (neutral > 0.8) tone = 'analytical';

      return {
        tone,
        creativity: Math.min(1, surprise + 0.3),
        empathy: Math.min(1, happiness + (1 - anger) * 0.5),
        intelligence: Math.min(1, neutral + 0.2)
      };
    } catch (error) {
      console.warn('AI personality analysis failed, using defaults:', error);
      return {
        tone: 'friendly',
        creativity: 0.6,
        empathy: 0.8,
        intelligence: 0.7
      };
    }
  }

  /**
   * Process AI interaction with emotional context
   */
  async processAIInteraction(
    agentId: string,
    userId: string,
    interactionType: AIInteraction['type'],
    input: any
  ): Promise<AIInteraction> {
    try {
      const agent = this.agents.get(agentId);
      if (!agent) {
        throw new Error(`AI agent ${agentId} not found`);
      }

      // Analyze user emotional state from input
      const userEmotion = await this.extractUserEmotion(input);
      
      // Calculate emotional compatibility between user and agent
      const emotionalCompatibility = this.calculateEmotionalCompatibility(
        userEmotion,
        agent.emotionVector
      );

      // Process interaction based on type
      const output = await this.processInteractionByType(
        interactionType,
        input,
        agent,
        userEmotion
      );

      const interaction: AIInteraction = {
        id: `interaction_${createHash('sha256').update(agentId + userId + Date.now()).digest('hex').substring(0, 16)}`,
        agentId,
        userId,
        type: interactionType,
        input,
        output,
        emotionContext: {
          userEmotion,
          agentEmotion: agent.emotionVector,
          emotionalCompatibility
        },
        timestamp: Date.now(),
        chain: 'near'
      };

      this.interactions.set(interaction.id, interaction);
      
      // Update agent reputation based on interaction success
      await this.updateAgentReputation(agentId, interaction);
      
      // Store interaction on chain
      await this.storeInteractionOnChain(interaction);

      console.log(`🤖 AI interaction processed: ${interactionType} (${emotionalCompatibility}% compatibility)`);
      return interaction;
    } catch (error) {
      console.error('AI interaction failed:', error);
      throw error;
    }
  }

  /**
   * Extract user emotion from interaction input
   */
  private async extractUserEmotion(input: any): Promise<number[]> {
    try {
      if (input.emotionVector) {
        return input.emotionVector;
      }
      
      if (input.text) {
        // Use Iron Learn for text emotion analysis
        const textAnalysis = await this.mlBridge.processWithIronLearn(
          [input.text.length, input.text.split(' ').length, 0.5, 0.5, 0.5, 0.5],
          'text_emotion'
        );
        return textAnalysis.predictions[0]?.vector || [0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
      }
      
      if (input.biometricData) {
        return input.biometricData.emotionVector;
      }
      
      return [0.5, 0.5, 0.5, 0.5, 0.5, 0.5]; // Neutral emotion
    } catch (error) {
      console.warn('User emotion extraction failed, using neutral:', error);
      return [0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
    }
  }

  /**
   * Calculate emotional compatibility between user and agent
   */
  private calculateEmotionalCompatibility(userEmotion: number[], agentEmotion: number[]): number {
    try {
      // Calculate cosine similarity between emotion vectors
      const dotProduct = userEmotion.reduce((sum, val, i) => sum + val * agentEmotion[i], 0);
      const userMagnitude = Math.sqrt(userEmotion.reduce((sum, val) => sum + val * val, 0));
      const agentMagnitude = Math.sqrt(agentEmotion.reduce((sum, val) => sum + val * val, 0));
      
      const similarity = dotProduct / (userMagnitude * agentMagnitude);
      return Math.round(similarity * 100);
    } catch (error) {
      console.warn('Emotional compatibility calculation failed:', error);
      return 50; // 50% compatibility as default
    }
  }

  /**
   * Process interaction based on type with AI enhancement
   */
  private async processInteractionByType(
    type: AIInteraction['type'],
    input: any,
    agent: BitteAIAgent,
    userEmotion: number[]
  ): Promise<any> {
    switch (type) {
      case 'emotion_recognition':
        return await this.mlBridge.processWithIronLearn(userEmotion, 'emotion');
        
      case 'biometric_verification':
        return await this.mlBridge.processWithCandle(input.biometricData, 'verification');
        
      case 'cross_chain_analysis':
        return await this.performCrossChainAIAnalysis(input);
        
      case 'governance_prediction':
        return await this.predictGovernanceOutcome(input);
        
      case 'federated_learning':
        return await this.coordinateFederatedLearning(input);
        
      case 'nft_interaction':
        return await this.analyzeNFTInteraction(input);
        
      case 'soulbound_verification':
        return await this.verifySoulboundWithAI(input);
        
      default:
        throw new Error(`Unknown interaction type: ${type}`);
    }
  }

  /**
   * Perform cross-chain AI analysis with unified ML pipeline
   */
  private async performCrossChainAIAnalysis(input: any): Promise<CrossChainAIAnalysis> {
    try {
      const { chains, dataType, analysisType } = input;
      
      // Analyze cross-chain compatibility using LanceDB
      const compatibilityResults = await this.mlBridge.queryLanceDB(
        'cross_chain_compatibility',
        [chains.length, dataType.length, analysisType.length, 0.5, 0.5, 0.5],
        5
      );
      
      // Use Iron Learn for optimization analysis
      const optimizationResults = await this.mlBridge.processWithIronLearn(
        [chains.length, dataType.length, analysisType.length, 0.7, 0.8, 0.6],
        'cross_chain_optimization'
      );
      
      // Use Candle for anomaly detection
      const anomalyResults = await this.mlBridge.processWithCandle(
        createHash('sha256').update(JSON.stringify(input)).digest('hex'),
        'anomaly_detection'
      );
      
      return {
        chains,
        dataType,
        analysisType,
        aiResults: {
          compatibility: compatibilityResults[0]?.score || 75,
          recommendations: [
            'Optimize data routing based on chain performance',
            'Implement cross-chain data validation',
            'Use Filecoin for large AI model storage'
          ],
          riskAssessment: Math.max(0, 100 - (anomalyResults.anomalyScore || 20)),
          emotionalContext: [0.6, 0.4, 0.7, 0.8, 0.5, 0.6]
        },
        timestamp: Date.now()
      };
    } catch (error) {
      console.error('Cross-chain AI analysis failed:', error);
      throw error;
    }
  }

  /**
   * Coordinate federated learning with emotion-aware participants
   */
  private async coordinateFederatedLearning(input: any): Promise<FederatedLearningSession> {
    try {
      const { modelType, participants, updates } = input;
      
      const sessionId = `federated_${modelType}_${Date.now()}`;
      
      // Analyze participant emotion vectors for optimal coordination
      const participantAnalysis = await Promise.all(
        participants.map(async (participant: any) => {
          const emotionAnalysis = await this.mlBridge.processWithIronLearn(
            participant.emotionVector,
            'federated_participant'
          );
          return {
            participantId: participant.id,
            emotionVector: participant.emotionVector,
            trustScore: emotionAnalysis.confidence,
            contributionQuality: emotionAnalysis.quality
          };
        })
      );
      
      // Create federated learning session
      const session: FederatedLearningSession = {
        id: sessionId,
        modelType,
        participants: participants.map((p: any) => p.id),
        updates: updates || [],
        globalModel: null,
        round: 1,
        status: 'active',
        aiAnalysis: {
          participantAnalysis,
          coordinationStrategy: 'emotion_weighted_average',
          expectedImprovement: 0.15
        }
      };
      
      this.federatedSessions.set(sessionId, session);
      
      console.log(`🤖 Federated learning session created: ${sessionId} with ${participants.length} participants`);
      return session;
    } catch (error) {
      console.error('Federated learning coordination failed:', error);
      throw error;
    }
  }

  /**
   * Update agent reputation based on interaction success
   */
  private async updateAgentReputation(agentId: string, interaction: AIInteraction): Promise<void> {
    try {
      const agent = this.agents.get(agentId);
      if (!agent) return;
      
      // Calculate reputation update based on interaction quality
      const compatibilityBonus = interaction.emotionContext.emotionalCompatibility / 1000; // 0-0.1
      const successBonus = interaction.output?.confidence ? interaction.output.confidence / 1000 : 0;
      
      agent.reputation = Math.max(0, Math.min(1, agent.reputation + compatibilityBonus + successBonus));
      agent.lastActive = Date.now();
      
      console.log(`📊 Agent ${agent.name} reputation updated: ${(agent.reputation * 100).toFixed(1)}%`);
    } catch (error) {
      console.warn('Agent reputation update failed:', error);
    }
  }

  /**
   * Store agent data on NEAR blockchain
   */
  private async storeAgentOnChain(agent: BitteAIAgent): Promise<void> {
    if (!this.wallet.isSignedIn()) return;
    
    try {
      const account = this.wallet.account();
      const contractId = this.config.nearContractId;
      
      await account.functionCall({
        contractId,
        methodName: 'store_ai_agent',
        args: {
          agent_id: agent.id,
          name: agent.name,
          capabilities: agent.capabilities,
          personality: agent.personality,
          emotion_vector: agent.emotionVector,
          reputation: agent.reputation,
          timestamp: agent.lastActive
        },
        gas: '300000000000000',
        attachedDeposit: '0'
      });
      
      console.log(`🔗 Agent ${agent.name} stored on NEAR blockchain`);
    } catch (error) {
      console.warn('Failed to store agent on chain:', error);
    }
  }

  /**
   * Store interaction data on IPFS/Filecoin
   */
  private async storeInteractionOnChain(interaction: AIInteraction): Promise<void> {
    try {
      // Create interaction metadata
      const metadata = {
        interaction_id: interaction.id,
        agent_id: interaction.agentId,
        user_id: interaction.userId,
        type: interaction.type,
        emotion_context: interaction.emotionContext,
        timestamp: interaction.timestamp,
        ai_analysis: interaction.output
      };
      
      // Store on IPFS
      const { create } = await import('ipfs-http-client');
      const ipfs = create({ url: 'https://ipfs.infura.io:5001/api/v0' });
      
      const { cid } = await ipfs.add(JSON.stringify(metadata));
      interaction.metadataUri = `ipfs://${cid}`;
      
      console.log(`💾 Interaction stored on IPFS: ${cid}`);
    } catch (error) {
      console.warn('Failed to store interaction on chain:', error);
    }
  }

  /**
   * Get AI agent by ID
   */
  getAgent(agentId: string): BitteAIAgent | undefined {
    return this.agents.get(agentId);
  }

  /**
   * Get all AI agents
   */
  getAllAgents(): BitteAIAgent[] {
    return Array.from(this.agents.values());
  }

  /**
   * Get interaction by ID
   */
  getInteraction(interactionId: string): AIInteraction | undefined {
    return this.interactions.get(interactionId);
  }

  /**
   * Get recent interactions
   */
  getRecentInteractions(limit: number = 10): AIInteraction[] {
    return Array.from(this.interactions.values())
      .sort((a, b) => b.timestamp - a.timestamp)
      .slice(0, limit);
  }

  /**
   * Get federated learning session
   */
  getFederatedSession(sessionId: string): FederatedLearningSession | undefined {
    return this.federatedSessions.get(sessionId);
  }

  /**
   * Get system statistics
   */
  getSystemStatistics() {
    return {
      totalAgents: this.agents.size,
      totalInteractions: this.interactions.size,
      activeFederatedSessions: this.federatedSessions.size,
      averageAgentReputation: Array.from(this.agents.values())
        .reduce((sum, agent) => sum + agent.reputation, 0) / this.agents.size || 0,
      topCapabilities: this.getTopCapabilities(),
      emotionalCompatibilityDistribution: this.getEmotionalCompatibilityStats()
    };
  }

  /**
   * Get top capabilities across all agents
   */
  private getTopCapabilities(): Array<{ capability: string; count: number }> {
    const capabilityCounts = new Map<string, number>();
    
    for (const agent of this.agents.values()) {
      for (const capability of agent.capabilities) {
        capabilityCounts.set(capability, (capabilityCounts.get(capability) || 0) + 1);
      }
    }
    
    return Array.from(capabilityCounts.entries())
      .sort(([,a], [,b]) => b - a)
      .slice(0, 5)
      .map(([capability, count]) => ({ capability, count }));
  }

  /**
   * Get emotional compatibility statistics
   */
  private getEmotionalCompatibilityStats(): Array<{ range: string; count: number }> {
    const ranges = [
      { range: '0-20%', min: 0, max: 20 },
      { range: '21-40%', min: 21, max: 40 },
      { range: '41-60%', min: 41, max: 60 },
      { range: '61-80%', min: 61, max: 80 },
      { range: '81-100%', min: 81, max: 100 }
    ];
    
    const stats = ranges.map(({ range, min, max }) => ({
      range,
      count: Array.from(this.interactions.values())
        .filter(interaction => {
          const compatibility = interaction.emotionContext.emotionalCompatibility;
          return compatibility >= min && compatibility <= max;
        }).length
    }));
    
    return stats;
  }

  /**
   * Cleanup resources
   */
  async cleanup(): Promise<void> {
    await this.mlBridge.cleanup();
    console.log('🧹 Bitte Protocol AI Enhanced cleanup completed');
  }
}

/**
 * Utility functions for Bitte Protocol AI Enhanced
 */

export function createEmotionVector(
  happiness: number,
  sadness: number,
  anger: number,
  fear: number,
  surprise: number,
  neutral: number
): number[] {
  return [
    Math.max(0, Math.min(1, happiness)),
    Math.max(0, Math.min(1, sadness)),
    Math.max(0, Math.min(1, anger)),
    Math.max(0, Math.min(1, fear)),
    Math.max(0, Math.min(1, surprise)),
    Math.max(0, Math.min(1, neutral))
  ];
}

export function validateEmotionVector(vector: number[]): boolean {
  return vector.length === 6 && vector.every(val => val >= 0 && val <= 1);
}

export async function runBitteAIExamples(): Promise<void> {
  console.log('🚀 Starting Bitte Protocol AI Enhanced Examples...\n');
  
  const config = {
    networkId: 'testnet',
    nodeUrl: 'https://rpc.testnet.near.org',
    nearContractId: 'bitte-ai-enhanced.testnet'
  };
  
  const bitteAI = new BitteProtocolAIEnhanced(config);
  
  try {
    await bitteAI.initialize();
    
    // Example 1: Create AI agents with different personalities
    console.log('🤖 Creating AI agents with emotion-based personalities...');
    
    const friendlyAgent = await bitteAI.createAIAgent(
      'Friendly Assistant',
      createEmotionVector(0.9, 0.1, 0.1, 0.2, 0.6, 0.5),
      ['emotion_recognition', 'user_support', 'recommendations']
    );
    
    const creativeAgent = await bitteAI.createAIAgent(
      'Creative Designer',
      createEmotionVector(0.7, 0.3, 0.2, 0.3, 0.9, 0.4),
      ['nft_design', 'creative_suggestions', 'art_generation']
    );
    
    const analyticalAgent = await bitteAI.createAIAgent(
      'Data Analyst',
      createEmotionVector(0.5, 0.2, 0.1, 0.4, 0.3, 0.8),
      ['cross_chain_analysis', 'governance_prediction', 'data_analysis']
    );
    
    console.log(`✅ Created ${bitteAI.getAllAgents().length} AI agents`);
    
    // Example 2: Process AI interactions with emotional context
    console.log('\n💬 Processing AI interactions with emotional context...');
    
    const interaction1 = await bitteAI.processAIInteraction(
      friendlyAgent.id,
      'user_123',
      'emotion_recognition',
      {
        text: 'I feel happy about my NFT collection!',
        emotionVector: createEmotionVector(0.8, 0.2, 0.1, 0.2, 0.6, 0.5)
      }
    );
    
    console.log(`🎯 Interaction compatibility: ${interaction1.emotionContext.emotionalCompatibility}%`);
    
    // Example 3: Cross-chain AI analysis
    console.log('\n🌉 Performing cross-chain AI analysis...');
    
    const crossChainAnalysis = await bitteAI.processAIInteraction(
      analyticalAgent.id,
      'user_456',
      'cross_chain_analysis',
      {
        chains: ['near', 'solana', 'filecoin'],
        dataType: 'biometric',
        analysisType: 'compatibility'
      }
    );
    
    console.log(`🔗 Cross-chain compatibility: ${crossChainAnalysis.output.aiResults.compatibility}%`);
    
    // Example 4: Federated learning coordination
    console.log('\n🌍 Coordinating federated learning session...');
    
    const federatedSession = await bitteAI.processAIInteraction(
      analyticalAgent.id,
      'coordinator_1',
      'federated_learning',
      {
        modelType: 'emotion',
        participants: [
          { id: 'participant_1', emotionVector: createEmotionVector(0.7, 0.3, 0.2, 0.4, 0.6, 0.5) },
          { id: 'participant_2', emotionVector: createEmotionVector(0.8, 0.2, 0.1, 0.3, 0.7, 0.6) },
          { id: 'participant_3', emotionVector: createEmotionVector(0.6, 0.4, 0.3, 0.5, 0.5, 0.7) }
        ],
        updates: []
      }
    );
    
    console.log(`🤝 Federated session created: ${federatedSession.output.id}`);
    
    // Display system statistics
    console.log('\n📊 System Statistics:');
    const stats = bitteAI.getSystemStatistics();
    console.log(`  Total agents: ${stats.totalAgents}`);
    console.log(`  Total interactions: ${stats.totalInteractions}`);
    console.log(`  Average agent reputation: ${(stats.averageAgentReputation * 100).toFixed(1)}%`);
    
    console.log('\n✅ Bitte Protocol AI Enhanced examples completed successfully!');
    
  } catch (error) {
    console.error('❌ Bitte AI examples failed:', error);
  } finally {
    await bitteAI.cleanup();
  }
}
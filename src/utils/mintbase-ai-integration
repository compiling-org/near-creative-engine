import { connect, keyStores, WalletConnection, Contract } from 'near-api-js';
import { WASMMLBridge } from './unified-ai-ml-integration';
import { FilecoinStorageClient } from './filecoin-storage';
import { createHash } from 'crypto';

export class MintbaseAIIntegration {
  constructor(config) {
    this.config = config;
    this.aiBridge = new WASMMLBridge();
    this.isConnected = false;
    
    if (config.web3StorageApiKey) {
      this.storageClient = new FilecoinStorageClient(config.web3StorageApiKey);
    }
  }

  /**
   * Initialize NEAR connection and wallet
   */
  async initialize() {
    try {
      console.log('🔌 Initializing Mintbase AI integration...');
      
      const config = {
        networkId: this.config.networkId,
        keyStore: new keyStores.BrowserLocalStorageKeyStore(),
        nodeUrl: this.config.networkId === 'mainnet' 
          ? 'https://rpc.mainnet.near.org'
          : 'https://rpc.testnet.near.org',
        walletUrl: this.config.networkId === 'mainnet'
          ? 'https://wallet.near.org'
          : 'https://wallet.testnet.near.org',
        helperUrl: this.config.networkId === 'mainnet'
          ? 'https://helper.mainnet.near.org'
          : 'https://helper.testnet.near.org',
        explorerUrl: this.config.networkId === 'mainnet'
          ? 'https://explorer.near.org'
          : 'https://explorer.testnet.near.org',
      };

      this.connection = await connect(config);
      this.wallet = new WalletConnection(this.connection, 'mintbase-ai-integration');
      
      if (this.wallet.isSignedIn()) {
        await this.initializeContracts();
        this.isConnected = true;
        console.log('✅ Mintbase AI integration initialized successfully');
      } else {
        console.log('⚠️ Wallet not signed in');
      }
      
    } catch (error) {
      console.error('❌ Mintbase initialization failed:', error);
      throw error;
    }
  }

  /**
   * Initialize contract connections
   */
  async initializeContracts() {
    if (!this.wallet) throw new Error('Wallet not initialized');

    // Main NFT contract
    this.contract = new Contract(
      this.wallet.account(),
      this.config.contractId,
      {
        viewMethods: ['nft_token', 'nft_tokens_for_owner', 'get_ai_analysis', 'get_modurust_tool', 'get_nuwe_session'],
        changeMethods: ['mint_modurust_ai', 'update_modurust_data', 'store_nuwe_session', 'store_ai_analysis'],
      }
    );

    // MODURUST contract
    if (this.config.modurustContract) {
      this.modurustContract = new Contract(
        this.wallet.account(),
        this.config.modurustContract,
        {
          viewMethods: ['get_tool', 'get_tool_analysis', 'get_tool_library', 'get_creator_tools'],
          changeMethods: ['create_tool', 'update_tool', 'analyze_tool', 'publish_tool'],
        }
      );
    }

    // NUWE contract
    if (this.config.nuweContract) {
      this.nuweContract = new Contract(
        this.wallet.account(),
        this.config.nuweContract,
        {
          viewMethods: ['get_session', 'get_session_analysis', 'get_user_sessions'],
          changeMethods: ['create_session', 'update_session', 'analyze_session'],
        }
      );
    }

    // DAO contract
    if (this.config.daoContract) {
      this.daoContract = new Contract(
        this.wallet.account(),
        this.config.daoContract,
        {
          viewMethods: ['get_proposal', 'get_proposal_analysis', 'get_dao_metrics'],
          changeMethods: ['create_proposal', 'vote_on_proposal', 'analyze_proposal'],
        }
      );
    }
  }

  /**
   * Connect wallet
   */
  async connectWallet() {
    if (!this.wallet) throw new Error('Wallet not initialized');

    await this.wallet.requestSignIn({
      contractId: this.config.contractId,
      methodNames: ['mint_modurust_ai', 'update_modurust_data', 'store_nuwe_session', 'store_ai_analysis'],
    });

    await this.initializeContracts();
    this.isConnected = true;
  }

  /**
   * Analyze MODURUST tool with AI
   */
  async analyzeMODURUSTTool(tool) {
    try {
      console.log('🔧 Analyzing MODURUST tool with AI...');
      
      // Extract tool features for AI analysis
      const toolFeatures = this.extractToolFeatures(tool);
      
      // Use Iron Learn for tool complexity analysis
      const complexityAnalysis = await this.aiBridge.processWithIronLearn({
        features: toolFeatures,
        codeComplexity: this.analyzeCodeComplexity(tool.code),
        parameterCount: tool.parameters.length,
        dependencyCount: tool.dependencies.length
      }, 'tool_complexity');
      
      // Analyze creative potential
      const creativeAnalysis = await this.aiBridge.classifyCreativeStyle({
        toolType: tool.type,
        parameters: tool.parameters,
        code: tool.code,
        creator: tool.creator
      });
      
      // Evaluate market potential
      const marketAnalysis = await this.evaluateMarketPotential(tool, creativeAnalysis);
      
      // Generate technical evaluation
      const technicalEvaluation = this.generateTechnicalEvaluation(tool, complexityAnalysis);
      
      const aiAnalysis = {
        complexity: complexityAnalysis.complexityScore || 0.5,
        performance: technicalEvaluation.performanceScore,
        creativity: creativeAnalysis.creativityScore || 0.5,
        usability: this.calculateUsabilityScore(tool),
        tags: this.generateToolTags(tool, creativeAnalysis),
        recommendations: this.generateToolRecommendations(tool, technicalEvaluation),
        marketPotential: marketAnalysis.score
      };
      
      return {
        toolId: tool.toolId,
        aiAnalysis,
        technicalEvaluation,
        creativePotential: creativeAnalysis
      };
      
    } catch (error) {
      console.error('❌ MODURUST tool analysis failed:', error);
      throw error;
    }
  }

  /**
   * Process NUWE creative session with AI
   */
  async processNUWESession(session) {
    try {
      console.log('🎨 Processing NUWE creative session with AI...');
      
      // Analyze creative process
      const creativeAnalysis = await this.analyzeCreativeProcess(session);
      
      // Analyze emotional journey
      const emotionalJourney = await this.analyzeEmotionalJourney(session);
      
      // Evaluate technical performance
      const technicalPerformance = this.evaluateTechnicalPerformance(session);
      
      // Calculate innovation score
      const innovationScore = await this.calculateInnovationScore(session);
      
      // Assess artistic merit
      const artisticMerit = await this.assessArtisticMerit(session);
      
      // Evaluate collaboration quality (if multiple tools used)
      const collaborationQuality = this.evaluateCollaborationQuality(session);
      
      // Store creative vectors in LanceDB
      const creativeVectors = this.generateCreativeVectors(session);
      await this.aiBridge.storeEmotionVectors(creativeVectors);
      
      // Generate creative hash
      const creativeHash = this.generateCreativeHash(session);
      
      // Calculate overall quality score
      const qualityScore = this.calculateCreativeQualityScore({
        creativeAnalysis,
        emotionalJourney,
        technicalPerformance,
        innovationScore,
        artisticMerit,
        collaborationQuality
      });
      
      // Evaluate DAO eligibility
      const daoEligibility = this.evaluateDAOEligibility({
        qualityScore,
        innovationScore,
        artisticMerit,
        collaborationQuality,
        session
      });
      
      const aiAnalysis = {
        creativeProcess: creativeAnalysis,
        emotionalJourney,
        technicalPerformance,
        innovationScore,
        artisticMerit,
        collaborationQuality,
        qualityScore,
        daoEligibility,
        analysisTimestamp: Date.now(),
        aiModel: 'IronLearn_LanceDB_Mintbase_NUWE_MODURUST'
      };
      
      return {
        sessionId: session.sessionId,
        aiAnalysis,
        creativeHash,
        qualityScore,
        daoEligibility
      };
      
    } catch (error) {
      console.error('❌ NUWE session processing failed:', error);
      throw error;
    }
  }

  /**
   * Create AI-powered MODURUST NFT
   */
  async createAIModurustNFT(session, generatedArt) {
    try {
      if (!this.contract) throw new Error('Contract not initialized');
      if (!this.wallet) throw new Error('Wallet not connected');

      console.log('🎨 Creating AI-powered MODURUST NFT...');

      // Process NUWE session with AI
      const processedSession = await this.processNUWESession(session);

      // Store artwork on Filecoin/IPFS
      let metadataCid = '';
      let metadataUrl = '';
      
      if (this.storageClient) {
        console.log('📤 Storing MODURUST creative work on Filecoin...');
        const storageResult = await this.storageClient.storeEmotionalArt({
          canvas: generatedArt.media,
          emotionData: session.emotionalData.emotions[session.emotionalData.emotions.length - 1],
          biometricHash: processedSession.creativeHash,
          aiModel: 'mintbase-modurust-ai-integration',
          generationParams: {
            title: generatedArt.title,
            description: generatedArt.description,
            tools: session.creativeData.tools.map(tool => tool.name),
            complexity: processedSession.aiAnalysis.creativeProcess.complexity,
            emotionBased: true,
            daoEligible: processedSession.daoEligibility.eligible
          }
        });
        
        metadataCid = storageResult.cid;
        metadataUrl = storageResult.url;
        console.log('✅ MODURUST creative work stored on Filecoin:', metadataCid);
      }

      // Create NFT metadata
      const tokenId = `modurust-ai-${session.sessionId}-${Date.now()}`;
      
      const nftData = {
        token_id: tokenId,
        receiver_id: this.wallet.getAccountId(),
        modurust_data: {
          session_id: session.sessionId,
          tools: session.creativeData.tools.map(tool => ({
            toolId: tool.toolId,
            name: tool.name,
            type: tool.type,
            complexity: tool.aiAnalysis?.complexity || 0.5
          })),
          complexity: processedSession.aiAnalysis.creativeProcess.complexity,
          hash: processedSession.creativeHash,
          tool_count: session.creativeData.tools.length
        },
        nuwe_data: {
          creative_process: processedSession.aiAnalysis.creativeProcess,
          emotional_journey: processedSession.aiAnalysis.emotionalJourney,
          performance_metrics: processedSession.aiAnalysis.technicalPerformance,
          innovation_score: processedSession.aiAnalysis.innovationScore,
          artistic_merit: processedSession.aiAnalysis.artisticMerit
        },
        emotion_data: {
          valence: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].valence,
          arousal: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].arousal,
          dominance: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].dominance,
          confidence: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].confidence || 0.8,
          timestamp: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].timestamp
        },
        creative_hash: processedSession.creativeHash,
        quality_score: processedSession.qualityScore,
        ai_analysis: processedSession.aiAnalysis,
        metadata_uri: metadataUrl,
        dao_eligibility: processedSession.daoEligibility,
        attributes: [
          ...(generatedArt.attributes || []),
          {
            trait_type: 'Tool Count',
            value: session.creativeData.tools.length
          },
          {
            trait_type: 'Innovation Score',
            value: Math.round(processedSession.aiAnalysis.innovationScore * 100)
          },
          {
            trait_type: 'Artistic Merit',
            value: Math.round(processedSession.aiAnalysis.artisticMerit * 100)
          },
          {
            trait_type: 'Quality Score',
            value: Math.round(processedSession.qualityScore * 100)
          },
          {
            trait_type: 'DAO Eligible',
            value: processedSession.daoEligibility.eligible ? 'Yes' : 'No'
          },
          {
            trait_type: 'Governance Weight',
            value: Math.round(processedSession.daoEligibility.governanceWeight * 100)
          },
          {
            trait_type: 'Storage Type',
            value: 'Filecoin'
          }
        ]
      };

      // Mint the NFT
      console.log('⛏️ Minting AI MODURUST NFT...');
      const transactionHash = await this.contract.mint_modurust_ai(nftData);

      console.log('✅ AI-powered MODURUST NFT created successfully!');

      return {
        tokenId,
        transactionHash,
        metadataCid,
        metadataUrl,
        aiAnalysis: processedSession.aiAnalysis,
        daoEligibility: processedSession.daoEligibility
      };

    } catch (error) {
      console.error('❌ AI MODURUST NFT creation failed:', error);
      throw error;
    }
  }

  /**
   * Extract tool features for AI analysis
   */
  extractToolFeatures(tool) {
    const features = [];
    
    // Tool type encoding
    features.push(this.encodeToolType(tool.type));
    
    // Parameter complexity
    features.push(tool.parameters.length / 20); // Normalize to 0-1
    
    // Input/output complexity
    features.push(tool.inputs.length / 10);
    features.push(tool.outputs.length / 10);
    
    // Dependency complexity
    features.push(tool.dependencies.length / 10);
    
    // Code complexity (basic metrics)
    const codeMetrics = this.analyzeCodeMetrics(tool.code);
    features.push(codeMetrics.lines / 1000);
    features.push(codeMetrics.functions / 50);
    features.push(codeMetrics.complexity / 10);
    
    // Version recency (simplified)
    const versionComponents = tool.version.split('.').map(Number);
    features.push((versionComponents[0] || 0) / 10);
    features.push((versionComponents[1] || 0) / 100);
    
    return features;
  }

  /**
   * Analyze creative process
   */
  async analyzeCreativeProcess(session) {
    try {
      // Analyze tool usage patterns
      const toolUsage = this.analyzeToolUsage(session.creativeData.tools);
      
      // Analyze parameter evolution
      const parameterEvolution = this.analyzeParameterEvolution(session.creativeData.timeline);
      
      // Analyze creative flow
      const creativeFlow = this.analyzeCreativeFlow(session.creativeData.connections);
      
      // Calculate complexity metrics
      const complexity = this.calculateCreativeComplexity(session);
      
      return {
        toolUsage,
        parameterEvolution,
        creativeFlow,
        complexity,
        processEfficiency: this.calculateProcessEfficiency(session),
        creativityPatterns: this.identifyCreativityPatterns(session)
      };
    } catch (error) {
      console.error('Creative process analysis failed:', error);
      return { error: 'Analysis failed' };
    }
  }

  /**
   * Analyze emotional journey
   */
  async analyzeEmotionalJourney(session) {
    if (!session.emotionalData.emotions || session.emotionalData.emotions.length === 0) {
      return { error: 'No emotional data available' };
    }
    
    const emotions = session.emotionalData.emotions;
    
    // Calculate emotional statistics
    const valenceStats = this.calculateEmotionStatistics(emotions.map(e => e.valence));
    const arousalStats = this.calculateEmotionStatistics(emotions.map(e => e.arousal));
    const dominanceStats = this.calculateEmotionStatistics(emotions.map(e => e.dominance));
    
    // Identify emotional patterns
    const emotionalPatterns = this.identifyEmotionalPatterns(emotions);
    
    // Correlate with creative process
    const creativeEmotionalCorrelation = this.correlateEmotionalCreative(
      emotions,
      session.creativeData.timeline
    );
    
    return {
      valenceStats,
      arousalStats,
      dominanceStats,
      emotionalPatterns,
      creativeEmotionalCorrelation,
      emotionalJourney: this.describeEmotionalJourney(emotions)
    };
  }

  /**
   * Calculate innovation score
   */
  async calculateInnovationScore(session) {
    try {
      // Analyze tool combinations
      const toolCombinations = this.analyzeToolCombinations(session.creativeData.tools);
      
      // Analyze parameter usage uniqueness
      const parameterUniqueness = this.analyzeParameterUniqueness(session.creativeData.parameters);
      
      // Analyze creative outputs
      const outputInnovation = await this.analyzeOutputInnovation(session.creativeData.outputs);
      
      // Combine scores
      const toolInnovation = Math.min(toolCombinations.uniqueCombinations / 10, 1.0);
      const parameterInnovation = Math.min(parameterUniqueness.uniquenessScore, 1.0);
      const outputInnovationScore = Math.min(outputInnovation.innovationScore, 1.0);
      
      return (toolInnovation + parameterInnovation + outputInnovationScore) / 3;
    } catch (error) {
      console.error('Innovation score calculation failed:', error);
      return 0.5; // Default fallback
    }
  }

  /**
   * Assess artistic merit
   */
  async assessArtisticMerit(session) {
    try {
      // Analyze emotional depth
      const emotionalDepth = this.calculateEmotionalDepth(session.emotionalData.emotions);
      
      // Analyze technical execution
      const technicalExecution = this.assessTechnicalExecution(session);
      
      // Analyze creative coherence
      const creativeCoherence = this.assessCreativeCoherence(session);
      
      // Combine scores
      return (emotionalDepth + technicalExecution + creativeCoherence) / 3;
    } catch (error) {
      console.error('Artistic merit assessment failed:', error);
      return 0.5; // Default fallback
    }
  }

  /**
   * Utility functions
   */
  encodeToolType(type) {
    const encoding = {
      'ShaderModule': 1.0,
      'AudioProcessor': 2.0,
      'VisualEffect': 3.0,
      'DataProcessor': 4.0,
      'CreativeTool': 5.0
    };
    return encoding[type] || 0;
  }

  analyzeCodeComplexity(code) {
    // Simple complexity analysis
    const lines = code.split('\n').length;
    const functions = (code.match(/function|fn|def/g) || []).length;
    const conditionals = (code.match(/if|while|for/g) || []).length;
    
    return (lines / 100) + (functions / 10) + (conditionals / 20);
  }

  analyzeCodeMetrics(code) {
    return {
      lines: code.split('\n').length,
      functions: (code.match(/function|fn|def/g) || []).length,
      complexity: this.analyzeCodeComplexity(code)
    };
  }

  calculateUsabilityScore(tool) {
    // Simple usability heuristic
    let score = 0.5;
    
    // Parameter clarity
    score += tool.parameters.every(p => p.description) ? 0.2 : 0;
    
    // Input/output clarity
    score += tool.inputs.every(i => i.description) ? 0.1 : 0;
    score += tool.outputs.every(o => o.description) ? 0.1 : 0;
    
    // Presets availability
    score += tool.presets.length > 0 ? 0.1 : 0;
    
    return Math.min(score, 1.0);
  }

  generateToolTags(tool, creativeAnalysis) {
    const tags = [];
    
    // Type-based tags
    tags.push(tool.type.toLowerCase());
    
    // Creative analysis tags
    if (creativeAnalysis.style) {
      tags.push(creativeAnalysis.style);
    }
    
    // Parameter-based tags
    tool.parameters.forEach(param => {
      if (param.name.includes('color') || param.name.includes('palette')) {
        tags.push('colorful');
      }
      if (param.name.includes('audio') || param.name.includes('sound')) {
        tags.push('audio-reactive');
      }
    });
    
    return [...new Set(tags)]; // Remove duplicates
  }

  async evaluateMarketPotential(tool, creativeAnalysis) {
    // Simplified market potential evaluation
    let score = 0.5;
    
    // Tool type factors
    const typeMultipliers = {
      'ShaderModule': 1.2,
      'AudioProcessor': 1.1,
      'VisualEffect': 1.3,
      'DataProcessor': 0.9,
      'CreativeTool': 1.0
    };
    
    score *= (typeMultipliers[tool.type] || 1.0);
    
    // Creative potential factor
    if (creativeAnalysis.creativityScore) {
      score *= (0.5 + creativeAnalysis.creativityScore * 0.5);
    }
    
    // Complexity vs usability balance
    const complexity = this.analyzeCodeComplexity(tool.code);
    const usability = this.calculateUsabilityScore(tool);
    const balance = Math.min(complexity, usability) / Math.max(complexity, usability);
    score *= (0.5 + balance * 0.5);
    
    return {
      score: Math.min(score, 1.0),
      factors: {
        toolType: tool.type,
        creativity: creativeAnalysis.creativityScore || 0.5,
        complexityUsabilityBalance: balance
      }
    };
  }

  generateTechnicalEvaluation(tool, complexityAnalysis) {
    return {
      complexityScore: complexityAnalysis.complexityScore || 0.5,
      performanceScore: this.estimatePerformanceScore(tool),
      codeQuality: this.assessCodeQuality(tool.code),
      maintainability: this.assessMaintainability(tool),
      scalability: this.assessScalability(tool)
    };
  }

  generateToolRecommendations(tool, technicalEvaluation) {
    const recommendations = [];
    
    if (technicalEvaluation.complexityScore > 0.8) {
      recommendations.push('Consider simplifying tool complexity for better performance');
    }
    
    if (technicalEvaluation.performanceScore < 0.5) {
      recommendations.push('Optimize code for better performance');
    }
    
    if (technicalEvaluation.maintainability < 0.6) {
      recommendations.push('Improve code organization and documentation');
    }
    
    if (tool.parameters.length === 0) {
      recommendations.push('Add configurable parameters for better usability');
    }
    
    return recommendations;
  }

  estimatePerformanceScore(tool) {
    // Simple performance estimation
    const codeComplexity = this.analyzeCodeComplexity(tool.code);
    const parameterOverhead = tool.parameters.length * 0.01;
    
    return Math.max(0.1, 1.0 - (codeComplexity * 0.3) - parameterOverhead);
  }

  assessCodeQuality(code) {
    // Simple code quality assessment
    let score = 0.5;
    
    // Check for comments
    if (code.includes('//') || code.includes('/*')) {
      score += 0.1;
    }
    
    // Check for proper function naming
    const functions = code.match(/function\s+(\w+)/g) || [];
    const wellNamedFunctions = functions.filter(f => 
      f.includes('function') && f.length > 10 // Simple heuristic
    ).length;
    score += (wellNamedFunctions / Math.max(functions.length, 1)) * 0.2;
    
    return Math.min(score, 1.0);
  }

  assessMaintainability(tool) {
    // Simple maintainability assessment
    let score = 0.5;
    
    // Parameter documentation
    score += tool.parameters.every(p => p.description) ? 0.2 : 0;
    
    // Code complexity vs functionality balance
    const complexity = this.analyzeCodeComplexity(tool.code);
    const functionality = tool.parameters.length + tool.inputs.length + tool.outputs.length;
    const balance = Math.min(functionality / (complexity + 1), 1.0);
    score += balance * 0.2;
    
    return Math.min(score, 1.0);
  }

  assessScalability(tool) {
    // Simple scalability assessment
    let score = 0.5;
    
    // Input/output flexibility
    score += tool.inputs.length > 0 ? 0.1 : 0;
    score += tool.outputs.length > 0 ? 0.1 : 0;
    
    // Parameter configurability
    score += Math.min(tool.parameters.length / 10, 0.3);
    
    return Math.min(score, 1.0);
  }

  analyzeToolUsage(tools) {
    const usage = {
      toolTypes: {},
      complexityDistribution: [],
      performanceDistribution: [],
      mostUsedTools: []
    };
    
    tools.forEach(tool => {
      // Count tool types
      usage.toolTypes[tool.type] = (usage.toolTypes[tool.type] || 0) + 1;
      
      // Analyze complexity
      const complexity = tool.aiAnalysis?.complexity || 0.5;
      usage.complexityDistribution.push(complexity);
      
      // Analyze performance
      const performance = tool.aiAnalysis?.performance || 0.5;
      usage.performanceDistribution.push(performance);
    });
    
    // Find most used tool types
    usage.mostUsedTools = Object.entries(usage.toolTypes)
      .sort(([,a], [,b]) => b - a)
      .slice(0, 3)
      .map(([type]) => type);
    
    return usage;
  }

  analyzeParameterEvolution(timeline) {
    if (timeline.length === 0) {
      return { evolution: 'No parameter changes recorded' };
    }
    
    const parameterChanges = {};
    
    timeline.forEach(entry => {
      if (!parameterChanges[entry.parameter]) {
        parameterChanges[entry.parameter] = [];
      }
      parameterChanges[entry.parameter].push({
        timestamp: entry.timestamp,
        value: entry.value,
        toolId: entry.toolId
      });
    });
    
    // Analyze evolution patterns
    const evolutionPatterns = Object.entries(parameterChanges).map(([param, changes]) => {
      const values = changes.map(c => c.value);
      const stats = this.calculateEmotionStatistics(values);
      
      return {
        parameter: param,
        changeCount: changes.length,
        evolutionPattern: this.identifyEvolutionPattern(values),
        statistics: stats
      };
    });
    
    return {
      parameterCount: Object.keys(parameterChanges).length,
      totalChanges: timeline.length,
      evolutionPatterns,
      mostActiveParameters: evolutionPatterns
        .sort((a, b) => b.changeCount - a.changeCount)
        .slice(0, 5)
        .map(p => p.parameter)
    };
  }

  analyzeCreativeFlow(connections) {
    const flow = {
      connectionCount: connections.length,
      toolConnections: {},
      parameterFlow: {},
      flowComplexity: 0
    };
    
    connections.forEach(connection => {
      // Count tool connections
      flow.toolConnections[connection.from] = (flow.toolConnections[connection.from] || 0) + 1;
      flow.toolConnections[connection.to] = (flow.toolConnections[connection.to] || 0) + 1;
      
      // Count parameter flows
      flow.parameterFlow[connection.parameter] = (flow.parameterFlow[connection.parameter] || 0) + 1;
    });
    
    // Calculate flow complexity
    const uniqueTools = new Set([...connections.map(c => c.from), ...connections.map(c => c.to)]);
    flow.flowComplexity = Math.min(connections.length / uniqueTools.size, 2.0);
    
    return flow;
  }

  calculateCreativeComplexity(session) {
    const factors = {
      toolCount: session.creativeData.tools.length,
      connectionCount: session.creativeData.connections.length,
      parameterCount: Object.keys(session.creativeData.parameters).length,
      timelineLength: session.creativeData.timeline.length
    };
    
    // Weighted complexity calculation
    const complexity = (
      factors.toolCount * 0.3 +
      factors.connectionCount * 0.2 +
      factors.parameterCount * 0.2 +
      factors.timelineLength * 0.1
    ) / 10; // Normalize
    
    return Math.min(complexity, 2.0);
  }

  calculateProcessEfficiency(session) {
    if (!session.performanceMetrics) return 0.5;
    
    const metrics = session.performanceMetrics.overall;
    
    // Efficiency based on performance metrics
    const fpsScore = Math.min(metrics.averageFps / 60, 1.0); // 60 FPS baseline
    const memoryScore = Math.max(0, 1 - (metrics.peakMemoryUsage / 1000)); // 1GB baseline
    const timeScore = Math.max(0, 1 - (metrics.totalExecutionTime / 1800)); // 30 min baseline
    
    return (fpsScore + memoryScore + timeScore) / 3;
  }

  identifyCreativityPatterns(session) {
    const patterns = {
      toolSwitching: this.analyzeToolSwitching(session.creativeData.timeline),
      parameterExploration: this.analyzeParameterExploration(session.creativeData.timeline),
      creativeBursts: this.identifyCreativeBursts(session.creativeData.timeline),
      emotionalCreativity: this.analyzeEmotionalCreativity(session)
    };
    
    return patterns;
  }

  calculateEmotionStatistics(emotions) {
    if (emotions.length === 0) return { average: 0, variance: 0, trend: 'none' };
    
    const average = emotions.reduce((sum, val) => sum + val, 0) / emotions.length;
    const variance = emotions.reduce((sum, val) => sum + Math.pow(val - average, 2), 0) / emotions.length;
    
    // Simple trend analysis
    const firstHalf = emotions.slice(0, Math.floor(emotions.length / 2));
    const secondHalf = emotions.slice(Math.floor(emotions.length / 2));
    const firstAvg = firstHalf.reduce((sum, val) => sum + val, 0) / firstHalf.length;
    const secondAvg = secondHalf.reduce((sum, val) => sum + val, 0) / secondHalf.length;
    
    let trend = 'stable';
    if (secondAvg > firstAvg + 0.1) trend = 'increasing';
    else if (secondAvg < firstAvg - 0.1) trend = 'decreasing';
    
    return { average, variance, trend };
  }

  identifyEmotionalPatterns(emotions) {
    const patterns = [];
    
    // Identify emotional arcs
    for (let i = 1; i < emotions.length; i++) {
      const current = emotions[i];
      const previous = emotions[i - 1];
      
      // Emotional transitions
      if (current.valence > previous.valence + 0.2) {
        patterns.push({
          type: 'positive_transition',
          timestamp: current.timestamp,
          magnitude: current.valence - previous.valence
        });
      } else if (current.valence < previous.valence - 0.2) {
        patterns.push({
          type: 'negative_transition',
          timestamp: current.timestamp,
          magnitude: previous.valence - current.valence
        });
      }
      
      // Arousal spikes
      if (current.arousal > previous.arousal + 0.3) {
        patterns.push({
          type: 'arousal_spike',
          timestamp: current.timestamp,
          magnitude: current.arousal - previous.arousal
        });
      }
    }
    
    return patterns;
  }

  correlateEmotionalCreative(emotions, timeline) {
    if (emotions.length === 0 || timeline.length === 0) {
      return { correlation: 0, analysis: 'Insufficient data' };
    }
    
    // Simple correlation between emotional intensity and creative activity
    const emotionIntensities = emotions.map(e => 
      Math.sqrt(e.valence * e.valence + e.arousal * e.arousal + e.dominance * e.dominance)
    );
    
    // Create timeline bins for correlation
    const timelineActivity = this.binTimelineActivity(timeline, emotions.length);
    
    // Calculate correlation
    const correlation = this.calculateCorrelation(emotionIntensities, timelineActivity);
    
    return {
      correlation,
      analysis: this.generateEmotionalCreativeAnalysis(correlation),
      intensityDistribution: emotionIntensities
    };
  }

  describeEmotionalJourney(emotions) {
    if (emotions.length === 0) return 'No emotional journey recorded';
    
    const valenceStats = this.calculateEmotionStatistics(emotions.map(e => e.valence));
    const arousalStats = this.calculateEmotionStatistics(emotions.map(e => e.arousal));
    
    let description = '';
    
    // Valence description
    if (valenceStats.average > 0.6) {
      description += 'Predominantly positive emotional experience. ';
    } else if (valenceStats.average < 0.4) {
      description += 'Challenging emotional experience. ';
    } else {
      description += 'Balanced emotional experience. ';
    }
    
    // Arousal description
    if (arousalStats.average > 0.7) {
      description += 'High energy and engagement throughout. ';
    } else if (arousalStats.average < 0.3) {
      description += 'Calm and contemplative creative process. ';
    } else {
      description += 'Varied energy levels during creation. ';
    }
    
    // Trend description
    if (valenceStats.trend === 'increasing') {
      description += 'Emotional state improved over time. ';
    } else if (valenceStats.trend === 'decreasing') {
      description += 'Emotional challenges emerged during creation. ';
    }
    
    return description.trim();
  }

  async analyzeOutputInnovation(outputs) {
    // Simplified innovation analysis based on outputs
    let innovationScore = 0.5;
    
    if (outputs.video) innovationScore += 0.2;
    if (outputs.audio) innovationScore += 0.2;
    if (outputs.images && outputs.images.length > 1) innovationScore += 0.1;
    if (outputs.metadata) innovationScore += 0.1;
    
    return {
      innovationScore: Math.min(innovationScore, 1.0),
      outputTypes: Object.keys(outputs).filter(key => outputs[key] !== undefined),
      multimediaScore: this.calculateMultimediaScore(outputs)
    };
  }

  calculateMultimediaScore(outputs) {
    let score = 0;
    if (outputs.video) score += 0.3;
    if (outputs.audio) score += 0.3;
    if (outputs.images) score += 0.2;
    if (outputs.metadata) score += 0.2;
    return Math.min(score, 1.0);
  }

  calculateEmotionalDepth(emotions) {
    if (!emotions || emotions.length === 0) return 0.5;
    
    const valenceRange = Math.max(...emotions.map(e => e.valence)) - Math.min(...emotions.map(e => e.valence));
    const arousalRange = Math.max(...emotions.map(e => e.arousal)) - Math.min(...emotions.map(e => e.arousal));
    
    return (valenceRange + arousalRange) / 2;
  }

  assessTechnicalExecution(session) {
    if (!session.performanceMetrics) return 0.5;
    
    const metrics = session.performanceMetrics.overall;
    
    // Technical execution based on performance
    const executionScore = (
      Math.min(metrics.averageFps / 60, 1.0) * 0.4 + // 60 FPS baseline
      Math.max(0, 1 - (metrics.peakMemoryUsage / 1000)) * 0.3 + // Memory efficiency
      Math.max(0, 1 - (metrics.totalExecutionTime / 3600)) * 0.3 // Time efficiency (30 min baseline)
    );
    
    return executionScore;
  }

  assessCreativeCoherence(session) {
    // Assess how well the creative elements work together
    const toolCoherence = this.calculateToolCoherence(session.creativeData.tools);
    const parameterCoherence = this.calculateParameterCoherence(session.creativeData.parameters);
    const emotionalCoherence = this.calculateEmotionalCoherence(session.emotionalData.emotions);
    
    return (toolCoherence + parameterCoherence + emotionalCoherence) / 3;
  }

  calculateToolCoherence(tools) {
    // Simple coherence based on tool type diversity and compatibility
    const types = tools.map(tool => tool.type);
    const uniqueTypes = new Set(types);
    
    // More diverse tool types can indicate higher coherence if used well
    const diversityScore = Math.min(uniqueTypes.size / 5, 1.0); // 5 is max tool types
    
    return diversityScore;
  }

  calculateParameterCoherence(parameters) {
    const paramCount = Object.keys(parameters).length;
    if (paramCount === 0) return 0.5;
    
    // Simple coherence based on parameter count and organization
    return Math.min(paramCount / 20, 1.0); // 20 parameters as reasonable max
  }

  calculateEmotionalCoherence(emotions) {
    if (!emotions || emotions.length < 2) return 0.5;
    
    // Calculate emotional consistency
    const valenceConsistency = 1 - this.calculateEmotionStatistics(emotions.map(e => e.valence)).variance;
    const arousalConsistency = 1 - this.calculateEmotionStatistics(emotions.map(e => e.arousal)).variance;
    
    return (valenceConsistency + arousalConsistency) / 2;
  }

  createToolUsageVector(tools) {
    const vector = new Array(5).fill(0); // 5 tool types
    
    tools.forEach(tool => {
      const typeIndex = this.getToolTypeIndex(tool.type);
      if (typeIndex >= 0) {
        vector[typeIndex] += 1 / tools.length; // Normalize
      }
    });
    
    return vector;
  }

  getToolTypeIndex(type) {
    const types = ['ShaderModule', 'AudioProcessor', 'VisualEffect', 'DataProcessor', 'CreativeTool'];
    return types.indexOf(type);
  }

  generateCreativeVectors(session) {
    const vectors = [];
    
    // Create vector from latest emotional state
    if (session.emotionalData.emotions.length > 0) {
      const latestEmotion = session.emotionalData.emotions[session.emotionalData.emotions.length - 1];
      
      // Tool usage vector
      const toolUsageVector = this.createToolUsageVector(session.creativeData.tools);
      
      vectors.push({
        vector: [
          latestEmotion.valence,
          latestEmotion.arousal,
          latestEmotion.dominance,
          latestEmotion.confidence || 0.8,
          ...toolUsageVector,
          session.creativeData.tools.length / 10, // Tool count normalized
          Object.keys(session.creativeData.parameters).length / 20 // Parameter count normalized
        ],
        metadata: {
          sessionId: session.sessionId,
          timestamp: latestEmotion.timestamp,
          userId: session.userId,
          type: 'mintbase_creative',
          toolCount: session.creativeData.tools.length,
          complexity: this.calculateCreativeComplexity(session)
        }
      });
    }
    
    return vectors;
  }

  generateCreativeHash(session) {
    const dataString = JSON.stringify({
      tools: session.creativeData.tools.map(t => t.toolId),
      connections: session.creativeData.connections,
      parameters: session.creativeData.parameters,
      timeline: session.creativeData.timeline.slice(-10),
      emotions: session.emotionalData.emotions?.slice(-10),
      performance: session.performanceMetrics,
      timestamp: Date.now()
    });
    
    return createHash('sha256').update(dataString).digest('hex');
  }

  calculateCreativeQualityScore(analysis) {
    try {
      let qualityScore = 0.4; // Base score
      
      // Creative process quality
      if (analysis.creativeAnalysis?.complexity) {
        qualityScore += Math.min(analysis.creativeAnalysis.complexity / 2, 0.2);
      }
      
      // Innovation factor
      qualityScore += analysis.innovationScore * 0.2;
      
      // Artistic merit factor
      qualityScore += analysis.artisticMerit * 0.2;
      
      // Technical performance factor
      if (analysis.technicalPerformance) {
        qualityScore += analysis.technicalPerformance * 0.1;
      }
      
      return Math.min(qualityScore, 1.0);
    } catch (error) {
      console.error('Error calculating creative quality score:', error);
      return 0.5; // Default fallback
    }
  }

  evaluateDAOEligibility(evaluation) {
    const { qualityScore, innovationScore, artisticMerit, collaborationQuality, session } = evaluation;
    
    // Basic eligibility criteria
    const minQualityScore = 0.6;
    const minInnovationScore = 0.5;
    const minArtisticMerit = 0.5;
    
    const eligible = qualityScore >= minQualityScore && 
                    innovationScore >= minInnovationScore && 
                    artisticMerit >= minArtisticMerit;
    
    // Calculate governance weight based on various factors
    let governanceWeight = 0.1; // Base weight
    
    if (eligible) {
      governanceWeight += qualityScore * 0.3;
      governanceWeight += innovationScore * 0.2;
      governanceWeight += artisticMerit * 0.2;
      governanceWeight += Math.min(collaborationQuality * 0.1, 0.1);
      
      // Bonus for tool diversity
      const toolDiversity = session.creativeData.tools.length / 10;
      governanceWeight += Math.min(toolDiversity * 0.1, 0.1);
    }
    
    // Calculate community score
    const communityScore = (qualityScore + innovationScore + artisticMerit) / 3;
    
    return {
      eligible,
      governanceWeight: Math.min(governanceWeight, 1.0),
      communityScore
    };
  }

  calculateAverage(data) {
    if (!data || data.length === 0) return 0;
    return data.reduce((sum, val) => sum + val, 0) / data.length;
  }

  calculateCorrelation(x, y) {
    if (x.length !== y.length || x.length === 0) return 0;
    
    const n = x.length;
    const sumX = x.reduce((sum, val) => sum + val, 0);
    const sumY = y.reduce((sum, val) => sum + val, 0);
    const sumXY = x.reduce((sum, val, i) => sum + val * y[i], 0);
    const sumX2 = x.reduce((sum, val) => sum + val * val, 0);
    const sumY2 = y.reduce((sum, val) => sum + val * val, 0);
    
    const numerator = n * sumXY - sumX * sumY;
    const denominator = Math.sqrt((n * sumX2 - sumX * sumX) * (n * sumY2 - sumY * sumY));
    
    return denominator === 0 ? 0 : numerator / denominator;
  }

  generateEmotionalCreativeAnalysis(correlation) {
    if (correlation > 0.7) {
      return 'Strong correlation between emotional state and creative activity - highly emotionally driven creation';
    } else if (correlation > 0.4) {
      return 'Moderate correlation between emotion and creativity - some emotional influence on creative process';
    } else if (correlation > -0.4) {
      return 'Weak correlation - creative activity relatively independent of emotional state';
    } else {
      return 'Negative correlation - creative activity inversely related to emotional state';
    }
  }

  identifyEvolutionPattern(values) {
    if (values.length < 2) return 'stable';
    
    const firstAvg = values.slice(0, Math.floor(values.length / 2)).reduce((sum, val) => sum + val, 0) / Math.floor(values.length / 2);
    const secondAvg = values.slice(Math.floor(values.length / 2)).reduce((sum, val) => sum + val, 0) / Math.ceil(values.length / 2);
    
    if (secondAvg > firstAvg + 0.1) return 'increasing';
    if (secondAvg < firstAvg - 0.1) return 'decreasing';
    return 'stable';
  }

  binTimelineActivity(timeline, binCount) {
    if (timeline.length === 0) return new Array(binCount).fill(0);
    
    const bins = new Array(binCount).fill(0);
    const timeRange = timeline[timeline.length - 1].timestamp - timeline[0].timestamp;
    const binSize = timeRange / binCount;
    
    timeline.forEach(entry => {
      const binIndex = Math.min(Math.floor((entry.timestamp - timeline[0].timestamp) / binSize), binCount - 1);
      bins[binIndex]++;
    });
    
    return bins.map(count => count / timeline.length); // Normalize
  }

  analyzeToolSwitching(timeline) {
    const toolSwitches = [];
    let currentTool = null;
    
    timeline.forEach(entry => {
      if (entry.toolId !== currentTool) {
        toolSwitches.push({
          timestamp: entry.timestamp,
          fromTool: currentTool,
          toTool: entry.toolId
        });
        currentTool = entry.toolId;
      }
    });
    
    return {
      switchCount: toolSwitches.length,
      switchingRate: toolSwitches.length / (timeline.length || 1),
      switches: toolSwitches
    };
  }

  analyzeParameterExploration(timeline) {
    const parameterValues = {};
    
    timeline.forEach(entry => {
      if (!parameterValues[entry.parameter]) {
        parameterValues[entry.parameter] = new Set();
      }
      parameterValues[entry.parameter].add(entry.value);
    });
    
    const exploration = Object.entries(parameterValues).map(([param, values]) => ({
      parameter: param,
      uniqueValues: values.size,
      explorationScore: Math.min(values.size / 10, 1.0)
    }));
    
    return {
      totalParameters: Object.keys(parameterValues).length,
      explorationPatterns: exploration,
      averageExploration: exploration.reduce((sum, p) => sum + p.explorationScore, 0) / exploration.length
    };
  }

  identifyCreativeBursts(timeline) {
    const bursts = [];
    const windowSize = 5;
    const activityThreshold = 3; // Minimum activities in window for a burst
    
    for (let i = 0; i <= timeline.length - windowSize; i++) {
      const window = timeline.slice(i, i + windowSize);
      const activityCount = window.length;
      
      if (activityCount >= activityThreshold) {
        bursts.push({
          startIndex: i,
          endIndex: i + windowSize - 1,
          activityCount,
          intensity: activityCount / windowSize
        });
      }
    }
    
    return {
      burstCount: bursts.length,
      totalBurstActivity: bursts.reduce((sum, burst) => sum + burst.activityCount, 0),
      bursts: bursts.slice(0, 5) // Top 5 bursts
    };
  }

  analyzeEmotionalCreativity(session) {
    if (!session.emotionalData.emotions || session.emotionalData.emotions.length === 0) {
      return { correlation: 0, analysis: 'No emotional data' };
    }
    
    const correlation = this.correlateEmotionalCreative(
      session.emotionalData.emotions,
      session.creativeData.timeline
    );
    
    return {
      emotionalDepth: this.calculateEmotionalDepth(session.emotionalData.emotions),
      creativityCorrelation: correlation,
      emotionalIntensity: this.calculateEmotionalIntensity(session.emotionalData.emotions)
    };
  }

  calculateEmotionalIntensity(emotions) {
    if (!emotions || emotions.length === 0) return 0;
    
    return emotions.map(e => 
      Math.sqrt(e.valence * e.valence + e.arousal * e.arousal + e.dominance * e.dominance)
    ).reduce((sum, intensity) => sum + intensity, 0) / emotions.length;
  }

  analyzeToolCombinations(tools) {
    const combinations = new Set();
    
    // Analyze all possible 2-tool combinations
    for (let i = 0; i < tools.length; i++) {
      for (let j = i + 1; j < tools.length; j++) {
        const combination = [tools[i].type, tools[j].type].sort().join('-');
        combinations.add(combination);
      }
    }
    
    return {
      uniqueCombinations: combinations.size,
      combinations: Array.from(combinations),
      diversityScore: Math.min(combinations.size / 10, 1.0)
    };
  }

  analyzeParameterUniqueness(parameters) {
    const paramKeys = Object.keys(parameters);
    const uniqueParams = new Set(paramKeys);
    
    return {
      uniquenessScore: Math.min(uniqueParams.size / 15, 1.0), // 15 unique params as max
      parameterCount: uniqueParams.size,
      parameters: Array.from(uniqueParams)
    };
  }

  evaluateTechnicalPerformance(session) {
    if (!session.performanceMetrics) return 0.5;
    
    const toolPerformance = Object.values(session.performanceMetrics.tools);
    
    const avgExecutionTime = toolPerformance.reduce((sum, tool) => 
      sum + this.calculateAverage(tool.executionTime), 0) / toolPerformance.length;
    
    const avgMemoryUsage = toolPerformance.reduce((sum, tool) => 
      sum + this.calculateAverage(tool.memoryUsage), 0) / toolPerformance.length;
    
    const avgErrorRate = toolPerformance.reduce((sum, tool) => 
      sum + tool.errorRate, 0) / toolPerformance.length;
    
    return {
      executionEfficiency: Math.max(0, 1 - (avgExecutionTime / 1000)), // 1 second baseline
      memoryEfficiency: Math.max(0, 1 - (avgMemoryUsage / 500)), // 500MB baseline
      reliability: Math.max(0, 1 - avgErrorRate),
      overall: (Math.max(0, 1 - (avgExecutionTime / 1000)) + Math.max(0, 1 - (avgMemoryUsage / 500)) + Math.max(0, 1 - avgErrorRate)) / 3
    };
  }

  evaluateCollaborationQuality(session) {
    const toolCount = session.creativeData.tools.length;
    const connectionCount = session.creativeData.connections.length;
    
    // Collaboration quality based on tool interaction
    const interactionDensity = connectionCount / Math.max(toolCount, 1);
    const toolDiversity = new Set(session.creativeData.tools.map(t => t.type)).size;
    
    return {
      interactionDensity: Math.min(interactionDensity, 2.0),
      toolDiversity: Math.min(toolDiversity / 5, 1.0), // 5 tool types max
      collaborationScore: (Math.min(interactionDensity, 1.0) + Math.min(toolDiversity / 5, 1.0)) / 2
    };
  }

  getConnectionStatus() {
    return this.isConnected;
  }

  getWallet() {
    return this.wallet;
  }

  getAccountId() {
    return this.wallet?.getAccountId() || null;
  }
}

// Factory function for easy instantiation
export function createMintbaseAIIntegration(config) {
  return new MintbaseAIIntegration(config);
}
import { connect, keyStores, WalletConnection, Contract } from 'near-api-js';
import { WASMMLBridge } from './unified-ai-ml-integration';
import { FilecoinStorageClient } from './filecoin-storage';
import { createHash } from 'crypto';

export interface NEARFractalAIConfig {
  networkId: 'mainnet' | 'testnet';
  contractId: string;
  web3StorageApiKey?: string;
  fractalStudioContract?: string;
  wgslStudioContract?: string;
}

export interface FractalSession {
  sessionId: string;
  userId: string;
  timestamp: number;
  fractalData: {
    type: 'mandelbrot' | 'julia' | 'burning_ship' | 'newton' | 'phoenix' | 'custom';
    parameters: {
      zoom: number;
      centerX: number;
      centerY: number;
      maxIterations: number;
      juliaC?: { real: number; imag: number };
      colorPalette?: string[];
    };
    shaderCode?: string;
    wgslCode?: string;
    keyframes?: Array<{
      timestamp: number;
      parameters: any;
      emotionalState?: EmotionalVector;
    }>;
  };
  emotionalData: {
    emotions: Array<{
      timestamp: number;
      valence: number;
      arousal: number;
      dominance: number;
      confidence?: number;
    }>;
    biometricSignals?: {
      eeg?: number[];
      heartRate?: number[];
      attention?: number;
      meditation?: number;
    };
  };
  performanceMetrics?: {
    fps: number[];
    renderTime: number[];
    gpuMemory: number[];
    complexity: number;
  };
}

export interface EmotionalVector {
  valence: number;
  arousal: number;
  dominance: number;
}

export interface NEARFractalNFT {
  tokenId: string;
  title: string;
  description: string;
  media: Blob;
  attributes: Array<{
    trait_type: string;
    value: string | number;
  }>;
  extra: {
    fractal: {
      sessionId: string;
      hash: string;
      type: string;
      parameters: any;
      complexity: number;
      aiAnalysis: any;
    };
    ai: {
      model: string;
      analysis: any;
      confidence: number;
      patternRecognition: any;
      styleClassification: any;
    };
    storage: {
      type: 'filecoin';
      cid: string;
      url: string;
    };
    wgsl?: {
      vertexShader: string;
      fragmentShader: string;
      computeShader?: string;
    };
  };
}

export class NEARFractalAIIntegration {
  private wallet: WalletConnection | null = null;
  private contract: Contract | null = null;
  private fractalContract: Contract | null = null;
  private wgslContract: Contract | null = null;
  private connection: any = null;
  private aiBridge: WASMMLBridge;
  private storageClient: FilecoinStorageClient | null = null;
  private config: NEARFractalAIConfig;
  private isConnected: boolean = false;

  constructor(config: NEARFractalAIConfig) {
    this.config = config;
    this.aiBridge = new WASMMLBridge();
    
    if (config.web3StorageApiKey) {
      this.storageClient = new FilecoinStorageClient(config.web3StorageApiKey);
    }
  }

  /**
   * Initialize NEAR connection and wallet
   */
  async initialize(): Promise<void> {
    try {
      console.log('🔌 Initializing NEAR Fractal AI integration...');
      
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
      this.wallet = new WalletConnection(this.connection, 'near-fractal-ai-integration');
      
      if (this.wallet.isSignedIn()) {
        await this.initializeContracts();
        this.isConnected = true;
        console.log('✅ NEAR Fractal AI integration initialized successfully');
      } else {
        console.log('⚠️ Wallet not signed in');
      }
      
    } catch (error) {
      console.error('❌ NEAR initialization failed:', error);
      throw error;
    }
  }

  /**
   * Initialize contract connections
   */
  private async initializeContracts(): Promise<void> {
    if (!this.wallet) throw new Error('Wallet not initialized');

    // Main NFT contract
    this.contract = new Contract(
      this.wallet.account(),
      this.config.contractId,
      {
        viewMethods: ['nft_token', 'nft_tokens_for_owner', 'get_fractal_analysis', 'get_wgsl_shader'],
        changeMethods: ['mint_fractal_ai', 'update_fractal_data', 'store_wgsl_shader', 'store_fractal_analysis'],
      }
    );

    // Fractal Studio contract
    if (this.config.fractalStudioContract) {
      this.fractalContract = new Contract(
        this.wallet.account(),
        this.config.fractalStudioContract,
        {
          viewMethods: ['get_fractal_session', 'get_fractal_params', 'get_performance_metrics'],
          changeMethods: ['create_fractal_session', 'update_fractal_params', 'record_performance'],
        }
      );
    }

    // WGSL Studio contract
    if (this.config.wgslStudioContract) {
      this.wgslContract = new Contract(
        this.wallet.account(),
        this.config.wgslStudioContract,
        {
          viewMethods: ['get_shader', 'get_shader_params', 'get_shader_performance'],
          changeMethods: ['create_shader', 'update_shader', 'record_shader_performance'],
        }
      );
    }
  }

  /**
   * Connect wallet
   */
  async connectWallet(): Promise<void> {
    if (!this.wallet) throw new Error('Wallet not initialized');

    await this.wallet.requestSignIn({
      contractId: this.config.contractId,
      methodNames: ['mint_fractal_ai', 'update_fractal_data', 'store_wgsl_shader', 'store_fractal_analysis'],
    });

    await this.initializeContracts();
    this.isConnected = true;
  }

  /**
   * Analyze fractal patterns with AI using Iron Learn
   */
  async analyzeFractalPatterns(
    fractalSession: FractalSession,
    imageData?: string
  ): Promise<{
    patterns: any;
    complexity: number;
    styleClassification: any;
    emotionalCorrelation: any;
    recommendations: any[];
  }> {
    try {
      console.log('🧠 Analyzing fractal patterns with AI...');
      
      // Extract fractal features for AI analysis
      const fractalFeatures = this.extractFractalFeatures(fractalSession);
      
      // Use Iron Learn for pattern recognition in fractal images
      const patternAnalysis = await this.aiBridge.processWithIronLearn(
        { 
          features: fractalFeatures,
          parameters: fractalSession.fractalData.parameters,
          emotionalData: fractalSession.emotionalData,
          performance: fractalSession.performanceMetrics
        },
        'pattern_recognition'
      );
      
      // Analyze fractal complexity
      const complexity = this.calculateFractalComplexity(fractalSession);
      
      // Classify fractal style using AI
      const styleClassification = await this.aiBridge.classifyCreativeStyle({
        fractalType: fractalSession.fractalData.type,
        parameters: fractalSession.fractalData.parameters,
        complexity,
        emotionalData: fractalSession.emotionalData
      });
      
      // Correlate with emotional data
      const emotionalCorrelation = this.analyzeEmotionalCorrelation(
        fractalSession.fractalData.parameters,
        fractalSession.emotionalData.emotions
      );
      
      // Generate AI recommendations
      const recommendations = await this.generateFractalRecommendations({
        currentPatterns: patternAnalysis,
        styleClassification,
        emotionalCorrelation,
        complexity
      });
      
      return {
        patterns: patternAnalysis,
        complexity,
        styleClassification,
        emotionalCorrelation,
        recommendations
      };
      
    } catch (error) {
      console.error('❌ Fractal pattern analysis failed:', error);
      throw error;
    }
  }

  /**
   * Process fractal session with comprehensive AI analysis
   */
  async processFractalSession(session: FractalSession): Promise<{
    sessionId: string;
    aiAnalysis: any;
    fractalHash: string;
    complexity: number;
    qualityScore: number;
    styleRecommendations: any[];
  }> {
    try {
      console.log('🎨 Processing fractal session with AI analysis...');
      
      // Analyze fractal patterns
      const patternAnalysis = await this.analyzeFractalPatterns(session);
      
      // Store fractal vectors in LanceDB for similarity search
      const fractalVectors = this.generateFractalVectors(session);
      await this.aiBridge.storeEmotionVectors(fractalVectors);
      
      // Generate style-based recommendations
      const styleRecommendations = await this.generateStyleRecommendations(session, patternAnalysis);
      
      // Generate fractal hash
      const fractalHash = this.generateFractalHash(session);
      
      // Calculate quality score based on AI analysis
      const qualityScore = this.calculateFractalQualityScore({
        patternAnalysis,
        complexity: patternAnalysis.complexity,
        emotionalCorrelation: patternAnalysis.emotionalCorrelation,
        performance: session.performanceMetrics
      });
      
      const aiAnalysis = {
        patterns: patternAnalysis.patterns,
        complexity: patternAnalysis.complexity,
        styleClassification: patternAnalysis.styleClassification,
        emotionalCorrelation: patternAnalysis.emotionalCorrelation,
        recommendations: patternAnalysis.recommendations,
        styleRecommendations,
        qualityScore,
        analysisTimestamp: Date.now(),
        aiModel: 'IronLearn_LanceDB_NEAR_Fractal_Studio'
      };
      
      // Store AI analysis on-chain if contract is available
      if (this.fractalContract) {
        try {
          await this.storeFractalAnalysis(session.sessionId, aiAnalysis);
        } catch (error) {
          console.warn('Failed to store fractal analysis on-chain:', error);
        }
      }
      
      return {
        sessionId: session.sessionId,
        aiAnalysis,
        fractalHash,
        complexity: patternAnalysis.complexity,
        qualityScore,
        styleRecommendations
      };
      
    } catch (error) {
      console.error('❌ Fractal session processing failed:', error);
      throw error;
    }
  }

  /**
   * Create AI-powered fractal NFT
   */
  async createAIFractalNFT(
    session: FractalSession,
    generatedArt: {
      title: string;
      description: string;
      media: Blob;
      attributes?: Array<{
        trait_type: string;
        value: string | number;
      }>;
    }
  ): Promise<{
    tokenId: string;
    transactionHash: string;
    metadataCid: string;
    metadataUrl: string;
    aiAnalysis: any;
    wgslCode?: string;
  }> {
    try {
      if (!this.contract) throw new Error('Contract not initialized');
      if (!this.wallet) throw new Error('Wallet not connected');

      console.log('🎨 Creating AI-powered fractal NFT...');

      // Process fractal session with AI
      const processedSession = await this.processFractalSession(session);

      // Generate WGSL shader code if not provided
      let wgslCode = session.fractalData.wgslCode;
      if (!wgslCode && this.wgslContract) {
        wgslCode = await this.generateWGSLShader(session);
      }

      // Store artwork on Filecoin/IPFS
      let metadataCid = '';
      let metadataUrl = '';
      
      if (this.storageClient) {
        console.log('📤 Storing fractal art on Filecoin...');
        const storageResult = await this.storageClient.storeEmotionalArt({
          canvas: generatedArt.media as any,
          emotionData: session.emotionalData.emotions[session.emotionalData.emotions.length - 1],
          biometricHash: processedSession.fractalHash,
          aiModel: 'near-fractal-ai-integration',
          generationParams: {
            title: generatedArt.title,
            description: generatedArt.description,
            fractalType: session.fractalData.type,
            complexity: processedSession.complexity,
            emotionBased: true,
            wgslCode
          }
        });
        
        metadataCid = storageResult.cid;
        metadataUrl = storageResult.url;
        console.log('✅ Fractal art stored on Filecoin:', metadataCid);
      }

      // Create NFT metadata
      const tokenId = `fractal-ai-${session.sessionId}-${Date.now()}`;
      
      const nftData = {
        token_id: tokenId,
        receiver_id: this.wallet.getAccountId(),
        fractal_data: {
          session_id: session.sessionId,
          fractal_type: session.fractalData.type,
          parameters: session.fractalData.parameters,
          complexity: processedSession.complexity,
          hash: processedSession.fractalHash,
          wgsl_code: wgslCode
        },
        emotion_data: {
          valence: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].valence,
          arousal: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].arousal,
          dominance: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].dominance,
          confidence: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].confidence || 0.8,
          timestamp: session.emotionalData.emotions[session.emotionalData.emotions.length - 1].timestamp
        },
        fractal_hash: processedSession.fractalHash,
        quality_score: processedSession.qualityScore,
        ai_analysis: processedSession.aiAnalysis,
        metadata_uri: metadataUrl,
        attributes: [
          ...(generatedArt.attributes || []),
          {
            trait_type: 'Fractal Type',
            value: session.fractalData.type
          },
          {
            trait_type: 'AI Confidence',
            value: Math.round(processedSession.aiAnalysis.styleClassification.confidence * 100)
          },
          {
            trait_type: 'Complexity Score',
            value: Math.round(processedSession.complexity * 100)
          },
          {
            trait_type: 'Quality Score',
            value: Math.round(processedSession.qualityScore * 100)
          },
          {
            trait_type: 'Emotion Vectors',
            value: session.emotionalData.emotions.length
          },
          {
            trait_type: 'Storage Type',
            value: 'Filecoin'
          },
          {
            trait_type: 'WGSL Support',
            value: wgslCode ? 'Yes' : 'No'
          }
        ]
      };

      // Mint the NFT
      console.log('⛏️ Minting AI fractal NFT...');
      const transactionHash = await (this.contract as any).mint_fractal_ai(nftData);

      console.log('✅ AI-powered fractal NFT created successfully!');

      return {
        tokenId,
        transactionHash,
        metadataCid,
        metadataUrl,
        aiAnalysis: processedSession.aiAnalysis,
        wgslCode
      };

    } catch (error) {
      console.error('❌ AI fractal NFT creation failed:', error);
      throw error;
    }
  }

  /**
   * Generate WGSL shader code using AI
   */
  async generateWGSLShader(session: FractalSession): Promise<string> {
    try {
      console.log('🔧 Generating WGSL shader with AI...');
      
      const creativeContent = await this.aiBridge.generateCreativeContent(
        'near_foundation_wgsl_shader_generation',
        {
          fractalType: session.fractalData.type,
          parameters: session.fractalData.parameters,
          emotionalState: session.emotionalData.emotions[session.emotionalData.emotions.length - 1],
          performance: session.performanceMetrics,
          grant: 'NEAR_Foundation_WGSL_Studio'
        }
      );
      
      return creativeContent.shaderCode || this.getDefaultWGSLShader(session.fractalData.type);
      
    } catch (error) {
      console.error('❌ WGSL shader generation failed:', error);
      return this.getDefaultWGSLShader(session.fractalData.type);
    }
  }

  /**
   * Find similar fractal sessions using LanceDB
   */
  async findSimilarFractals(
    emotionVector: number[],
    fractalType?: string,
    limit: number = 5
  ): Promise<any[]> {
    try {
      const searchVector = [...emotionVector];
      if (fractalType) {
        // Add fractal type encoding to vector
        const typeEncoding = this.encodeFractalType(fractalType);
        searchVector.push(typeEncoding);
      }
      
      return await this.aiBridge.searchSimilarEmotions(searchVector, limit);
    } catch (error) {
      console.error('Failed to search similar fractals:', error);
      throw error;
    }
  }

  /**
   * Get user's fractal NFTs
   */
  async getUserFractalNFTs(accountId?: string): Promise<any[]> {
    if (!this.contract) throw new Error('Contract not initialized');
    
    const userId = accountId || this.wallet?.getAccountId();
    if (!userId) throw new Error('No account ID available');
    
    try {
      const nfts = await (this.contract as any).nft_tokens_for_owner({ account_id: userId });
      return nfts.filter((nft: any) => nft.metadata?.extra?.includes('fractal'));
    } catch (error) {
      console.error('Failed to get user fractal NFTs:', error);
      throw error;
    }
  }

  /**
   * Extract fractal features for AI processing
   */
  private extractFractalFeatures(session: FractalSession): number[] {
    const features: number[] = [];
    const params = session.fractalData.parameters;
    
    // Fractal parameter features
    features.push(
      params.zoom,
      params.centerX,
      params.centerY,
      params.maxIterations,
      this.encodeFractalType(session.fractalData.type)
    );
    
    // Julia set specific features
    if (session.fractalData.type === 'julia' && params.juliaC) {
      features.push(params.juliaC.real, params.juliaC.imag);
    } else {
      features.push(0, 0);
    }
    
    // Emotional features (if available)
    if (session.emotionalData.emotions && session.emotionalData.emotions.length > 0) {
      const latestEmotion = session.emotionalData.emotions[session.emotionalData.emotions.length - 1];
      features.push(
        latestEmotion.valence,
        latestEmotion.arousal,
        latestEmotion.dominance,
        latestEmotion.confidence || 0.8
      );
    } else {
      features.push(0.5, 0.5, 0.5, 0.8); // Neutral emotion
    }
    
    // Performance features (if available)
    if (session.performanceMetrics) {
      features.push(
        this.calculateAverage(session.performanceMetrics.fps),
        this.calculateAverage(session.performanceMetrics.renderTime),
        session.performanceMetrics.complexity
      );
    } else {
      features.push(30, 16.67, 1); // Default performance
    }
    
    return features;
  }

  /**
   * Calculate fractal complexity score
   */
  private calculateFractalComplexity(session: FractalSession): number {
    const params = session.fractalData.parameters;
    
    // Base complexity from iterations
    let complexity = params.maxIterations / 1000;
    
    // Zoom factor complexity
    complexity += Math.log10(params.zoom + 1) / 10;
    
    // Fractal type complexity multiplier
    const typeMultiplier = {
      'mandelbrot': 1.0,
      'julia': 1.2,
      'burning_ship': 1.3,
      'newton': 1.5,
      'phoenix': 1.4,
      'custom': 2.0
    };
    
    complexity *= (typeMultiplier[session.fractalData.type] || 1.0);
    
    // Performance complexity factor
    if (session.performanceMetrics) {
      const avgRenderTime = this.calculateAverage(session.performanceMetrics.renderTime);
      complexity += (avgRenderTime / 33.33) * 0.5; // Normalize to 30 FPS baseline
    }
    
    return Math.min(complexity, 10.0); // Cap at 10.0
  }

  /**
   * Analyze emotional correlation with fractal parameters
   */
  private analyzeEmotionalCorrelation(
    parameters: any,
    emotions: any[]
  ): any {
    if (!emotions || emotions.length === 0) {
      return { correlation: 0, analysis: 'No emotional data available' };
    }
    
    const latestEmotion = emotions[emotions.length - 1];
    
    // Simple correlation analysis
    const zoomEmotionCorrelation = this.calculateEmotionParameterCorrelation(
      parameters.zoom,
      latestEmotion.dominance
    );
    
    const iterationEmotionCorrelation = this.calculateEmotionParameterCorrelation(
      parameters.maxIterations / 500, // Normalize
      latestEmotion.arousal
    );
    
    const centerEmotionCorrelation = this.calculateEmotionParameterCorrelation(
      Math.abs(parameters.centerX) + Math.abs(parameters.centerY),
      Math.abs(latestEmotion.valence)
    );
    
    const overallCorrelation = (zoomEmotionCorrelation + iterationEmotionCorrelation + centerEmotionCorrelation) / 3;
    
    return {
      correlation: overallCorrelation,
      zoomCorrelation: zoomEmotionCorrelation,
      iterationCorrelation: iterationEmotionCorrelation,
      centerCorrelation: centerEmotionCorrelation,
      analysis: this.generateEmotionalAnalysis(overallCorrelation, latestEmotion)
    };
  }

  /**
   * Generate fractal recommendations based on AI analysis
   */
  private async generateFractalRecommendations(analysis: any): Promise<any[]> {
    const recommendations: any[] = [];
    
    // Pattern-based recommendations
    if (analysis.patterns?.detectedPatterns) {
      for (const pattern of analysis.patterns.detectedPatterns) {
        recommendations.push({
          type: 'pattern',
          description: `Enhance ${pattern.name} patterns for better visual impact`,
          parameters: pattern.recommendedParams,
          confidence: pattern.confidence
        });
      }
    }
    
    // Style-based recommendations
    if (analysis.styleClassification?.style) {
      recommendations.push({
        type: 'style',
        description: `Consider exploring ${analysis.styleClassification.style} variations`,
        examples: analysis.styleClassification.similarStyles,
        confidence: analysis.styleClassification.confidence
      });
    }
    
    // Emotional recommendations
    if (analysis.emotionalCorrelation?.correlation < 0.5) {
      recommendations.push({
        type: 'emotion',
        description: 'Improve emotional parameter mapping for stronger artistic expression',
        suggestions: [
          'Map valence to color saturation',
          'Map arousal to animation speed',
          'Map dominance to zoom level'
        ],
        confidence: 0.8
      });
    }
    
    // Complexity recommendations
    if (analysis.complexity < 2.0) {
      recommendations.push({
        type: 'complexity',
        description: 'Increase fractal complexity for more detailed visuals',
        suggestions: [
          'Increase iteration count',
          'Add color gradient variations',
          'Implement multi-layer fractals'
        ],
        confidence: 0.7
      });
    } else if (analysis.complexity > 8.0) {
      recommendations.push({
        type: 'performance',
        description: 'Optimize for better performance while maintaining visual quality',
        suggestions: [
          'Reduce iteration count selectively',
          'Implement level-of-detail rendering',
          'Use GPU compute shaders'
        ],
        confidence: 0.8
      });
    }
    
    return recommendations;
  }

  /**
   * Generate style recommendations using AI
   */
  private async generateStyleRecommendations(
    session: FractalSession,
    patternAnalysis: any
  ): Promise<any[]> {
    try {
      const creativeContent = await this.aiBridge.generateCreativeContent(
        'near_foundation_style_recommendations',
        {
          currentSession: session,
          patternAnalysis,
          grant: 'NEAR_Foundation_Fractal_Studio'
        }
      );
      
      return creativeContent.styleRecommendations || [];
    } catch (error) {
      console.error('❌ Style recommendations generation failed:', error);
      return [];
    }
  }

  /**
   * Generate fractal hash from session data
   */
  private generateFractalHash(session: FractalSession): string {
    const dataString = JSON.stringify({
      fractalType: session.fractalData.type,
      parameters: session.fractalData.parameters,
      keyframes: session.fractalData.keyframes?.slice(-10),
      emotions: session.emotionalData.emotions?.slice(-10),
      performance: session.performanceMetrics,
      timestamp: Date.now()
    });
    
    return createHash('sha256').update(dataString).digest('hex');
  }

  /**
   * Calculate fractal quality score
   */
  private calculateFractalQualityScore(analysis: any): number {
    try {
      let qualityScore = 0.5; // Base score
      
      // Pattern recognition quality
      if (analysis.patternAnalysis?.confidence) {
        qualityScore += analysis.patternAnalysis.confidence * 0.2;
      }
      
      // Complexity appropriateness
      if (analysis.complexity >= 2.0 && analysis.complexity <= 8.0) {
        qualityScore += 0.2;
      }
      
      // Emotional correlation
      if (analysis.emotionalCorrelation?.correlation > 0.5) {
        qualityScore += analysis.emotionalCorrelation.correlation * 0.2;
      }
      
      // Performance quality
      if (analysis.performance) {
        const avgFps = this.calculateAverage(analysis.performance.fps);
        if (avgFps >= 30) qualityScore += 0.1;
        if (avgFps >= 60) qualityScore += 0.1;
      }
      
      return Math.min(qualityScore, 1.0); // Cap at 1.0
    } catch (error) {
      console.error('Error calculating fractal quality score:', error);
      return 0.5; // Default fallback
    }
  }

  /**
   * Generate fractal vectors for LanceDB storage
   */
  private generateFractalVectors(session: FractalSession): any[] {
    const vectors: any[] = [];
    
    // Create vector from latest emotional state
    if (session.emotionalData.emotions.length > 0) {
      const latestEmotion = session.emotionalData.emotions[session.emotionalData.emotions.length - 1];
      
      vectors.push({
        vector: [
          latestEmotion.valence,
          latestEmotion.arousal,
          latestEmotion.dominance,
          latestEmotion.confidence || 0.8,
          session.fractalData.parameters.zoom,
          session.fractalData.parameters.maxIterations / 500 // Normalize
        ],
        metadata: {
          sessionId: session.sessionId,
          timestamp: latestEmotion.timestamp,
          userId: session.userId,
          type: 'near_fractal',
          fractalType: session.fractalData.type,
          complexity: this.calculateFractalComplexity(session)
        }
      });
    }
    
    return vectors;
  }

  /**
   * Store fractal analysis on-chain
   */
  private async storeFractalAnalysis(sessionId: string, analysis: any): Promise<string> {
    if (!this.fractalContract) throw new Error('Fractal contract not initialized');
    
    try {
      const result = await (this.fractalContract as any).store_fractal_analysis({
        session_id: sessionId,
        analysis_data: analysis
      });
      return result;
    } catch (error) {
      console.error('Failed to store fractal analysis:', error);
      throw error;
    }
  }

  /**
   * Utility functions
   */
  private encodeFractalType(type: string): number {
    const encoding = {
      'mandelbrot': 1.0,
      'julia': 2.0,
      'burning_ship': 3.0,
      'newton': 4.0,
      'phoenix': 5.0,
      'custom': 6.0
    };
    return encoding[type] || 0;
  }

  private calculateEmotionParameterCorrelation(parameter: number, emotion: number): number {
    // Simple correlation calculation
    const normalizedParameter = Math.abs(parameter);
    const normalizedEmotion = Math.abs(emotion);
    return 1 - Math.abs(normalizedParameter - normalizedEmotion);
  }

  private generateEmotionalAnalysis(correlation: number, emotion: EmotionalVector): string {
    if (correlation > 0.7) {
      return 'Strong emotional correlation detected - fractal parameters well-aligned with emotional state';
    } else if (correlation > 0.4) {
      return 'Moderate emotional correlation - some alignment between fractal and emotion';
    } else {
      return 'Weak emotional correlation - consider adjusting parameter mapping for better artistic expression';
    }
  }

  private getDefaultWGSLShader(fractalType: string): string {
    const templates = {
      'mandelbrot': `
@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> resolution: vec2<f32>;
@group(0) @binding(2) var<uniform> zoom: f32;
@group(0) @binding(3) var<uniform> center: vec2<f32>;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    var uv = (pos.xy / resolution - 0.5) * zoom + center;
    var z = vec2<f32>(0.0);
    var iterations = 0;
    
    for (var i = 0; i < 100; i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + uv;
        iterations = i;
    }
    
    let color = f32(iterations) / 100.0;
    return vec4<f32>(vec3<f32>(color), 1.0);
}`,
      'julia': `
@group(0) @binding(0) var<uniform> time: f32;
@group(0) @binding(1) var<uniform> resolution: vec2<f32>;
@group(0) @binding(2) var<uniform> zoom: f32;
@group(0) @binding(3) var<uniform> center: vec2<f32>;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    var z = (pos.xy / resolution - 0.5) * zoom + center;
    var c = vec2<f32>(-0.7, 0.27015);
    var iterations = 0;
    
    for (var i = 0; i < 100; i = i + 1) {
        if (length(z) > 2.0) {
            break;
        }
        z = vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
        iterations = i;
    }
    
    let color = f32(iterations) / 100.0;
    return vec4<f32>(vec3<f32>(color), 1.0);
}`
    };
    
    return templates[fractalType] || templates['mandelbrot'];
  }

  private calculateAverage(data: number[]): number {
    if (!data || data.length === 0) return 0;
    return data.reduce((sum, val) => sum + val, 0) / data.length;
  }

  /**
   * Get connection status
   */
  getConnectionStatus(): boolean {
    return this.isConnected;
  }

  /**
   * Get wallet instance
   */
  getWallet(): WalletConnection | null {
    return this.wallet;
  }

  /**
   * Get account ID
   */
  getAccountId(): string | null {
    return this.wallet?.getAccountId() || null;
  }
}

// Factory function for easy instantiation
export function createNEARFractalAIIntegration(config: NEARFractalAIConfig): NEARFractalAIIntegration {
  return new NEARFractalAIIntegration(config);
}

// Utility function to validate NEAR Fractal AI setup
export function validateNEARFractalAISetup(): {
  near: boolean;
  ai: boolean;
  storage: boolean;
  fractal: boolean;
  wgsl: boolean;
  errors: string[];
} {
  const result = {
    near: false,
    ai: false,
    storage: false,
    fractal: false,
    wgsl: false,
    errors: [] as string[]
  };

  try {
    // Check if NEAR API is available
    if (typeof connect !== 'undefined') {
      result.near = true;
    } else {
      result.errors.push('NEAR API not available');
    }

    // Check if AI bridge is available
    if (typeof WASMMLBridge !== 'undefined') {
      result.ai = true;
    } else {
      result.errors.push('AI/ML bridge not available');
    }

    // Check if storage client is available
    if (typeof FilecoinStorageClient !== 'undefined') {
      result.storage = true;
    } else {
      result.errors.push('Filecoin storage client not available');
    }

    // Check if fractal processing is available (basic check)
    result.fractal = true; // Assume available if AI bridge is working
    result.wgsl = true; // Assume available if AI bridge is working

  } catch (error) {
    result.errors.push(`Setup validation failed: ${error}`);
  }

  return result;
}
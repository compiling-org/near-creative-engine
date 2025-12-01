import { WASMMLBridge } from './unified-ai-ml-integration.js';
import { createHash } from 'crypto';

/**
 * Unified AI/ML and IPFS Integration Hub for All Grant Projects
 * Coordinates AI inference, data storage, and cross-project communication
 */

export interface GrantProject {
  id: string;
  name: string;
  blockchain: 'filecoin' | 'solana' | 'near' | 'polkadot' | 'bitte';
  aiCapabilities: string[];
  ipfsConfig: {
    storageType: 'metadata' | 'models' | 'interactions' | 'analytics';
    encryption: boolean;
    replication: number;
  };
  status: 'active' | 'development' | 'maintenance';
  lastSync: number;
}

export interface AIProjectData {
  projectId: string;
  dataType: 'biometric' | 'emotion' | 'nft' | 'governance' | 'cross_chain';
  aiAnalysis: {
    model: string;
    confidence: number;
    results: any;
    timestamp: number;
  };
  ipfsHash?: string;
  filecoinDeal?: string;
  metadata: {
    size: number;
    format: string;
    encryption?: string;
    tags: string[];
  };
}

export interface CrossProjectAIAnalysis {
  projects: string[];
  analysisType: 'compatibility' | 'optimization' | 'trends' | 'anomalies';
  aiResults: {
    compatibility: number;
    recommendations: string[];
    riskFactors: string[];
    opportunities: string[];
  };
  ipfsReport: string;
  timestamp: number;
}

export class UnifiedAIIPFSHub {
  private mlBridge: WASMMLBridge;
  private projects: Map<string, GrantProject> = new Map();
  private projectData: Map<string, AIProjectData[]> = new Map();
  private crossProjectAnalyses: CrossProjectAIAnalysis[] = [];
  private ipfsClient: any;
  private filecoinClient: any;

  constructor() {
    this.mlBridge = new WASMMLBridge();
  }

  async initialize(): Promise<void> {
    try {
      // Initialize unified ML pipeline
      await this.mlBridge.initialize();

      // Initialize IPFS client
      const { create } = await import('ipfs-http-client');
      this.ipfsClient = create({ 
        url: 'https://ipfs.infura.io:5001/api/v0',
        headers: {
          authorization: 'Bearer YOUR_INFURA_PROJECT_SECRET'
        }
      });

      // Initialize Filecoin client
      await this.initializeFilecoinClient();

      // Load existing projects
      await this.loadGrantProjects();

      console.log('🚀 Unified AI/IPFS Hub initialized successfully');
    } catch (error) {
      console.error('Failed to initialize Unified AI/IPFS Hub:', error);
      throw error;
    }
  }

  /**
   * Initialize Filecoin client for decentralized storage
   */
  private async initializeFilecoinClient(): Promise<void> {
    try {
      const { LotusRPC } = await import('@filecoin-shipyard/lotus-client-rpc');
      const { NodejsProvider } = await import('@filecoin-shipyard/lotus-client-provider-nodejs');
      
      const provider = new NodejsProvider('https://api.calibration.node.glif.io/rpc/v1');
      this.filecoinClient = new LotusRPC(provider, { 
        schema: 'https://raw.githubusercontent.com/filecoin-project/lotus/master/documentation/en/api-v1-unstable-methods.md' 
      });
      
      console.log('🔗 Filecoin client initialized');
    } catch (error) {
      console.warn('Filecoin client initialization failed, using IPFS only:', error);
      this.filecoinClient = null;
    }
  }

  /**
   * Load grant projects from configuration
   */
  private async loadGrantProjects(): Promise<void> {
    const grantProjects: GrantProject[] = [
      {
        id: 'filecoin-foundation',
        name: 'Filecoin Foundation - Universal Creative Asset Storage',
        blockchain: 'filecoin',
        aiCapabilities: ['biometric_storage', 'emotion_preservation', 'creative_analysis', 'cross_chain_storage'],
        ipfsConfig: {
          storageType: 'models',
          encryption: true,
          replication: 3
        },
        status: 'active',
        lastSync: Date.now()
      },
      {
        id: 'solana-foundation',
        name: 'Solana Foundation - Neuroemotive AI + Stream Diffusion',
        blockchain: 'solana',
        aiCapabilities: ['real_time_inference', 'gpu_acceleration', 'stream_processing', 'neuroemotive_analysis'],
        ipfsConfig: {
          storageType: 'interactions',
          encryption: false,
          replication: 2
        },
        status: 'active',
        lastSync: Date.now()
      },
      {
        id: 'near-foundation',
        name: 'NEAR Foundation - Fractal Studio & WGSL Studio',
        blockchain: 'near',
        aiCapabilities: ['fractal_generation', 'wgsl_optimization', 'creative_coding', 'blockchain_integration'],
        ipfsConfig: {
          storageType: 'metadata',
          encryption: true,
          replication: 2
        },
        status: 'active',
        lastSync: Date.now()
      },
      {
        id: 'web3-foundation',
        name: 'Web3 Foundation - Cross-Chain Neuroemotive Bridge',
        blockchain: 'polkadot',
        aiCapabilities: ['cross_chain_bridge', 'emotion_detection', 'biometric_transfer', 'bridge_optimization'],
        ipfsConfig: {
          storageType: 'analytics',
          encryption: true,
          replication: 3
        },
        status: 'active',
        lastSync: Date.now()
      },
      {
        id: 'bitte-protocol',
        name: 'Bitte Protocol - AI Agent Network',
        blockchain: 'bitte',
        aiCapabilities: ['agent_coordination', 'federated_learning', 'emotion_intelligence', 'decentralized_ai'],
        ipfsConfig: {
          storageType: 'models',
          encryption: false,
          replication: 2
        },
        status: 'active',
        lastSync: Date.now()
      }
    ];

    for (const project of grantProjects) {
      this.projects.set(project.id, project);
      this.projectData.set(project.id, []);
    }

    console.log(`📋 Loaded ${grantProjects.length} grant projects`);
  }

  /**
   * Process AI data for a specific grant project
   */
  async processProjectData(
    projectId: string,
    dataType: AIProjectData['dataType'],
    rawData: any,
    options: {
      storeOnIPFS?: boolean;
      encrypt?: boolean;
      generateReport?: boolean;
    } = {}
  ): Promise<AIProjectData> {
    try {
      const project = this.projects.get(projectId);
      if (!project) {
        throw new Error(`Project ${projectId} not found`);
      }

      console.log(`🔄 Processing ${dataType} data for ${project.name}...`);

      // Perform AI analysis based on project capabilities
      const aiAnalysis = await this.performAIAnalysis(project, dataType, rawData);

      // Create project data object
      const projectData: AIProjectData = {
        projectId,
        dataType,
        aiAnalysis,
        metadata: {
          size: JSON.stringify(rawData).length,
          format: 'json',
          tags: [project.blockchain, dataType, ...project.aiCapabilities]
        }
      };

      // Store on IPFS if requested
      if (options.storeOnIPFS) {
        const storageResult = await this.storeOnIPFS(projectData, options.encrypt);
        projectData.ipfsHash = storageResult.ipfsHash;
        projectData.filecoinDeal = storageResult.filecoinDeal;
      }

      // Store in local cache
      const projectDataList = this.projectData.get(projectId) || [];
      projectDataList.push(projectData);
      this.projectData.set(projectId, projectDataList);

      console.log(`✅ Processed ${dataType} data for ${project.name}`);
      return projectData;
    } catch (error) {
      console.error(`Failed to process project data for ${projectId}:`, error);
      throw error;
    }
  }

  /**
   * Perform AI analysis based on project capabilities
   */
  private async performAIAnalysis(
    project: GrantProject,
    dataType: AIProjectData['dataType'],
    rawData: any
  ): Promise<AIProjectData['aiAnalysis']> {
    try {
      let results: any;
      let model: string;
      let confidence: number;

      switch (dataType) {
        case 'biometric':
          // Use Iron Learn for biometric analysis
          const biometricResult = await this.mlBridge.processWithIronLearn(
            rawData.emotionVector || rawData.biometricData,
            'biometric'
          );
          results = biometricResult;
          model = 'IronLearn-Biometric-v1';
          confidence = biometricResult.confidence || 0.85;
          break;

        case 'emotion':
          // Use Candle for emotion detection
          const emotionResult = await this.mlBridge.processWithCandle(
            JSON.stringify(rawData),
            'emotion_detection'
          );
          results = emotionResult;
          model = 'Candle-Emotion-v2';
          confidence = emotionResult.confidence || 0.90;
          break;

        case 'nft':
          // Use LanceDB for NFT similarity analysis
          const nftResult = await this.mlBridge.queryLanceDB(
            'nft_analysis',
            rawData.featureVector || rawData.metadata,
            5
          );
          results = nftResult;
          model = 'LanceDB-NFT-Similarity-v1';
          confidence = nftResult[0]?.score || 0.75;
          break;

        case 'governance':
          // Use Iron Learn for governance prediction
          const governanceResult = await this.mlBridge.processWithIronLearn(
            rawData.votingData || rawData.proposalData,
            'governance_prediction'
          );
          results = governanceResult;
          model = 'IronLearn-Governance-v1';
          confidence = governanceResult.confidence || 0.80;
          break;

        case 'cross_chain':
          // Use unified pipeline for cross-chain analysis
          const crossChainResult = await this.performCrossChainAnalysis(rawData);
          results = crossChainResult;
          model = 'Unified-CrossChain-v3';
          confidence = crossChainResult.compatibility || 0.88;
          break;

        default:
          throw new Error(`Unsupported data type: ${dataType}`);
      }

      return {
        model,
        confidence,
        results,
        timestamp: Date.now()
      };
    } catch (error) {
      console.warn(`AI analysis failed for ${dataType}, using fallback:`, error);
      return {
        model: 'Fallback-Analysis',
        confidence: 0.5,
        results: { error: error.message, rawData },
        timestamp: Date.now()
      };
    }
  }

  /**
   * Perform cross-chain analysis using unified ML pipeline
   */
  private async performCrossChainAnalysis(rawData: any): Promise<any> {
    try {
      // Extract features from cross-chain data
      const features = this.extractCrossChainFeatures(rawData);
      
      // Use LanceDB for similarity analysis
      const similarityResults = await this.mlBridge.queryLanceDB(
        'cross_chain_compatibility',
        features,
        10
      );

      // Use Iron Learn for optimization
      const optimizationResults = await this.mlBridge.processWithIronLearn(
        features,
        'cross_chain_optimization'
      );

      // Use Candle for validation
      const validationResults = await this.mlBridge.processWithCandle(
        JSON.stringify(rawData),
        'cross_chain_validation'
      );

      return {
        compatibility: (similarityResults[0]?.score || 75) / 100,
        optimization: optimizationResults,
        validation: validationResults,
        recommendations: this.generateCrossChainRecommendations(similarityResults),
        riskAssessment: this.assessCrossChainRisks(validationResults)
      };
    } catch (error) {
      console.error('Cross-chain analysis failed:', error);
      return {
        compatibility: 0.5,
        error: error.message
      };
    }
  }

  /**
   * Extract features from cross-chain data
   */
  private extractCrossChainFeatures(rawData: any): number[] {
    // Extract numerical features for ML processing
    const features = [];
    
    if (rawData.chains) {
      features.push(rawData.chains.length);
    }
    
    if (rawData.transferData) {
      features.push(rawData.transferData.amount || 0);
      features.push(rawData.transferData.confidence || 0.5);
    }
    
    if (rawData.emotionData) {
      features.push(...(rawData.emotionData.vector || [0.5, 0.5, 0.5, 0.5, 0.5, 0.5]));
    }
    
    // Pad to 10 features if necessary
    while (features.length < 10) {
      features.push(0.5);
    }
    
    return features.slice(0, 10);
  }

  /**
   * Generate cross-chain recommendations
   */
  private generateCrossChainRecommendations(similarityResults: any[]): string[] {
    const recommendations = [];
    
    if (similarityResults.length > 0) {
      const topResult = similarityResults[0];
      if (topResult.score > 80) {
        recommendations.push('High compatibility detected - proceed with standard protocol');
      } else if (topResult.score > 60) {
        recommendations.push('Moderate compatibility - implement additional validation');
      } else {
        recommendations.push('Low compatibility - consider alternative chains or enhanced bridging');
      }
    }
    
    recommendations.push('Monitor cross-chain performance metrics');
    recommendations.push('Implement fallback mechanisms for failed transfers');
    
    return recommendations;
  }

  /**
   * Assess cross-chain risks
   */
  private assessCrossChainRisks(validationResults: any): any {
    const risks = [];
    
    if (validationResults.anomalies && validationResults.anomalies.length > 0) {
      risks.push('Anomalies detected in cross-chain data');
    }
    
    if (validationResults.confidence && validationResults.confidence < 0.7) {
      risks.push('Low confidence in validation results');
    }
    
    if (risks.length === 0) {
      risks.push('No significant risks detected');
    }
    
    return {
      riskLevel: risks.length > 1 ? 'high' : risks.length === 1 ? 'medium' : 'low',
      riskFactors: risks
    };
  }

  /**
   * Store data on IPFS with optional encryption
   */
  private async storeOnIPFS(
    data: AIProjectData,
    encrypt: boolean = false
  ): Promise<{ ipfsHash: string; filecoinDeal?: string }> {
    try {
      let dataToStore = JSON.stringify(data, null, 2);
      
      // Encrypt if requested
      if (encrypt) {
        dataToStore = await this.encryptData(dataToStore);
      }
      
      // Store on IPFS
      const { cid } = await this.ipfsClient.add(dataToStore, {
        pin: true,
        cidVersion: 1
      });
      
      const ipfsHash = cid.toString();
      console.log(`📤 Stored on IPFS: ${ipfsHash}`);
      
      let filecoinDeal: string | undefined;
      
      // Attempt Filecoin storage if available
      if (this.filecoinClient && data.metadata.size > 1000) {
        try {
          filecoinDeal = await this.storeOnFilecoin(ipfsHash, dataToStore);
        } catch (error) {
          console.warn('Filecoin storage failed:', error);
        }
      }
      
      return { ipfsHash, filecoinDeal };
    } catch (error) {
      console.error('IPFS storage failed:', error);
      throw error;
    }
  }

  /**
   * Store data on Filecoin for long-term persistence
   */
  private async storeOnFilecoin(ipfsHash: string, data: string): Promise<string> {
    try {
      // Create storage deal proposal
      const dealProposal = {
        Data: {
          TransferType: 'graphsync',
          Root: {
            '/': ipfsHash
          }
        },
        Wallet: 'f1abjxfbp274xpdqcpuaykwkfb43inzt4awoowdya', // Example wallet
        Miner: 'f01234', // Example miner
        EpochPrice: '1000000000000', // Price in attoFIL
        MinBlocksDuration: 518400 // ~6 months
      };
      
      // Initiate storage deal
      const dealResult = await this.filecoinClient.clientStartDeal(dealProposal);
      
      console.log(`💎 Filecoin storage deal initiated: ${dealResult['/']}`);
      return dealResult['/'];
    } catch (error) {
      console.error('Filecoin storage failed:', error);
      throw error;
    }
  }

  /**
   * Encrypt data before storage
   */
  private async encryptData(data: string): Promise<string> {
    // Simple encryption (in production, use proper encryption)
    const crypto = await import('crypto');
    const algorithm = 'aes-256-cbc';
    const key = crypto.randomBytes(32);
    const iv = crypto.randomBytes(16);
    
    const cipher = crypto.createCipher(algorithm, key);
    let encrypted = cipher.update(data, 'utf8', 'hex');
    encrypted += cipher.final('hex');
    
    return JSON.stringify({
      encrypted,
      key: key.toString('hex'),
      iv: iv.toString('hex'),
      algorithm
    });
  }

  /**
   * Perform cross-project AI analysis
   */
  async performCrossProjectAnalysis(
    projectIds: string[],
    analysisType: CrossProjectAIAnalysis['analysisType']
  ): Promise<CrossProjectAIAnalysis> {
    try {
      console.log(`🔍 Performing ${analysisType} analysis across ${projectIds.length} projects...`);
      
      // Collect data from all projects
      const allProjectData: AIProjectData[] = [];
      for (const projectId of projectIds) {
        const projectData = this.projectData.get(projectId) || [];
        allProjectData.push(...projectData);
      }
      
      // Perform unified AI analysis
      const analysisResults = await this.performUnifiedAnalysis(allProjectData, analysisType);
      
      // Generate comprehensive report
      const report = {
        projects: projectIds,
        analysisType,
        aiResults: analysisResults,
        ipfsReport: '',
        timestamp: Date.now()
      };
      
      // Store report on IPFS
      const { cid } = await this.ipfsClient.add(JSON.stringify(report, null, 2));
      report.ipfsReport = cid.toString();
      
      this.crossProjectAnalyses.push(report);
      
      console.log(`✅ Cross-project analysis completed: ${analysisType}`);
      return report;
    } catch (error) {
      console.error('Cross-project analysis failed:', error);
      throw error;
    }
  }

  /**
   * Perform unified analysis across multiple projects
   */
  private async performUnifiedAnalysis(
    projectData: AIProjectData[],
    analysisType: CrossProjectAIAnalysis['analysisType']
  ): Promise<CrossProjectAIAnalysis['aiResults']> {
    try {
      switch (analysisType) {
        case 'compatibility':
          return await this.analyzeCompatibility(projectData);
        case 'optimization':
          return await this.analyzeOptimization(projectData);
        case 'trends':
          return await this.analyzeTrends(projectData);
        case 'anomalies':
          return await this.detectAnomalies(projectData);
        default:
          throw new Error(`Unsupported analysis type: ${analysisType}`);
      }
    } catch (error) {
      console.error('Unified analysis failed:', error);
      return {
        compatibility: 0,
        recommendations: ['Analysis failed - check logs'],
        riskFactors: [error.message],
        opportunities: []
      };
    }
  }

  /**
   * Analyze compatibility across projects
   */
  private async analyzeCompatibility(projectData: AIProjectData[]): Promise<any> {
    const compatibilityScores = [];
    const recommendations = [];
    const riskFactors = [];
    const opportunities = [];
    
    // Group data by type
    const dataByType = this.groupDataByType(projectData);
    
    // Analyze compatibility within each type
    for (const [dataType, data] of Object.entries(dataByType)) {
      if (data.length < 2) continue;
      
      const avgConfidence = data.reduce((sum, item) => sum + item.aiAnalysis.confidence, 0) / data.length;
      compatibilityScores.push(avgConfidence);
      
      if (avgConfidence > 0.8) {
        opportunities.push(`High compatibility in ${dataType} data across projects`);
      } else if (avgConfidence < 0.6) {
        riskFactors.push(`Low compatibility in ${dataType} data across projects`);
      }
    }
    
    // Cross-type compatibility analysis
    const overallCompatibility = compatibilityScores.length > 0 
      ? compatibilityScores.reduce((sum, score) => sum + score, 0) / compatibilityScores.length 
      : 0;
    
    recommendations.push('Standardize data formats across projects for better compatibility');
    recommendations.push('Implement cross-project validation protocols');
    
    return {
      compatibility: overallCompatibility,
      recommendations,
      riskFactors,
      opportunities
    };
  }

  /**
   * Analyze optimization opportunities
   */
  private async analyzeOptimization(projectData: AIProjectData[]): Promise<any> {
    const recommendations = [];
    const riskFactors = [];
    const opportunities = [];
    
    // Analyze performance metrics
    const performanceData = projectData.map(item => ({
      confidence: item.aiAnalysis.confidence,
      timestamp: item.aiAnalysis.timestamp,
      projectId: item.projectId
    }));
    
    // Identify optimization opportunities
    const lowConfidenceProjects = performanceData.filter(item => item.confidence < 0.7);
    if (lowConfidenceProjects.length > 0) {
      recommendations.push(`Improve AI model performance for ${lowConfidenceProjects.length} projects`);
      riskFactors.push('Low confidence predictions may impact reliability');
    }
    
    const recentProjects = performanceData.filter(item => 
      Date.now() - item.timestamp < 7 * 24 * 60 * 60 * 1000 // Last 7 days
    );
    if (recentProjects.length > 0) {
      opportunities.push('Recent data available for real-time optimization');
    }
    
    return {
      compatibility: 0.75,
      recommendations: [
        ...recommendations,
        'Implement model ensemble techniques for better accuracy',
        'Use transfer learning across similar projects',
        'Optimize storage strategies based on data access patterns'
      ],
      riskFactors,
      opportunities
    };
  }

  /**
   * Analyze trends across projects
   */
  private async analyzeTrends(projectData: AIProjectData[]): Promise<any> {
    const trends = [];
    const recommendations = [];
    
    // Time-based trend analysis
    const timeGroups = this.groupDataByTime(projectData);
    
    for (const [timeGroup, data] of Object.entries(timeGroups)) {
      const avgConfidence = data.reduce((sum, item) => sum + item.aiAnalysis.confidence, 0) / data.length;
      trends.push({
        timeGroup,
        avgConfidence,
        dataPoints: data.length
      });
    }
    
    // Project-based trend analysis
    const projectGroups = this.groupDataByProject(projectData);
    
    for (const [projectId, data] of Object.entries(projectGroups)) {
      const project = this.projects.get(projectId);
      const avgConfidence = data.reduce((sum, item) => sum + item.aiAnalysis.confidence, 0) / data.length;
      trends.push({
        projectId,
        projectName: project?.name || projectId,
        avgConfidence,
        dataPoints: data.length
      });
    }
    
    recommendations.push('Monitor confidence trends for early detection of model degradation');
    recommendations.push('Implement automated retraining based on trend analysis');
    
    return {
      compatibility: 0.8,
      recommendations,
      riskFactors: [],
      opportunities: ['Trend data available for predictive analytics']
    };
  }

  /**
   * Detect anomalies across projects
   */
  private async detectAnomalies(projectData: AIProjectData[]): Promise<any> {
    const anomalies = [];
    const recommendations = [];
    const riskFactors = [];
    
    // Statistical anomaly detection
    const confidences = projectData.map(item => item.aiAnalysis.confidence);
    const mean = confidences.reduce((sum, val) => sum + val, 0) / confidences.length;
    const stdDev = Math.sqrt(confidences.reduce((sum, val) => sum + Math.pow(val - mean, 2), 0) / confidences.length);
    
    // Identify outliers
    const threshold = 2 * stdDev;
    confidences.forEach((confidence, index) => {
      if (Math.abs(confidence - mean) > threshold) {
        anomalies.push({
          projectId: projectData[index].projectId,
          dataType: projectData[index].dataType,
          confidence,
          deviation: Math.abs(confidence - mean),
          severity: Math.abs(confidence - mean) > 3 * stdDev ? 'high' : 'medium'
        });
      }
    });
    
    if (anomalies.length > 0) {
      riskFactors.push(`${anomalies.length} anomalies detected in AI confidence scores`);
      recommendations.push('Investigate anomalous data points for potential issues');
      recommendations.push('Consider retraining models if anomalies persist');
    }
    
    return {
      compatibility: anomalies.length === 0 ? 0.95 : 0.6,
      recommendations,
      riskFactors,
      opportunities: anomalies.length === 0 ? ['No anomalies detected - system operating normally'] : []
    };
  }

  /**
   * Helper methods for data grouping
   */
  private groupDataByType(projectData: AIProjectData[]): Record<string, AIProjectData[]> {
    const groups: Record<string, AIProjectData[]> = {};
    
    for (const item of projectData) {
      if (!groups[item.dataType]) {
        groups[item.dataType] = [];
      }
      groups[item.dataType].push(item);
    }
    
    return groups;
  }

  private groupDataByTime(projectData: AIProjectData[]): Record<string, AIProjectData[]> {
    const groups: Record<string, AIProjectData[]> = {
      'last-24h': [],
      'last-7d': [],
      'last-30d': [],
      'older': []
    };
    
    const now = Date.now();
    
    for (const item of projectData) {
      const age = now - item.aiAnalysis.timestamp;
      
      if (age < 24 * 60 * 60 * 1000) {
        groups['last-24h'].push(item);
      } else if (age < 7 * 24 * 60 * 60 * 1000) {
        groups['last-7d'].push(item);
      } else if (age < 30 * 24 * 60 * 60 * 1000) {
        groups['last-30d'].push(item);
      } else {
        groups['older'].push(item);
      }
    }
    
    return groups;
  }

  private groupDataByProject(projectData: AIProjectData[]): Record<string, AIProjectData[]> {
    const groups: Record<string, AIProjectData[]> = {};
    
    for (const item of projectData) {
      if (!groups[item.projectId]) {
        groups[item.projectId] = [];
      }
      groups[item.projectId].push(item);
    }
    
    return groups;
  }

  /**
   * Get system statistics
   */
  getSystemStatistics() {
    const totalProjects = this.projects.size;
    const totalDataPoints = Array.from(this.projectData.values())
      .reduce((sum, data) => sum + data.length, 0);
    const totalAnalyses = this.crossProjectAnalyses.length;
    
    const projectStats = Array.from(this.projects.values()).map(project => ({
      id: project.id,
      name: project.name,
      dataPoints: this.projectData.get(project.id)?.length || 0,
      lastSync: project.lastSync,
      status: project.status
    }));
    
    return {
      totalProjects,
      totalDataPoints,
      totalAnalyses,
      projectStats,
      ipfsStorage: {
        enabled: !!this.ipfsClient,
        filecoinEnabled: !!this.filecoinClient
      },
      mlPipeline: {
        initialized: this.mlBridge ? true : false,
        models: ['IronLearn', 'Candle', 'LanceDB']
      }
    };
  }

  /**
   * Get project by ID
   */
  getProject(projectId: string): GrantProject | undefined {
    return this.projects.get(projectId);
  }

  /**
   * Get all projects
   */
  getAllProjects(): GrantProject[] {
    return Array.from(this.projects.values());
  }

  /**
   * Get project data
   */
  getProjectData(projectId: string): AIProjectData[] {
    return this.projectData.get(projectId) || [];
  }

  /**
   * Get cross-project analyses
   */
  getCrossProjectAnalyses(): CrossProjectAIAnalysis[] {
    return this.crossProjectAnalyses;
  }

  /**
   * Cleanup resources
   */
  async cleanup(): Promise<void> {
    if (this.mlBridge) {
      await this.mlBridge.cleanup();
    }
    console.log('🧹 Unified AI/IPFS Hub cleanup completed');
  }
}

/**
 * Utility functions
 */

export function createUnifiedDataPackage(
  projectId: string,
  dataType: AIProjectData['dataType'],
  rawData: any
): any {
  return {
    projectId,
    dataType,
    rawData,
    timestamp: Date.now(),
    version: '1.0'
  };
}

export async function runUnifiedAIIPFSExamples(): Promise<void> {
  console.log('🚀 Starting Unified AI/IPFS Integration Examples...\n');
  
  const hub = new UnifiedAIIPFSHub();
  
  try {
    await hub.initialize();
    
    // Example 1: Process biometric data for Filecoin project
    console.log('📊 Example 1: Processing biometric data for Filecoin Foundation');
    const filecoinBiometric = await hub.processProjectData(
      'filecoin-foundation',
      'biometric',
      {
        emotionVector: [0.8, 0.2, 0.1, 0.9, 0.3, 0.7],
        biometricHash: '0x1234567890abcdef',
        qualityScore: 0.95,
        timestamp: Date.now()
      },
      { storeOnIPFS: true, encrypt: true }
    );
    console.log(`✅ Biometric analysis completed: ${filecoinBiometric.aiAnalysis.confidence}% confidence`);
    console.log(`📤 Stored on IPFS: ${filecoinBiometric.ipfsHash}`);
    
    // Example 2: Process emotion data for Solana project
    console.log('\n🎭 Example 2: Processing emotion data for Solana Foundation');
    const solanaEmotion = await hub.processProjectData(
      'solana-foundation',
      'emotion',
      {
        userId: 'user_123',
        sessionData: {
          happiness: 0.8,
          sadness: 0.2,
          anger: 0.1,
          fear: 0.3,
          surprise: 0.6,
          neutral: 0.5
        },
        context: 'NFT_creation',
        timestamp: Date.now()
      },
      { storeOnIPFS: true }
    );
    console.log(`✅ Emotion analysis completed: ${solanaEmotion.aiAnalysis.confidence}% confidence`);
    
    // Example 3: Cross-project analysis
    console.log('\n🌉 Example 3: Performing cross-project compatibility analysis');
    const crossProjectAnalysis = await hub.performCrossProjectAnalysis(
      ['filecoin-foundation', 'solana-foundation', 'near-foundation'],
      'compatibility'
    );
    console.log(`🔗 Cross-project compatibility: ${(crossProjectAnalysis.aiResults.compatibility * 100).toFixed(1)}%`);
    console.log(`📊 Recommendations:`, crossProjectAnalysis.aiResults.recommendations);
    
    // Display system statistics
    console.log('\n📈 System Statistics:');
    const stats = hub.getSystemStatistics();
    console.log(`  Total projects: ${stats.totalProjects}`);
    console.log(`  Total data points: ${stats.totalDataPoints}`);
    console.log(`  Total analyses: ${stats.totalAnalyses}`);
    console.log(`  IPFS storage: ${stats.ipfsStorage.enabled ? 'Enabled' : 'Disabled'}`);
    console.log(`  Filecoin storage: ${stats.ipfsStorage.filecoinEnabled ? 'Enabled' : 'Disabled'}`);
    
    console.log('\n✅ Unified AI/IPFS Integration examples completed successfully!');
    
  } catch (error) {
    console.error('❌ Unified AI/IPFS examples failed:', error);
  } finally {
    await hub.cleanup();
  }
}
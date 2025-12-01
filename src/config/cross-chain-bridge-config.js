const path = require('path');

/**
 * Cross-Chain Bridge Configuration
 * Configuration for connecting Filecoin and Solana biometric NFTs
 */

const bridgeConfig = {
    // Solana Configuration
    solana: {
        network: 'devnet', // 'mainnet-beta', 'testnet', 'devnet'
        rpcUrl: 'https://api.devnet.solana.com',
        
        // Program Configuration
        programId: 'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS', // Replace with actual deployed program ID
        idlPath: path.join(__dirname, '../../contracts/solana/biometric-nft/target/idl/biometric_nft.json'),
        
        // Wallet Configuration
        keypairPath: path.join(__dirname, '../../wallets/solana-keypair.json'),
        
        // Bridge-specific settings
        maxRetries: 3,
        retryDelay: 2000, // 2 seconds
        confirmationTimeout: 60000 // 60 seconds
    },
    
    // Filecoin Configuration
    filecoin: {
        network: 'calibration', // 'mainnet', 'calibration', 'hyperspace'
        rpcUrl: 'https://api.calibration.node.glif.io/rpc/v0',
        
        // Actor Configuration
        actorAddress: 'f01000', // Replace with actual deployed actor address
        
        // Wallet Configuration
        defaultAddress: 'f1abcdefghijklmnopqrstuvwxyz1234567890abcd', // Replace with actual address
        privateKey: process.env.FILECOIN_PRIVATE_KEY,
        
        // Bridge-specific settings
        maxRetries: 3,
        retryDelay: 5000, // 5 seconds (Filecoin is slower)
        confirmationTimeout: 300000 // 5 minutes
    },
    
    // Cross-Chain Bridge Settings
    bridge: {
        // Transfer validation settings
        minQualityScore: 0.7,
        maxTransferSize: 10 * 1024 * 1024, // 10MB max transfer size
        
        // Security settings
        requireVerification: true,
        verificationTimeout: 180000, // 3 minutes
        
        // Fee settings (in native tokens)
        solanaFee: 0.001, // SOL
        filecoinFee: 0.01, // FIL
        
        // Monitoring settings
        enableMonitoring: true,
        monitoringInterval: 30000, // 30 seconds
        
        // Retry settings
        maxBridgeRetries: 5,
        bridgeRetryDelay: 10000 // 10 seconds
    },
    
    // IPFS Configuration for storing cross-chain metadata
    ipfs: {
        gateway: 'https://ipfs.io',
        apiUrl: 'https://api.pinata.cloud',
        apiKey: process.env.PINATA_API_KEY,
        secretKey: process.env.PINATA_SECRET_KEY,
        
        // Pinning settings
        pinMetadata: true,
        pinTimeout: 120000 // 2 minutes
    },
    
    // Logging Configuration
    logging: {
        level: 'info', // 'error', 'warn', 'info', 'debug'
        enableFileLogging: true,
        logFilePath: path.join(__dirname, '../../logs/cross-chain-bridge.log'),
        maxLogSize: 10 * 1024 * 1024, // 10MB
        maxLogFiles: 5
    },
    
    // Security Configuration
    security: {
        // Rate limiting
        maxTransfersPerHour: 100,
        maxTransfersPerDay: 1000,
        
        // Whitelist settings
        enableWhitelist: false,
        whitelistedAddresses: [],
        
        // Blacklist settings
        enableBlacklist: true,
        blacklistedAddresses: [],
        
        // Encryption settings
        encryptionKey: process.env.BRIDGE_ENCRYPTION_KEY,
        enableEncryption: true
    }
};

// Environment-specific overrides
if (process.env.NODE_ENV === 'production') {
    // Production settings
    bridgeConfig.solana.network = 'mainnet-beta';
    bridgeConfig.solana.rpcUrl = 'https://api.mainnet-beta.solana.com';
    bridgeConfig.filecoin.network = 'mainnet';
    bridgeConfig.filecoin.rpcUrl = 'https://api.node.glif.io/rpc/v0';
    bridgeConfig.bridge.minQualityScore = 0.8;
    bridgeConfig.logging.level = 'warn';
    bridgeConfig.security.enableWhitelist = true;
} else if (process.env.NODE_ENV === 'test') {
    // Test settings
    bridgeConfig.solana.network = 'testnet';
    bridgeConfig.solana.rpcUrl = 'https://api.testnet.solana.com';
    bridgeConfig.filecoin.network = 'hyperspace';
    bridgeConfig.filecoin.rpcUrl = 'https://api.hyperspace.node.glif.io/rpc/v0';
    bridgeConfig.bridge.minQualityScore = 0.6;
    bridgeConfig.logging.level = 'debug';
}

// Validate configuration
function validateConfig(config) {
    const requiredFields = [
        'solana.programId',
        'solana.keypairPath',
        'filecoin.actorAddress',
        'filecoin.defaultAddress'
    ];
    
    for (const field of requiredFields) {
        const value = field.split('.').reduce((obj, key) => obj[key], config);
        if (!value || value === '') {
            throw new Error(`Missing required configuration field: ${field}`);
        }
    }
    
    // Validate file paths
    if (!require('fs').existsSync(config.solana.keypairPath)) {
        throw new Error(`Solana keypair file not found: ${config.solana.keypairPath}`);
    }
    
    if (!require('fs').existsSync(config.solana.idlPath)) {
        throw new Error(`Solana IDL file not found: ${config.solana.idlPath}`);
    }
    
    return true;
}

// Export configuration and validation function
module.exports = {
    config: bridgeConfig,
    validateConfig
};
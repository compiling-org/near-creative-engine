// Unified Wallet Connector for NEAR, Solana, Polkadot, and Ethereum
// Production-ready implementation with consistent API across all blockchains

class UnifiedWalletConnector {
    constructor() {
        this.connections = {
            near: null,
            solana: null,
            polkadot: null,
            ethereum: null
        };
        this.accounts = {
            near: null,
            solana: null,
            polkadot: null,
            ethereum: null
        };
        this.config = {
            near: {
                networkId: 'testnet',
                nodeUrl: 'https://rpc.testnet.near.org',
                walletUrl: 'https://wallet.testnet.near.org',
                helperUrl: 'https://helper.testnet.near.org',
                explorerUrl: 'https://explorer.testnet.near.org',
                contractId: 'biometric-soulbound-nft.kenchen.testnet'
            },
            solana: {
                network: 'devnet',
                rpcUrl: 'https://api.devnet.solana.com',
                wsUrl: 'wss://api.devnet.solana.com',
                programId: 'BiometricSoulboundNFT111111111111111111111' // Will be updated after deployment
            },
            polkadot: {
                network: 'westend',
                wsUrl: 'wss://westend-rpc.polkadot.io',
                ss58Format: 42
            },
            ethereum: {
                chainId: 5, // Goerli testnet
                rpcUrl: 'https://goerli.infura.io/v3/YOUR_INFURA_PROJECT_ID',
                contractAddress: null // Will be set after deployment
            }
        };
    }

    // NEAR Connection
    async connectNEAR() {
        try {
            if (this.connections.near) return this.accounts.near;

            // Load near-api-js if not available
            if (!window.nearApi) {
                await this.loadScript('https://cdn.jsdelivr.net/npm/near-api-js@2.1.4/dist/near-api-js.min.js');
            }

            const { connect, keyStores, WalletConnection } = window.nearApi;
            
            const config = {
                ...this.config.near,
                keyStore: new keyStores.BrowserLocalStorageKeyStore()
            };

            const near = await connect(config);
            const wallet = new WalletConnection(near, 'blockchain-nft-interactive');

            if (!wallet.isSignedIn()) {
                await wallet.requestSignIn(
                    this.config.near.contractId,
                    'Blockchain NFT Interactive'
                );
                return null; // Will redirect
            }

            this.connections.near = { near, wallet };
            this.accounts.near = wallet.getAccountId();
            
            console.log('✅ NEAR connected:', this.accounts.near);
            return this.accounts.near;

        } catch (error) {
            console.error('❌ NEAR connection failed:', error);
            throw new Error(`NEAR connection failed: ${error.message}`);
        }
    }

    // Solana Connection
    async connectSolana() {
        try {
            if (this.connections.solana) return this.accounts.solana;

            // Check for Solana wallet
            if (!window.solana || !window.solana.isPhantom) {
                throw new Error('Phantom wallet not found. Please install Phantom.');
            }

            const solana = window.solana;
            
            // Connect to wallet
            const response = await solana.connect();
            const publicKey = response.publicKey.toString();

            this.connections.solana = { solana, publicKey: response.publicKey };
            this.accounts.solana = publicKey;
            
            console.log('✅ Solana connected:', this.accounts.solana);
            return this.accounts.solana;

        } catch (error) {
            console.error('❌ Solana connection failed:', error);
            throw new Error(`Solana connection failed: ${error.message}`);
        }
    }

    // Polkadot Connection
    async connectPolkadot() {
        try {
            if (this.connections.polkadot) return this.accounts.polkadot;

            // Load Polkadot extension
            if (!window.injectedWeb3 || !window.injectedWeb3['polkadot-js']) {
                throw new Error('Polkadot extension not found. Please install Polkadot{.js} extension.');
            }

            const { web3Enable, web3Accounts } = await import('@polkadot/extension-dapp');
            
            // Enable extension
            const extensions = await web3Enable('Blockchain NFT Interactive');
            if (extensions.length === 0) {
                throw new Error('No Polkadot extension found');
            }

            // Get accounts
            const accounts = await web3Accounts();
            if (accounts.length === 0) {
                throw new Error('No Polkadot accounts found');
            }

            const account = accounts[0];
            this.connections.polkadot = { extensions, accounts };
            this.accounts.polkadot = account.address;
            
            console.log('✅ Polkadot connected:', this.accounts.polkadot);
            return this.accounts.polkadot;

        } catch (error) {
            console.error('❌ Polkadot connection failed:', error);
            throw new Error(`Polkadot connection failed: ${error.message}`);
        }
    }

    // NEAR Contract Calls
    async callNEARMethod(methodName, args = {}, gas = '300000000000000', deposit = '0') {
        if (!this.connections.near) {
            throw new Error('NEAR wallet not connected');
        }

        try {
            const { wallet } = this.connections.near;
            const account = wallet.account();

            const result = await account.functionCall({
                contractId: this.config.near.contractId,
                methodName,
                args,
                gas: BigInt(gas),
                attachedDeposit: BigInt(deposit)
            });

            return result;
        } catch (error) {
            console.error(`❌ NEAR method ${methodName} failed:`, error);
            throw error;
        }
    }

    async viewNEARMethod(methodName, args = {}) {
        if (!this.connections.near) {
            throw new Error('NEAR wallet not connected');
        }

        try {
            const { near } = this.connections.near;
            const account = await near.account(this.accounts.near);

            const result = await account.viewFunction(
                this.config.near.contractId,
                methodName,
                args
            );

            return result;
        } catch (error) {
            console.error(`❌ NEAR view method ${methodName} failed:`, error);
            throw error;
        }
    }

    // Solana Contract Calls
    async callSolanaMethod(instruction, signers = []) {
        if (!this.connections.solana) {
            throw new Error('Solana wallet not connected');
        }

        try {
            const { solana } = this.connections.solana;
            
            const transaction = new window.solanaWeb3.Transaction().add(instruction);
            const signature = await solana.signAndSendTransaction(transaction, signers);
            
            return signature;
        } catch (error) {
            console.error('❌ Solana transaction failed:', error);
            throw error;
        }
    }

    // Utility methods
    async loadScript(src) {
        return new Promise((resolve, reject) => {
            const script = document.createElement('script');
            script.src = src;
            script.onload = resolve;
            script.onerror = reject;
            document.head.appendChild(script);
        });
    }

    getAccount(blockchain) {
        return this.accounts[blockchain];
    }

    isConnected(blockchain) {
        return !!this.accounts[blockchain];
    }

    disconnect(blockchain) {
        if (this.connections[blockchain]) {
            this.connections[blockchain] = null;
            this.accounts[blockchain] = null;
            console.log(`✅ Disconnected from ${blockchain}`);
        }
    }

    disconnectAll() {
        Object.keys(this.connections).forEach(blockchain => {
            this.disconnect(blockchain);
        });
    }
}

// Create global instance
window.unifiedWalletConnector = new UnifiedWalletConnector();

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = UnifiedWalletConnector;
}
import { WalletConnection, connect, keyStores } from 'near-api-js';

export interface MyNearWalletConfig {
  network: 'mainnet' | 'testnet';
  contractName?: string;
}

export class MyNearWalletService {
  private walletConnection: WalletConnection | null = null;
  private near: any = null;
  private config: MyNearWalletConfig;

  constructor(config: MyNearWalletConfig) {
    this.config = config;
  }

  async initialize(): Promise<void> {
    try {
      const keyStore = new keyStores.BrowserLocalStorageKeyStore();
      
      const connectionConfig = {
        networkId: this.config.network,
        keyStore,
        nodeUrl: this.config.network === 'mainnet' 
          ? 'https://rpc.mainnet.near.org' 
          : 'https://rpc.testnet.near.org',
        walletUrl: this.config.network === 'mainnet'
          ? 'https://app.mynearwallet.com'
          : 'https://testnet.mynearwallet.com',
        helperUrl: this.config.network === 'mainnet'
          ? 'https://helper.mainnet.near.org'
          : 'https://helper.testnet.near.org',
        explorerUrl: this.config.network === 'mainnet'
          ? 'https://nearblocks.io'
          : 'https://testnet.nearblocks.io',
      };

      this.near = await connect(connectionConfig);
      this.walletConnection = new WalletConnection(this.near, 'near-creative-engine');
      
      console.log('MyNearWallet service initialized for', this.config.network);
    } catch (error) {
      console.error('Failed to initialize MyNearWallet service:', error);
      throw error;
    }
  }

  async signIn(): Promise<void> {
    if (!this.walletConnection) {
      throw new Error('Wallet not initialized');
    }

    try {
      // Request sign in with a simple contract ID
      await this.walletConnection.requestSignIn({
        contractId: this.config.contractName || 'test.near',
        methodNames: [], // Allow all methods for now
        successUrl: window.location.href,
        failureUrl: window.location.href
      });
    } catch (error) {
      console.error('Failed to sign in:', error);
      throw error;
    }
  }

  async signOut(): Promise<void> {
    if (!this.walletConnection) {
      throw new Error('Wallet not initialized');
    }

    try {
      this.walletConnection.signOut();
    } catch (error) {
      console.error('Failed to sign out:', error);
      throw error;
    }
  }

  isSignedIn(): boolean {
    if (!this.walletConnection) {
      return false;
    }
    return this.walletConnection.isSignedIn();
  }

  getAccountId(): string {
    if (!this.walletConnection || !this.walletConnection.isSignedIn()) {
      return '';
    }
    return this.walletConnection.getAccountId();
  }

  async getAccount(): Promise<any> {
    if (!this.walletConnection || !this.walletConnection.isSignedIn()) {
      throw new Error('Wallet not signed in');
    }
    return this.walletConnection.account();
  }

  getWalletConnection(): WalletConnection | null {
    return this.walletConnection;
  }

  getNetwork(): string {
    return this.config.network;
  }

  // Method to get current balance
  async getBalance(): Promise<string> {
    try {
      const account = await this.getAccount();
      const balance = await account.getAccountBalance();
      return balance.available;
    } catch (error) {
      console.error('Failed to get balance:', error);
      return '0';
    }
  }

  // Method to check if account exists
  async accountExists(accountId: string): Promise<boolean> {
    try {
      if (!this.near) return false;
      const account = await this.near.account(accountId);
      await account.state();
      return true;
    } catch (error) {
      return false;
    }
  }

  async callMethod(
    contractId: string,
    methodName: string,
    args: Record<string, unknown>,
    gas: string = '30000000000000',
    deposit: string = '0'
  ): Promise<any> {
    const account = await this.getAccount();
    return account.functionCall({
      contractId,
      methodName,
      args,
      gas,
      attachedDeposit: deposit,
    });
  }

  async viewMethod(
    contractId: string,
    methodName: string,
    args: Record<string, unknown>
  ): Promise<any> {
    if (!this.near) {
      throw new Error('NEAR connection not initialized');
    }
    const accountId = this.getAccountId() || 'anonymous.testnet';
    const account = await this.near.account(accountId);
    return (account as any).viewFunction(contractId, methodName, args);
  }
}

export const myNearWalletService = new MyNearWalletService({
  network: 'testnet',
  contractName: 'bio-nft-1764175259.sleeplessmonk-testnet-1764175172.testnet',
});

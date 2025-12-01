/**
 * @vitest-environment jsdom
 */
import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import AIBlockchainIntegration from '../components/AIBlockchainIntegration';
import { FilecoinStorageClient } from '../utils/filecoin-storage';

// Mock canvas reference
const mockCanvasRef = {
  current: {
    toDataURL: () => 'mock-data-url',
    toBlob: (callback: Function) => {
      callback(new Blob(['mock-image-data'], { type: 'image/png' }));
    }
  }
};

// Mock the blockchain clients
vi.mock('../utils/solana-client', () => ({
  BiometricNFTClient: vi.fn().mockImplementation(() => ({
    initializeNFT: vi.fn().mockResolvedValue({
      transactionSignature: 'mock-signature',
      nftAccount: { toString: () => 'mock-nft-account' }
    })
  }))
}));

vi.mock('../utils/filecoin-storage', () => ({
  FilecoinStorageClient: vi.fn().mockImplementation(() => ({
    storeEmotionalArt: vi.fn().mockResolvedValue('mock-filecoin-cid')
  }))
}));

vi.mock('../utils/polkadot-client', () => ({
  PolkadotSoulboundClient: vi.fn().mockImplementation(() => ({
    createIdentity: vi.fn().mockResolvedValue({
      identityId: 123,
      transactionHash: 'mock-tx-hash'
    }),
    getIdentity: vi.fn().mockResolvedValue({
      identity_id: 123,
      owner: 'mock-owner',
      name: 'mock-name',
      biometric_hash: 'mock-hash',
      emotion_data: { valence: 0.5, arousal: 0.5, dominance: 0.5 },
      metadata_uri: 'mock-uri',
      is_verified: true
    })
  }))
}));

// Mock wallet hooks
vi.mock('@solana/wallet-adapter-react', () => ({
  useWallet: () => ({
    publicKey: { toString: () => 'mock-public-key' },
    signTransaction: vi.fn(),
    connected: true
  }),
  useConnection: () => ({
    connection: 'mock-connection'
  })
}));

describe('AIBlockchainIntegration', () => {
  const mockEmotionalState = {
    valence: 0.7,
    arousal: 0.5,
    dominance: 0.8
  };

  it('renders integration options correctly', () => {
    render(
      <AIBlockchainIntegration
        canvasRef={mockCanvasRef as any}
        emotionalState={mockEmotionalState}
      />
    );

    expect(screen.getByText('AI Blockchain Integration')).toBeTruthy();
    expect(screen.getByText('Filecoin Storage')).toBeTruthy();
    expect(screen.getByText('Solana NFT')).toBeTruthy();
    expect(screen.getByText('Polkadot Identity')).toBeTruthy();
  });

  it('executes full integration when button is clicked', async () => {
    const mockOnComplete = vi.fn();
    
    render(
      <AIBlockchainIntegration
        canvasRef={mockCanvasRef as any}
        emotionalState={mockEmotionalState}
        onIntegrationComplete={mockOnComplete}
      />
    );

    const integrateButton = screen.getByText('Execute Full Blockchain Integration');
    fireEvent.click(integrateButton);

    // Wait for the integration to complete
    await waitFor(() => {
      expect(mockOnComplete).toHaveBeenCalledWith(
        expect.objectContaining({
          filecoinCid: 'mock-filecoin-cid',
          solanaNft: expect.objectContaining({
            signature: 'mock-signature',
            nftAccount: 'mock-nft-account'
          }),
          polkadotIdentity: expect.objectContaining({
            identityId: 123,
            transactionHash: 'mock-tx-hash'
          })
        })
      );
    }, { timeout: 10000 });
  });

  it('handles wallet disconnection gracefully', () => {
    // Mock disconnected wallet
    vi.mock('@solana/wallet-adapter-react', () => ({
      useWallet: () => ({
        publicKey: null,
        signTransaction: null,
        connected: false
      }),
      useConnection: () => ({
        connection: 'mock-connection'
      })
    }));

    render(
      <AIBlockchainIntegration
        canvasRef={mockCanvasRef as any}
        emotionalState={mockEmotionalState}
      />
    );

    expect(screen.getByText('Connect wallet to enable')).toBeTruthy();
  });

  it('shows progress during integration', async () => {
    render(
      <AIBlockchainIntegration
        canvasRef={mockCanvasRef as any}
        emotionalState={mockEmotionalState}
      />
    );

    const integrateButton = screen.getByText('Execute Full Blockchain Integration');
    fireEvent.click(integrateButton);

    // Check that progress is shown
    await waitFor(() => {
      expect(screen.getByText(/Step 1\/3/)).toBeTruthy();
    });
  });

  it('handles errors gracefully', async () => {
    // Mock failed Filecoin upload - reset mock first
    vi.clearAllMocks();
    const mockFilecoinClient = {
      storeEmotionalArt: vi.fn().mockRejectedValue(new Error('Filecoin upload failed'))
    };
    vi.mocked(FilecoinStorageClient).mockReturnValue(mockFilecoinClient as any);

    const mockOnComplete = vi.fn();
    
    render(
      <AIBlockchainIntegration
        canvasRef={mockCanvasRef as any}
        emotionalState={mockEmotionalState}
        onIntegrationComplete={mockOnComplete}
      />
    );

    const integrateButton = screen.getByText('Execute Full Blockchain Integration');
    fireEvent.click(integrateButton);

    await waitFor(() => {
      expect(mockOnComplete).toHaveBeenCalledWith(
        expect.objectContaining({
          errors: expect.arrayContaining([expect.stringContaining('Integration failed')])
        })
      );
    });
  });
});
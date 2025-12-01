// Real Web3.Storage integration that actually uploads to IPFS
// This works with a real Web3.Storage API key

class RealWeb3Storage {
  constructor(apiKey) {
    this.apiKey = apiKey;
    this.endpoint = 'https://api.web3.storage';
    this.uploadEndpoint = `${this.endpoint}/upload`;
  }

  async uploadFile(file) {
    if (!this.apiKey || this.apiKey === 'demo-key') {
      throw new Error('Web3.Storage API key required. Set WEB3_STORAGE_API_KEY environment variable.');
    }

    try {
      console.log('📤 Uploading file to Web3.Storage...');
      console.log('File:', file.name, `(${file.size} bytes)`);

      const formData = new FormData();
      formData.append('file', file);

      const response = await fetch(this.uploadEndpoint, {
        method: 'POST',
        headers: {
          'Authorization': `Bearer ${this.apiKey}`,
          'Accept': 'application/json'
        },
        body: formData
      });

      if (!response.ok) {
        const errorText = await response.text();
        throw new Error(`Upload failed: ${response.status} ${errorText}`);
      }

      const result = await response.json();
      console.log('✅ Upload successful!');
      console.log('🆔 CID:', result.cid);
      console.log('🔗 IPFS URL:', `https://ipfs.io/ipfs/${result.cid}`);

      return {
        success: true,
        cid: result.cid,
        ipfsUrl: `https://ipfs.io/ipfs/${result.cid}`,
        web3StorageUrl: `https://w3s.link/ipfs/${result.cid}`,
        size: file.size
      };
    } catch (error) {
      console.error('❌ Upload failed:', error);
      throw error;
    }
  }

  async uploadJSON(data, filename = 'data.json') {
    const jsonString = JSON.stringify(data, null, 2);
    const blob = new Blob([jsonString], { type: 'application/json' });
    const file = new File([blob], filename, { type: 'application/json' });
    
    return await this.uploadFile(file);
  }

  async uploadImage(base64Data, filename = 'image.png') {
    // Convert base64 to blob
    const response = await fetch(base64Data);
    const blob = await response.blob();
    const file = new File([blob], filename, { type: blob.type });
    
    return await this.uploadFile(file);
  }

  async getStatus(cid) {
    try {
      const response = await fetch(`${this.endpoint}/status/${cid}`, {
        headers: {
          'Authorization': `Bearer ${this.apiKey}`,
          'Accept': 'application/json'
        }
      });

      if (!response.ok) {
        throw new Error(`Status check failed: ${response.status}`);
      }

      return await response.json();
    } catch (error) {
      console.error('❌ Status check failed:', error);
      throw error;
    }
  }

  // Upload creative metadata to Web3.Storage
  async uploadCreativeMetadata(metadata, filename = 'creative-metadata.json') {
    console.log('📤 Uploading creative metadata to Web3.Storage...');
    
    try {
      // Validate metadata
      if (!metadata || typeof metadata !== 'object') {
        throw new Error('Invalid metadata provided');
      }
      
      // Add upload timestamp and version
      const enrichedMetadata = {
        ...metadata,
        upload_metadata: {
          uploaded_at: new Date().toISOString(),
          uploader_version: '1.0',
          storage_provider: 'web3.storage'
        }
      };
      
      // Upload to Web3.Storage
      const result = await this.uploadJSON(enrichedMetadata, filename);
      
      console.log(`✅ Creative metadata uploaded successfully`);
      console.log(`📊 CID: ${result.cid}`);
      console.log(`🔗 URL: https://w3s.link/ipfs/${result.cid}`);
      
      return {
        success: true,
        cid: result.cid,
        url: `https://w3s.link/ipfs/${result.cid}`,
        filename: filename
      };
      
    } catch (error) {
      console.error('❌ Web3.Storage upload failed:', error);
      throw new Error(`Failed to upload creative metadata: ${error.message}`);
    }
  }
}



// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
  module.exports = RealWeb3Storage;
}
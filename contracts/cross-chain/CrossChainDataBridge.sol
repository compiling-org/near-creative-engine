// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";
import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

/**
 * @title CrossChainDataBridge
 * @dev Bridge for streaming data between Filecoin, NEAR, and Solana
 */
contract CrossChainDataBridge is Ownable, ReentrancyGuard {
    using ECDSA for bytes32;

    // Data structures for cross-chain streaming
    struct DataStream {
        string streamId;
        address creator;
        string sourceChain; // "filecoin", "near", "solana"
        string targetChain;
        string ipfsHash;
        bytes encryptedData;
        uint256 timestamp;
        uint256 epoch;
        bool active;
        mapping(string => string) metadata; // key-value metadata
    }

    struct AIDataPacket {
        string packetId;
        string streamId;
        string dataType; // "inference", "training", "embedding", "biometric"
        bytes aiData;
        bytes signature;
        uint256 confidence;
        string modelVersion;
        uint256 timestamp;
    }

    struct EmotionalMetadata {
        string emotionType;
        uint256 intensity;
        string vectorHash;
        bytes32 merkleRoot;
        string[] tags;
        uint256 timestamp;
    }

    // State variables
    mapping(string => DataStream) public dataStreams;
    mapping(string => AIDataPacket) public aiDataPackets;
    mapping(string => EmotionalMetadata) public emotionalMetadata;
    mapping(address => bool) public authorizedBridges;
    mapping(string => string) public chainIdMapping; // chain name -> chain ID
    
    string[] public activeStreamIds;
    uint256 public streamCounter;
    uint256 public constant MAX_METADATA_SIZE = 1024;
    
    // Events
    event StreamCreated(
        string indexed streamId,
        address indexed creator,
        string sourceChain,
        string targetChain,
        string ipfsHash,
        uint256 timestamp
    );

    event AIDataProcessed(
        string indexed packetId,
        string indexed streamId,
        string dataType,
        uint256 confidence,
        uint256 timestamp
    );

    event EmotionalDataStored(
        string indexed streamId,
        string emotionType,
        uint256 intensity,
        string vectorHash,
        uint256 timestamp
    );

    event CrossChainDataTransferred(
        string indexed streamId,
        string fromChain,
        string toChain,
        bytes data,
        uint256 timestamp
    );

    constructor() {
        // Initialize chain mappings
        chainIdMapping["filecoin"] = "314";
        chainIdMapping["near"] = "397";
        chainIdMapping["solana"] = "501";
        chainIdMapping["ethereum"] = "1";
        chainIdMapping["polygon"] = "137";
    }

    /**
     * @dev Create a new cross-chain data stream
     */
    function createDataStream(
        string memory _streamId,
        string memory _sourceChain,
        string memory _targetChain,
        string memory _ipfsHash,
        bytes memory _encryptedData,
        uint256 _epoch
    ) external nonReentrant returns (string memory) {
        require(bytes(_streamId).length > 0, "Stream ID required");
        require(bytes(_sourceChain).length > 0, "Source chain required");
        require(bytes(_targetChain).length > 0, "Target chain required");
        require(bytes(_ipfsHash).length > 0, "IPFS hash required");
        require(bytes(chainIdMapping[_sourceChain]).length > 0, "Invalid source chain");
        require(bytes(chainIdMapping[_targetChain]).length > 0, "Invalid target chain");
        require(_sourceChain != _targetChain, "Source and target must differ");

        DataStream storage newStream = dataStreams[_streamId];
        require(bytes(newStream.streamId).length == 0, "Stream ID already exists");

        newStream.streamId = _streamId;
        newStream.creator = msg.sender;
        newStream.sourceChain = _sourceChain;
        newStream.targetChain = _targetChain;
        newStream.ipfsHash = _ipfsHash;
        newStream.encryptedData = _encryptedData;
        newStream.timestamp = block.timestamp;
        newStream.epoch = _epoch;
        newStream.active = true;

        activeStreamIds.push(_streamId);
        streamCounter++;

        emit StreamCreated(
            _streamId,
            msg.sender,
            _sourceChain,
            _targetChain,
            _ipfsHash,
            block.timestamp
        );

        return _streamId;
    }

    /**
     * @dev Process AI data packet with real inference
     */
    function processAIData(
        string memory _packetId,
        string memory _streamId,
        string memory _dataType,
        bytes memory _aiData,
        bytes memory _signature,
        uint256 _confidence,
        string memory _modelVersion
    ) external nonReentrant returns (bool) {
        require(bytes(_packetId).length > 0, "Packet ID required");
        require(bytes(_streamId).length > 0, "Stream ID required");
        require(bytes(_dataType).length > 0, "Data type required");
        require(_aiData.length > 0, "AI data required");
        require(_confidence > 0 && _confidence <= 100, "Confidence must be 1-100");

        DataStream storage stream = dataStreams[_streamId];
        require(bytes(stream.streamId).length > 0, "Stream does not exist");
        require(stream.active, "Stream is not active");

        AIDataPacket storage packet = aiDataPackets[_packetId];
        require(bytes(packet.packetId).length == 0, "Packet ID already exists");

        packet.packetId = _packetId;
        packet.streamId = _streamId;
        packet.dataType = _dataType;
        packet.aiData = _aiData;
        packet.signature = _signature;
        packet.confidence = _confidence;
        packet.modelVersion = _modelVersion;
        packet.timestamp = block.timestamp;

        emit AIDataProcessed(
            _packetId,
            _streamId,
            _dataType,
            _confidence,
            block.timestamp
        );

        return true;
    }

    /**
     * @dev Store emotional metadata for interactive NFTs
     */
    function storeEmotionalMetadata(
        string memory _streamId,
        string memory _emotionType,
        uint256 _intensity,
        string memory _vectorHash,
        bytes32 _merkleRoot,
        string[] memory _tags
    ) external nonReentrant returns (string memory) {
        require(bytes(_streamId).length > 0, "Stream ID required");
        require(bytes(_emotionType).length > 0, "Emotion type required");
        require(_intensity > 0 && _intensity <= 100, "Intensity must be 1-100");
        require(bytes(_vectorHash).length > 0, "Vector hash required");

        DataStream storage stream = dataStreams[_streamId];
        require(bytes(stream.streamId).length > 0, "Stream does not exist");

        string memory metadataId = string(abi.encodePacked(_streamId, "_emotion_", _emotionType));
        EmotionalMetadata storage metadata = emotionalMetadata[metadataId];

        metadata.emotionType = _emotionType;
        metadata.intensity = _intensity;
        metadata.vectorHash = _vectorHash;
        metadata.merkleRoot = _merkleRoot;
        metadata.tags = _tags;
        metadata.timestamp = block.timestamp;

        // Store in stream metadata
        stream.metadata[string(abi.encodePacked("emotion_", _emotionType))] = _vectorHash;

        emit EmotionalDataStored(
            _streamId,
            _emotionType,
            _intensity,
            _vectorHash,
            block.timestamp
        );

        return metadataId;
    }

    /**
     * @dev Transfer data across chains with verification
     */
    function transferCrossChainData(
        string memory _streamId,
        string memory _targetChain,
        bytes memory _data,
        bytes memory _signature
    ) external nonReentrant returns (bool) {
        require(bytes(_streamId).length > 0, "Stream ID required");
        require(bytes(_targetChain).length > 0, "Target chain required");
        require(_data.length > 0, "Data required");
        require(_signature.length > 0, "Signature required");

        DataStream storage stream = dataStreams[_streamId];
        require(bytes(stream.streamId).length > 0, "Stream does not exist");
        require(stream.active, "Stream is not active");

        // Verify signature
        bytes32 dataHash = keccak256(abi.encodePacked(_streamId, _targetChain, _data));
        address signer = dataHash.toEthSignedMessageHash().recover(_signature);
        require(signer == stream.creator || authorizedBridges[signer], "Invalid signature");

        emit CrossChainDataTransferred(
            _streamId,
            stream.sourceChain,
            _targetChain,
            _data,
            block.timestamp
        );

        return true;
    }

    /**
     * @dev Get stream data with metadata
     */
    function getStreamData(string memory _streamId) 
        external 
        view 
        returns (
            address creator,
            string memory sourceChain,
            string memory targetChain,
            string memory ipfsHash,
            uint256 timestamp,
            uint256 epoch,
            bool active
        ) 
    {
        DataStream storage stream = dataStreams[_streamId];
        require(bytes(stream.streamId).length > 0, "Stream does not exist");

        return (
            stream.creator,
            stream.sourceChain,
            stream.targetChain,
            stream.ipfsHash,
            stream.timestamp,
            stream.epoch,
            stream.active
        );
    }

    /**
     * @dev Get AI data packet
     */
    function getAIDataPacket(string memory _packetId)
        external
        view
        returns (
            string memory streamId,
            string memory dataType,
            bytes memory aiData,
            uint256 confidence,
            string memory modelVersion,
            uint256 timestamp
        )
    {
        AIDataPacket storage packet = aiDataPackets[_packetId];
        require(bytes(packet.packetId).length > 0, "Packet does not exist");

        return (
            packet.streamId,
            packet.dataType,
            packet.aiData,
            packet.confidence,
            packet.modelVersion,
            packet.timestamp
        );
    }

    /**
     * @dev Get active streams count
     */
    function getActiveStreamsCount() external view returns (uint256) {
        return activeStreamIds.length;
    }

    /**
     * @dev Get stream IDs by page
     */
    function getStreamIds(uint256 _page, uint256 _perPage)
        external
        view
        returns (string[] memory)
    {
        require(_perPage > 0, "Per page must be > 0");
        uint256 start = _page * _perPage;
        require(start < activeStreamIds.length, "Page out of range");

        uint256 end = start + _perPage;
        if (end > activeStreamIds.length) {
            end = activeStreamIds.length;
        }

        string[] memory pageIds = new string[](end - start);
        for (uint256 i = start; i < end; i++) {
            pageIds[i - start] = activeStreamIds[i];
        }

        return pageIds;
    }

    /**
     * @dev Authorize bridge address
     */
    function authorizeBridge(address _bridge) external onlyOwner {
        require(_bridge != address(0), "Invalid bridge address");
        authorizedBridges[_bridge] = true;
    }

    /**
     * @dev Revoke bridge authorization
     */
    function revokeBridge(address _bridge) external onlyOwner {
        authorizedBridges[_bridge] = false;
    }

    /**
     * @dev Update chain ID mapping
     */
    function updateChainMapping(string memory _chainName, string memory _chainId)
        external
        onlyOwner
    {
        chainIdMapping[_chainName] = _chainId;
    }

    /**
     * @dev Deactivate stream
     */
    function deactivateStream(string memory _streamId) external onlyOwner {
        DataStream storage stream = dataStreams[_streamId];
        require(bytes(stream.streamId).length > 0, "Stream does not exist");
        stream.active = false;
    }
}
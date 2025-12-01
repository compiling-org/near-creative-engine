//! MODURUST Storage - IPFS integration for modular tool assets
//! 
//! Handles storage of modular tools, patches, and configurations

use crate::ipfs_client::IpfsClient;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// MODURUST tool module
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModurustTool {
    pub tool_id: String,
    pub name: String,
    pub version: String,
    pub creator: String,
    pub created_at: u64,
    pub tool_type: ToolType,
    pub module_assets: Vec<ModuleAsset>,
    pub dependencies: Vec<String>,
    pub configuration: ToolConfiguration,
}

/// Type of MODURUST tool
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ToolType {
    ShaderModule,
    AudioProcessor,
    VisualEffect,
    DataTransform,
    ControlInterface,
    CustomModule,
}

/// Asset within a tool module
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModuleAsset {
    pub asset_name: String,
    pub asset_type: AssetType,
    pub cid: String,
    pub size_bytes: u64,
    pub checksum: String,
}

/// Type of module asset
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AssetType {
    WasmBinary,
    ShaderCode,
    ConfigFile,
    PresetData,
    Documentation,
    Icon,
}

/// Tool configuration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolConfiguration {
    pub parameters: Vec<ToolParameter>,
    pub inputs: Vec<IOPort>,
    pub outputs: Vec<IOPort>,
    pub metadata: ToolMetadata,
}

/// Tool parameter definition
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub default_value: String,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub description: String,
}

/// Parameter type
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ParameterType {
    Float,
    Integer,
    Boolean,
    String,
    Color,
    Enum(Vec<String>),
}

/// Input/Output port definition
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct IOPort {
    pub name: String,
    pub data_type: DataType,
    pub description: String,
}

/// Data type for ports
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum DataType {
    Audio,
    Video,
    Image,
    Numerical,
    Emotional,
    Generic,
}

/// Tool metadata
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ToolMetadata {
    pub description: String,
    pub tags: Vec<String>,
    pub license: String,
    pub homepage: Option<String>,
    pub github_url: Option<String>,
}

/// MODURUST patch - collection of connected tools
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ModurustPatch {
    pub patch_id: String,
    pub name: String,
    pub creator: String,
    pub created_at: u64,
    pub tools: Vec<String>, // Tool IDs
    pub connections: Vec<Connection>,
    pub parameter_states: Vec<ParameterState>,
}

/// Connection between tools
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Connection {
    pub from_tool: String,
    pub from_output: String,
    pub to_tool: String,
    pub to_input: String,
}

/// Current state of a parameter
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ParameterState {
    pub tool_id: String,
    pub parameter_name: String,
    pub current_value: String,
}

impl ModurustTool {
    /// Create a new tool
    pub fn new(tool_id: String, name: String, version: String, creator: String, tool_type: ToolType) -> Self {
        Self {
            tool_id,
            name,
            version,
            creator,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            tool_type,
            module_assets: Vec::new(),
            dependencies: Vec::new(),
            configuration: ToolConfiguration {
                parameters: Vec::new(),
                inputs: Vec::new(),
                outputs: Vec::new(),
                metadata: ToolMetadata {
                    description: String::new(),
                    tags: Vec::new(),
                    license: "MIT".to_string(),
                    homepage: None,
                    github_url: None,
                },
            },
        }
    }

    /// Add an asset to the tool
    pub fn add_asset(&mut self, asset: ModuleAsset) {
        self.module_assets.push(asset);
    }

    /// Add a parameter
    pub fn add_parameter(&mut self, param: ToolParameter) {
        self.configuration.parameters.push(param);
    }

    /// Add an input port
    pub fn add_input(&mut self, input: IOPort) {
        self.configuration.inputs.push(input);
    }

    /// Add an output port
    pub fn add_output(&mut self, output: IOPort) {
        self.configuration.outputs.push(output);
    }

    /// Store tool to IPFS
    pub async fn store_to_ipfs(&self, client: &IpfsClient) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        client.add_json(&json).await
    }

    /// Get total asset size
    pub fn total_asset_size(&self) -> u64 {
        self.module_assets.iter().map(|a| a.size_bytes).sum()
    }
}

impl ModurustPatch {
    /// Create a new patch
    pub fn new(patch_id: String, name: String, creator: String) -> Self {
        Self {
            patch_id,
            name,
            creator,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            tools: Vec::new(),
            connections: Vec::new(),
            parameter_states: Vec::new(),
        }
    }

    /// Add a tool to the patch
    pub fn add_tool(&mut self, tool_id: String) {
        self.tools.push(tool_id);
    }

    /// Add a connection between tools
    pub fn add_connection(&mut self, connection: Connection) {
        self.connections.push(connection);
    }

    /// Set parameter state
    pub fn set_parameter(&mut self, tool_id: String, param_name: String, value: String) {
        // Remove existing state if present
        self.parameter_states.retain(|p| !(p.tool_id == tool_id && p.parameter_name == param_name));
        
        // Add new state
        self.parameter_states.push(ParameterState {
            tool_id,
            parameter_name: param_name,
            current_value: value,
        });
    }

    /// Store patch to IPFS
    pub async fn store_to_ipfs(&self, client: &IpfsClient) -> Result<String, Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        client.add_json(&json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modurust_tool_creation() {
        let tool = ModurustTool::new(
            "tool_001".to_string(),
            "Fractal Generator".to_string(),
            "1.0.0".to_string(),
            "creator.near".to_string(),
            ToolType::ShaderModule,
        );

        assert_eq!(tool.tool_id, "tool_001");
        assert_eq!(tool.name, "Fractal Generator");
    }

    #[test]
    fn test_add_parameter() {
        let mut tool = ModurustTool::new(
            "tool_001".to_string(),
            "Test Tool".to_string(),
            "1.0.0".to_string(),
            "creator".to_string(),
            ToolType::CustomModule,
        );

        tool.add_parameter(ToolParameter {
            name: "zoom".to_string(),
            param_type: ParameterType::Float,
            default_value: "1.0".to_string(),
            min_value: Some(0.1),
            max_value: Some(10.0),
            description: "Zoom level".to_string(),
        });

        assert_eq!(tool.configuration.parameters.len(), 1);
    }

    #[test]
    fn test_patch_connections() {
        let mut patch = ModurustPatch::new(
            "patch_001".to_string(),
            "Test Patch".to_string(),
            "creator".to_string(),
        );

        patch.add_tool("tool_a".to_string());
        patch.add_tool("tool_b".to_string());

        patch.add_connection(Connection {
            from_tool: "tool_a".to_string(),
            from_output: "output1".to_string(),
            to_tool: "tool_b".to_string(),
            to_input: "input1".to_string(),
        });

        assert_eq!(patch.tools.len(), 2);
        assert_eq!(patch.connections.len(), 1);
    }
}

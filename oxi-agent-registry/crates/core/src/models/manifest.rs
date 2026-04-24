use super::RegistryRef;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AgentManifest represents the agent project configuration and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentManifest {
    #[serde(rename = "agentName")]
    pub name: String,
    pub image: String,
    pub language: String,
    pub framework: String,
    pub model_provider: String,
    pub model_name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telemetry_endpoint: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp_servers: Vec<McpServerType>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<SkillRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prompts: Vec<PromptRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl AgentManifest {
    /// ExtractMCPServerRefs extracts RegistryRef entries from the manifest's McpServers list.
    /// Only new-format entries (no Type or RegistryServerName set) are included.
    pub fn extract_mcp_server_refs(&self) -> Vec<RegistryRef> {
        self.mcp_servers
            .iter()
            .filter_map(|s| s.to_registry_ref())
            .collect()
    }
}

/// SkillRef represents a skill reference in the agent manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRef {
    /// Name is the local name for the skill in this agent project.
    pub name: String,
    /// Image is a Docker image containing the skill (for image type).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    /// RegistryURL is the registry URL for pulling the skill (for registry type).
    #[serde(rename = "registryURL", skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    /// RegistrySkillName is the skill name in the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_skill_name: Option<String>,
    /// RegistrySkillVersion is the version of the skill to pull.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_skill_version: Option<String>,
}

/// PromptRef represents a prompt reference in the agent manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRef {
    /// Name is the local name for the prompt in this agent project.
    pub name: String,
    /// RegistryURL is the registry URL for pulling the prompt (for registry type).
    #[serde(rename = "registryURL", skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    /// RegistryPromptName is the prompt name in the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_prompt_name: Option<String>,
    /// RegistryPromptVersion is the version of the prompt to pull.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_prompt_version: Option<String>,
}

/// McpServerType represents a single MCP server configuration.
/// New declarative format: only Name + Version are set (Name is the registry server name).
/// Legacy format: Type is set ("remote", "command", or "registry") with type-specific fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct McpServerType {
    /// Name is the registry server name (new format) or local display name (legacy format).
    pub name: String,
    /// Version is the server version for the new declarative format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    // TODO(legacy): remove fields below once declarative API is the only supported path
    /// Type is the MCP server type -- remote, command, registry (legacy format only).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub headers: HashMap<String, String>,
    // Registry MCP server fields
    #[serde(rename = "registryURL", skip_serializing_if = "Option::is_none")]
    pub registry_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_server_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registry_server_version: Option<String>,
    #[serde(default)]
    pub registry_server_prefer_remote: bool,
}

impl McpServerType {
    /// IsLegacyFormat returns true if this MCP server entry uses the legacy format
    /// (Type or RegistryServerName set), false if it uses the new RegistryRef format.
    pub fn is_legacy_format(&self) -> bool {
        self.server_type.is_some() || self.registry_server_name.is_some()
    }

    /// ToRegistryRef converts a new-format McpServerType entry to a RegistryRef.
    /// Returns None if this is a legacy-format entry.
    pub fn to_registry_ref(&self) -> Option<RegistryRef> {
        if self.is_legacy_format() {
            return None;
        }
        Some(RegistryRef {
            name: self.name.clone(),
            version: self.version.clone(),
        })
    }
}

pub mod agent;
pub mod server;
pub mod skill;
pub mod prompt;
pub mod deployment;
pub mod provider;
pub mod manifest;

// Re-export specific types to avoid ambiguity
pub use agent::{
    AgentJSON, AgentListResponse, AgentMetadata, AgentPackageInfo, AgentRegistryExtensions,
    AgentResponse, AgentResponseMeta, AgentSemanticMeta, Transport, TransportType,
};
pub use deployment::*;
pub use manifest::*;
pub use prompt::*;
pub use provider::*;
pub use server::{
    PackageInfo, RegistryExtensions, RemoteInfo, ServerJSON, ServerListResponse, ServerMetadata,
    ServerResponse, ServerResponseMeta, ServerSemanticMeta, TransportInfo,
};
pub use skill::*;

// Note: agent::Repository and server::Repository are intentionally not re-exported at the top level
// to avoid ambiguity. Use the module path if you need them.

use serde::{Deserialize, Serialize};

/// RegistryRef is the unified reference type for all agent dependencies
/// (MCP servers, skills, prompts) in the declarative agent spec.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RegistryRef {
    /// registry resource name (e.g. "myorg/weather-mcp")
    pub name: String,
    /// version; empty = resolve to latest
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl RegistryRef {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }

    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
}

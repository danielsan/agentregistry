use super::{deployment::ResourceDeploymentsMeta, manifest::AgentManifest, RegistryRef};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// AgentJSON mirrors the ServerJSON shape for now, defined locally
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentJSON {
    #[serde(flatten)]
    pub manifest: AgentManifest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<AgentPackageInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remotes: Vec<Transport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPackageInfo {
    pub registry_type: String,
    pub identifier: String,
    pub version: String,
    pub transport: TransportType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportType {
    #[serde(rename = "type")]
    pub transport_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    #[serde(rename = "type")]
    pub transport_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// AgentRegistryExtensions mirrors official metadata stored separately
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRegistryExtensions {
    pub status: String,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_latest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSemanticMeta {
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentResponseMeta {
    #[serde(
        rename = "io.modelcontextprotocol.registry/official",
        skip_serializing_if = "Option::is_none"
    )]
    pub official: Option<AgentRegistryExtensions>,
    #[serde(rename = "aregistry.ai/semantic", skip_serializing_if = "Option::is_none")]
    pub semantic: Option<AgentSemanticMeta>,
    #[serde(
        rename = "aregistry.ai/deployments",
        skip_serializing_if = "Option::is_none"
    )]
    pub deployments: Option<ResourceDeploymentsMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentResponse {
    pub agent: AgentJSON,
    #[serde(rename = "_meta")]
    pub meta: AgentResponseMeta,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub mcp_server_refs: Vec<RegistryRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skill_refs: Vec<RegistryRef>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prompt_refs: Vec<RegistryRef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentListResponse {
    pub agents: Vec<AgentResponse>,
    pub metadata: AgentMetadata,
}

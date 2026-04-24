use super::deployment::ResourceDeploymentsMeta;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ServerSemanticMeta carries semantic search metadata for servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSemanticMeta {
    pub score: f64,
}

/// ServerResponseMeta mirrors the MCP ResponseMeta but adds semantic metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerResponseMeta {
    #[serde(
        rename = "io.modelcontextprotocol.registry/official",
        skip_serializing_if = "Option::is_none"
    )]
    pub official: Option<RegistryExtensions>,
    #[serde(rename = "aregistry.ai/semantic", skip_serializing_if = "Option::is_none")]
    pub semantic: Option<ServerSemanticMeta>,
    #[serde(
        rename = "aregistry.ai/deployments",
        skip_serializing_if = "Option::is_none"
    )]
    pub deployments: Option<ResourceDeploymentsMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryExtensions {
    pub status: String,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_latest: bool,
}

/// ServerResponse is the server API shape with registry-managed metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerResponse {
    pub server: ServerJSON,
    #[serde(rename = "_meta")]
    pub meta: ServerResponseMeta,
}

/// ServerJSON represents an MCP server in the registry.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerJSON {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub description: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<PackageInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remotes: Vec<RemoteInfo>,
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
pub struct PackageInfo {
    pub registry_type: String,
    pub identifier: String,
    pub version: String,
    pub transport: TransportInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportInfo {
    #[serde(rename = "type")]
    pub transport_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RemoteInfo {
    pub url: String,
}

/// ServerMetadata holds pagination info for server listings.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub count: usize,
}

/// ServerListResponse wraps a list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerListResponse {
    pub servers: Vec<ServerResponse>,
    pub metadata: ServerMetadata,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Deployment status values used across registry workflows and API payloads.
pub const DEPLOYMENT_STATUS_DEPLOYING: &str = "deploying";
pub const DEPLOYMENT_STATUS_DEPLOYED: &str = "deployed";
pub const DEPLOYMENT_STATUS_FAILED: &str = "failed";
pub const DEPLOYMENT_STATUS_CANCELLED: &str = "cancelled";
pub const DEPLOYMENT_STATUS_DISCOVERED: &str = "discovered";

/// Deployment represents a deployed resource with unified deployment metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Deployment {
    pub id: String,
    /// deployed resource name
    pub server_name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    pub resource_type: String,
    /// deploying, deployed, failed, cancelled, discovered
    pub status: String,
    /// managed, discovered
    pub origin: String,
    #[serde(default)]
    pub env: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_config: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<JsonValue>,
    #[serde(default)]
    pub prefer_remote: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub deployed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// DeploymentActionResult captures provider-specific execution outcome from adapters.
/// The registry service owns persistence and applies this result to deployment rows.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentActionResult {
    /// Status should be a terminal or in-flight deployment status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Error contains provider-specific failure details, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ProviderConfig stores provider-specific effective config to persist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_config: Option<JsonValue>,
    /// ProviderMetadata stores provider-specific runtime metadata to persist
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_metadata: Option<JsonValue>,
}

/// DeploymentStatePatch describes partial deployment state updates.
#[derive(Debug, Clone, Default)]
pub struct DeploymentStatePatch {
    pub status: Option<String>,
    pub error: Option<String>,
    pub provider_config: Option<JsonValue>,
    pub provider_metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KubernetesProviderMetadata {
    pub is_external: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// DeploymentFilter defines filtering options for deployment queries
#[derive(Debug, Clone, Default)]
pub struct DeploymentFilter {
    /// local, kubernetes
    pub platform: Option<String>,
    pub provider_id: Option<String>,
    /// mcp or agent
    pub resource_type: Option<String>,
    pub status: Option<String>,
    pub origin: Option<String>,
    /// case-insensitive substring filter
    pub resource_name: Option<String>,
}

/// DeploymentSummary is a compact deployment view embedded in catalog metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeploymentSummary {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<String>,
    pub status: String,
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub deployed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// ResourceDeploymentsMeta is the `_meta["aregistry.ai/deployments"]` payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourceDeploymentsMeta {
    pub deployments: Vec<DeploymentSummary>,
    pub count: usize,
}

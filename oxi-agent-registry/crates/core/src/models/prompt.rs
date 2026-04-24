use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// PromptJSON represents a prompt in the registry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptJSON {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    pub description: String,
    pub version: String,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptRegistryExtensions {
    pub status: String,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_latest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptResponseMeta {
    #[serde(
        rename = "io.modelcontextprotocol.registry/official",
        skip_serializing_if = "Option::is_none"
    )]
    pub official: Option<PromptRegistryExtensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptResponse {
    pub prompt: PromptJSON,
    #[serde(rename = "_meta")]
    pub meta: PromptResponseMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PromptListResponse {
    pub prompts: Vec<PromptResponse>,
    pub metadata: PromptMetadata,
}

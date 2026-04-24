use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// SkillJSON mirrors the ServerJSON shape for now, defined locally
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillJSON {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    pub description: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<SkillRepository>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub packages: Vec<SkillPackageInfo>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remotes: Vec<SkillRemoteInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRepository {
    pub url: String,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillPackageInfo {
    pub registry_type: String,
    pub identifier: String,
    pub version: String,
    pub transport: SkillTransportInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillTransportInfo {
    #[serde(rename = "type")]
    pub transport_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRemoteInfo {
    pub url: String,
}

/// SkillRegistryExtensions mirrors official metadata stored separately
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillRegistryExtensions {
    pub status: String,
    pub published_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_latest: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillResponseMeta {
    #[serde(
        rename = "io.modelcontextprotocol.registry/official",
        skip_serializing_if = "Option::is_none"
    )]
    pub official: Option<SkillRegistryExtensions>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillResponse {
    pub skill: SkillJSON,
    #[serde(rename = "_meta")]
    pub meta: SkillResponseMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillListResponse {
    pub skills: Vec<SkillResponse>,
    pub metadata: SkillMetadata,
}

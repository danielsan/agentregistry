use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// ServerFilter defines filtering options for server queries
#[derive(Debug, Clone, Default)]
pub struct ServerFilter {
    pub name: Option<String>,
    pub remote_url: Option<String>,
    pub updated_since: Option<DateTime<Utc>>,
    pub substring_name: Option<String>,
    pub version: Option<String>,
    pub is_latest: Option<bool>,
    pub semantic: Option<SemanticSearchOptions>,
}

/// ServerReadme represents a stored README blob for a server version
#[derive(Debug, Clone)]
pub struct ServerReadme {
    pub server_name: String,
    pub version: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub size_bytes: i32,
    pub sha256: Vec<u8>,
    pub fetched_at: DateTime<Utc>,
}

/// SkillFilter defines filtering options for skill queries
#[derive(Debug, Clone, Default)]
pub struct SkillFilter {
    pub name: Option<String>,
    pub remote_url: Option<String>,
    pub updated_since: Option<DateTime<Utc>>,
    pub substring_name: Option<String>,
    pub version: Option<String>,
    pub is_latest: Option<bool>,
    pub semantic: Option<SemanticSearchOptions>,
}

/// AgentFilter defines filtering options for agent queries
#[derive(Debug, Clone, Default)]
pub struct AgentFilter {
    pub name: Option<String>,
    pub remote_url: Option<String>,
    pub updated_since: Option<DateTime<Utc>>,
    pub substring_name: Option<String>,
    pub version: Option<String>,
    pub is_latest: Option<bool>,
    pub semantic: Option<SemanticSearchOptions>,
}

/// PromptFilter defines filtering options for prompt queries
#[derive(Debug, Clone, Default)]
pub struct PromptFilter {
    pub name: Option<String>,
    pub updated_since: Option<DateTime<Utc>>,
    pub substring_name: Option<String>,
    pub version: Option<String>,
    pub is_latest: Option<bool>,
}

/// SemanticEmbedding captures data stored alongside registry resources for semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEmbedding {
    pub vector: Vec<f32>,
    pub provider: String,
    pub model: String,
    pub dimensions: i32,
    pub checksum: String,
    pub generated: DateTime<Utc>,
}

/// SemanticEmbeddingMetadata captures stored metadata about an embedding without the vector payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticEmbeddingMetadata {
    pub has_embedding: bool,
    pub provider: String,
    pub model: String,
    pub dimensions: i32,
    pub checksum: String,
    pub generated: DateTime<Utc>,
}

/// SemanticSearchOptions drives vector similarity queries when listing resources
#[derive(Debug, Clone)]
pub struct SemanticSearchOptions {
    /// RawQuery retains the original search string for embedding generation (service layer use only)
    pub raw_query: String,
    /// QueryEmbedding holds the vector representation expected by the database layer
    pub query_embedding: Vec<f32>,
    /// Threshold filters out matches whose distance exceeds this value
    pub threshold: f64,
    /// HybridSubstring preserves substring conditions for hybrid search
    pub hybrid_substring: Option<String>,
}

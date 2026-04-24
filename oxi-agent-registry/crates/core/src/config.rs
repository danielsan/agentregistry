use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Main configuration for the AgentRegistry application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Server address to bind to (e.g., ":8080" or "0.0.0.0:8080")
    #[serde(default = "default_server_address")]
    pub server_address: String,

    /// MCP port (0 = disabled)
    #[serde(default)]
    pub mcp_port: u16,

    /// Database connection URL
    #[serde(default = "default_database_url")]
    pub database_url: String,

    /// Enable vector search with pgvector
    #[serde(default)]
    pub database_vector_enabled: bool,

    /// Seed registry from external source
    #[serde(default)]
    pub seed_from: Option<String>,

    /// Enable enrichment of server data
    #[serde(default)]
    pub enrich_server_data: bool,

    /// Disable built-in seed data
    #[serde(default = "default_true")]
    pub disable_builtin_seed: bool,

    /// Application version
    #[serde(default = "default_version")]
    pub version: String,

    /// JWT private key for signing tokens
    #[serde(default)]
    pub jwt_private_key: Option<String>,

    /// Enable registry validation
    #[serde(default = "default_true")]
    pub enable_registry_validation: bool,

    /// Log level (trace, debug, info, warn, error)
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Platform mode: "docker" or "kubernetes"
    #[serde(default = "default_platform_mode")]
    pub platform_mode: String,

    /// Agent Gateway port
    #[serde(default = "default_agent_gateway_port")]
    pub agent_gateway_port: u16,

    /// Runtime directory for temporary files
    #[serde(default = "default_runtime_dir")]
    pub runtime_dir: PathBuf,

    /// Verbose logging
    #[serde(default)]
    pub verbose: bool,

    /// Embeddings configuration
    #[serde(default)]
    pub embeddings: EmbeddingsConfig,
}

/// Configuration for embeddings and semantic search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingsConfig {
    /// Enable embeddings generation
    #[serde(default)]
    pub enabled: bool,

    /// Provider (e.g., "openai")
    #[serde(default = "default_embeddings_provider")]
    pub provider: String,

    /// Model name
    #[serde(default = "default_embeddings_model")]
    pub model: String,

    /// Embedding dimensions
    #[serde(default = "default_embeddings_dimensions")]
    pub dimensions: usize,

    /// OpenAI API key
    #[serde(default)]
    pub openai_api_key: Option<String>,

    /// OpenAI base URL
    #[serde(default = "default_openai_base_url")]
    pub openai_base_url: String,

    /// OpenAI organization
    #[serde(default)]
    pub openai_org: Option<String>,

    /// Generate embeddings on publish
    #[serde(default)]
    pub on_publish: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server_address: default_server_address(),
            mcp_port: 0,
            database_url: default_database_url(),
            database_vector_enabled: false,
            seed_from: None,
            enrich_server_data: false,
            disable_builtin_seed: true,
            version: default_version(),
            jwt_private_key: None,
            enable_registry_validation: true,
            log_level: default_log_level(),
            platform_mode: default_platform_mode(),
            agent_gateway_port: default_agent_gateway_port(),
            runtime_dir: default_runtime_dir(),
            verbose: false,
            embeddings: EmbeddingsConfig::default(),
        }
    }
}

impl Default for EmbeddingsConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            provider: default_embeddings_provider(),
            model: default_embeddings_model(),
            dimensions: default_embeddings_dimensions(),
            openai_api_key: None,
            openai_base_url: default_openai_base_url(),
            openai_org: None,
            on_publish: false,
        }
    }
}

impl Config {
    /// Load configuration from environment variables with AGENT_REGISTRY_ prefix
    pub fn from_env() -> Result<Self, crate::Error> {
        // Try to load .env file (ignore errors if it doesn't exist)
        let _ = dotenvy::dotenv();

        let config = config::Config::builder()
            .add_source(
                config::Environment::with_prefix("AGENT_REGISTRY")
                    .separator("_")
                    .try_parsing(true),
            )
            .build()
            .map_err(|e| crate::Error::InvalidInput(format!("Failed to build config: {}", e)))?;

        let mut cfg: Config = config
            .try_deserialize()
            .map_err(|e| crate::Error::InvalidInput(format!("Failed to deserialize config: {}", e)))?;

        // If runtime_dir wasn't explicitly set, append a random suffix
        if std::env::var("AGENT_REGISTRY_RUNTIME_DIR").is_err() {
            let suffix = generate_random_hex(8)?;
            let mut dir = cfg.runtime_dir.into_os_string();
            dir.push(format!("-{}", suffix));
            cfg.runtime_dir = PathBuf::from(dir);
        }

        Ok(cfg)
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.database_url.is_empty() {
            return Err(crate::Error::InvalidInput(
                "DATABASE_URL is required".to_string(),
            ));
        }

        if self.embeddings.enabled && self.embeddings.openai_api_key.is_none() {
            return Err(crate::Error::InvalidInput(
                "OPENAI_API_KEY is required when embeddings are enabled".to_string(),
            ));
        }

        if !matches!(self.platform_mode.as_str(), "docker" | "kubernetes") {
            return Err(crate::Error::InvalidInput(format!(
                "Invalid platform_mode: {}. Must be 'docker' or 'kubernetes'",
                self.platform_mode
            )));
        }

        Ok(())
    }
}

// Default value functions
fn default_server_address() -> String {
    ":8080".to_string()
}

fn default_database_url() -> String {
    "postgres://agentregistry:agentregistry@localhost:5432/agentregistry?sslmode=disable".to_string()
}

fn default_true() -> bool {
    true
}

fn default_version() -> String {
    "dev".to_string()
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_platform_mode() -> String {
    "kubernetes".to_string()
}

fn default_agent_gateway_port() -> u16 {
    8081
}

fn default_runtime_dir() -> PathBuf {
    PathBuf::from("/tmp/arctl-runtime")
}

fn default_embeddings_provider() -> String {
    "openai".to_string()
}

fn default_embeddings_model() -> String {
    "text-embedding-3-small".to_string()
}

fn default_embeddings_dimensions() -> usize {
    1536
}

fn default_openai_base_url() -> String {
    "https://api.openai.com/v1".to_string()
}

/// Generate a random hex string of n bytes
fn generate_random_hex(n: usize) -> Result<String, crate::Error> {
    use rand::Rng;
    let bytes: Vec<u8> = (0..n).map(|_| rand::thread_rng().gen()).collect();
    Ok(hex::encode(&bytes))
}

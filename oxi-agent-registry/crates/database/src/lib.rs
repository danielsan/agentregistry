pub mod error;
pub mod postgres;
pub mod migration;
pub mod types;

pub use error::{Error, Result};
pub use postgres::PostgreSQL;
pub use types::*;

use async_trait::async_trait;
use oxi_core::models::*;

/// Common database errors as sentinel values
pub const ERR_NOT_FOUND: &str = "record not found";
pub const ERR_FORBIDDEN: &str = "forbidden";
pub const ERR_ALREADY_EXISTS: &str = "record already exists";
pub const ERR_INVALID_INPUT: &str = "invalid input";
pub const ERR_INVALID_VERSION: &str = "invalid version: cannot publish duplicate version";
pub const ERR_MAX_VERSIONS_REACHED: &str = "maximum number of versions reached (10000): please reach out at https://github.com/modelcontextprotocol/registry to explain your use case";

/// Database transaction scope providing access to all stores
#[async_trait]
pub trait Scope: Send + Sync {
    fn servers(&self) -> &dyn ServerStore;
    fn providers(&self) -> &dyn ProviderStore;
    fn agents(&self) -> &dyn AgentStore;
    fn skills(&self) -> &dyn SkillStore;
    fn prompts(&self) -> &dyn PromptStore;
    fn deployments(&self) -> &dyn DeploymentStore;
}

/// Root database interface providing store access and transaction support
#[async_trait]
pub trait Database: Send + Sync {
    fn servers(&self) -> &dyn ServerStore;
    fn providers(&self) -> &dyn ProviderStore;
    fn agents(&self) -> &dyn AgentStore;
    fn skills(&self) -> &dyn SkillStore;
    fn prompts(&self) -> &dyn PromptStore;
    fn deployments(&self) -> &dyn DeploymentStore;

    async fn in_transaction<F, T>(&self, f: F) -> Result<T>
    where
        F: for<'a> FnOnce(&'a dyn Scope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send + 'a>> + Send,
        T: Send + 'static;
}

/// Server store interface for MCP server CRUD operations
#[async_trait]
pub trait ServerStore: ServerReader + Send + Sync {
    async fn create_server(&self, server_json: &ServerJSON, official_meta: &RegistryExtensions) -> Result<ServerResponse>;
    async fn update_server(&self, server_name: &str, version: &str, server_json: &ServerJSON) -> Result<ServerResponse>;
    async fn set_server_status(&self, server_name: &str, version: &str, status: &str) -> Result<ServerResponse>;
    async fn delete_server(&self, server_name: &str, version: &str) -> Result<()>;
    async fn get_latest_server(&self, server_name: &str) -> Result<ServerResponse>;
    async fn count_server_versions(&self, server_name: &str) -> Result<i32>;
    async fn check_version_exists(&self, server_name: &str, version: &str) -> Result<bool>;
    async fn unmark_as_latest(&self, server_name: &str) -> Result<()>;
    async fn acquire_server_create_lock(&self, server_name: &str) -> Result<()>;
    async fn set_server_embedding(&self, server_name: &str, version: &str, embedding: &SemanticEmbedding) -> Result<()>;
    async fn upsert_server_readme(&self, readme: &ServerReadme) -> Result<()>;
}

#[async_trait]
pub trait ServerReader: Send + Sync {
    async fn list_servers(&self, filter: &ServerFilter, cursor: &str, limit: i32) -> Result<(Vec<ServerResponse>, String)>;
    async fn get_server(&self, server_name: &str) -> Result<ServerResponse>;
    async fn get_server_version(&self, server_name: &str, version: &str) -> Result<ServerResponse>;
    async fn get_server_versions(&self, server_name: &str) -> Result<Vec<ServerResponse>>;
    async fn get_server_readme(&self, server_name: &str, version: &str) -> Result<ServerReadme>;
    async fn get_latest_server_readme(&self, server_name: &str) -> Result<ServerReadme>;
    async fn get_server_embedding_metadata(&self, server_name: &str, version: &str) -> Result<SemanticEmbeddingMetadata>;
}

/// Agent store interface
#[async_trait]
pub trait AgentStore: AgentReader + Send + Sync {
    async fn create_agent(&self, agent_json: &AgentJSON, official_meta: &AgentRegistryExtensions) -> Result<AgentResponse>;
    async fn update_agent(&self, agent_name: &str, version: &str, agent_json: &AgentJSON) -> Result<AgentResponse>;
    async fn set_agent_status(&self, agent_name: &str, version: &str, status: &str) -> Result<AgentResponse>;
    async fn delete_agent(&self, agent_name: &str, version: &str) -> Result<()>;
    async fn get_latest_agent(&self, agent_name: &str) -> Result<AgentResponse>;
    async fn count_agent_versions(&self, agent_name: &str) -> Result<i32>;
    async fn check_agent_version_exists(&self, agent_name: &str, version: &str) -> Result<bool>;
    async fn unmark_agent_as_latest(&self, agent_name: &str) -> Result<()>;
    async fn set_agent_embedding(&self, agent_name: &str, version: &str, embedding: &SemanticEmbedding) -> Result<()>;
}

#[async_trait]
pub trait AgentReader: Send + Sync {
    async fn list_agents(&self, filter: &AgentFilter, cursor: &str, limit: i32) -> Result<(Vec<AgentResponse>, String)>;
    async fn get_agent(&self, agent_name: &str) -> Result<AgentResponse>;
    async fn get_agent_version(&self, agent_name: &str, version: &str) -> Result<AgentResponse>;
    async fn get_agent_versions(&self, agent_name: &str) -> Result<Vec<AgentResponse>>;
    async fn get_agent_embedding_metadata(&self, agent_name: &str, version: &str) -> Result<SemanticEmbeddingMetadata>;
}

/// Skill store interface
#[async_trait]
pub trait SkillStore: SkillReader + Send + Sync {
    async fn create_skill(&self, skill_json: &SkillJSON, official_meta: &SkillRegistryExtensions) -> Result<SkillResponse>;
    async fn update_skill(&self, skill_name: &str, version: &str, skill_json: &SkillJSON) -> Result<SkillResponse>;
    async fn set_skill_status(&self, skill_name: &str, version: &str, status: &str) -> Result<SkillResponse>;
    async fn delete_skill(&self, skill_name: &str, version: &str) -> Result<()>;
    async fn get_latest_skill(&self, skill_name: &str) -> Result<SkillResponse>;
    async fn count_skill_versions(&self, skill_name: &str) -> Result<i32>;
    async fn check_skill_version_exists(&self, skill_name: &str, version: &str) -> Result<bool>;
    async fn unmark_skill_as_latest(&self, skill_name: &str) -> Result<()>;
}

#[async_trait]
pub trait SkillReader: Send + Sync {
    async fn list_skills(&self, filter: &SkillFilter, cursor: &str, limit: i32) -> Result<(Vec<SkillResponse>, String)>;
    async fn get_skill(&self, skill_name: &str) -> Result<SkillResponse>;
    async fn get_skill_version(&self, skill_name: &str, version: &str) -> Result<SkillResponse>;
    async fn get_skill_versions(&self, skill_name: &str) -> Result<Vec<SkillResponse>>;
}

/// Prompt store interface
#[async_trait]
pub trait PromptStore: PromptReader + Send + Sync {
    async fn create_prompt(&self, prompt_json: &PromptJSON, official_meta: &PromptRegistryExtensions) -> Result<PromptResponse>;
    async fn update_prompt(&self, prompt_name: &str, version: &str, prompt_json: &PromptJSON) -> Result<PromptResponse>;
    async fn delete_prompt(&self, prompt_name: &str, version: &str) -> Result<()>;
    async fn get_latest_prompt(&self, prompt_name: &str) -> Result<PromptResponse>;
    async fn count_prompt_versions(&self, prompt_name: &str) -> Result<i32>;
    async fn check_prompt_version_exists(&self, prompt_name: &str, version: &str) -> Result<bool>;
    async fn unmark_prompt_as_latest(&self, prompt_name: &str) -> Result<()>;
}

#[async_trait]
pub trait PromptReader: Send + Sync {
    async fn list_prompts(&self, filter: &PromptFilter, cursor: &str, limit: i32) -> Result<(Vec<PromptResponse>, String)>;
    async fn get_prompt(&self, prompt_name: &str) -> Result<PromptResponse>;
    async fn get_prompt_version(&self, prompt_name: &str, version: &str) -> Result<PromptResponse>;
    async fn get_prompt_versions(&self, prompt_name: &str) -> Result<Vec<PromptResponse>>;
}

/// Provider store interface
#[async_trait]
pub trait ProviderStore: Send + Sync {
    async fn create_provider(&self, provider: &Provider) -> Result<Provider>;
    async fn get_provider(&self, name: &str) -> Result<Provider>;
    async fn list_providers(&self) -> Result<Vec<Provider>>;
    async fn update_provider(&self, provider: &Provider) -> Result<Provider>;
    async fn delete_provider(&self, name: &str) -> Result<()>;
}

/// Deployment store interface
#[async_trait]
pub trait DeploymentStore: Send + Sync {
    async fn create_deployment(&self, deployment: &Deployment) -> Result<()>;
    async fn list_deployments(&self, filter: &DeploymentFilter) -> Result<Vec<Deployment>>;
    async fn get_deployment(&self, id: &str) -> Result<Deployment>;
    async fn update_deployment_state(&self, id: &str, patch: &DeploymentStatePatch) -> Result<()>;
    async fn delete_deployment(&self, id: &str) -> Result<()>;
    async fn acquire_apply_lock(&self, identity_key: &str) -> Result<()>;
}

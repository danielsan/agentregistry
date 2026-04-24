use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Record already exists: {0}")]
    AlreadyExists(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Invalid version: cannot publish duplicate version")]
    InvalidVersion,

    #[error("Maximum number of versions reached (10000): please reach out at https://github.com/modelcontextprotocol/registry to explain your use case")]
    MaxVersionsReached,

    #[error("SQLx error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<oxi_core::Error> for Error {
    fn from(err: oxi_core::Error) -> Self {
        match err {
            oxi_core::Error::NotFound(msg) => Error::NotFound(msg),
            oxi_core::Error::AlreadyExists(msg) => Error::AlreadyExists(msg),
            oxi_core::Error::InvalidInput(msg) => Error::InvalidInput(msg),
            oxi_core::Error::Unauthorized(msg) => Error::Forbidden(msg),
            oxi_core::Error::Forbidden(msg) => Error::Forbidden(msg),
            oxi_core::Error::Internal(msg) => Error::Internal(msg),
            oxi_core::Error::Database(msg) => Error::Database(msg),
            _ => Error::Internal(err.to_string()),
        }
    }
}

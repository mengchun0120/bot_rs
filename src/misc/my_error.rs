use serde_json;
use thiserror;

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Duplicate Key {0}")]
    DuplicateKey(String),

    #[error("Cannot find {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}

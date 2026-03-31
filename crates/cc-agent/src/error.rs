use thiserror::Error;

pub type Result<T> = std::result::Result<T, AgentError>;

#[derive(Debug, Error)]
pub enum AgentError {
    #[error("internal error: {0}")]
    Internal(String),
    #[error("execution failed: {0}")]
    ExecutionFailed(String),
    #[error("invalid stage transition: {0}")]
    InvalidStage(String),
}

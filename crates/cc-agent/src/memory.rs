use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryExperience {
    pub solution: String,
}

#[async_trait]
pub trait MemoryStore: Send + Sync {
    async fn save_experience(&self, key: &str, value: &str, durable: bool) -> Result<()>;
    async fn search(&self, query: &str, limit: usize) -> Result<Vec<MemoryExperience>>;
}

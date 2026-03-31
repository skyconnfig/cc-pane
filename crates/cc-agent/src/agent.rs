use crate::error::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentOptions;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub output: String,
}

#[derive(Debug, Clone, Default)]
pub struct SmartAgent;

impl SmartAgent {
    pub async fn execute(&self, prompt: &str, _options: AgentOptions) -> Result<AgentResponse> {
        Ok(AgentResponse {
            output: prompt.to_string(),
        })
    }
}

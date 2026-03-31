use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AgentRole {
    Architect,
    Coordinator,
    Documentation,
    Reviewer,
    BackendCoder,
    FrontendCoder,
    DatabaseEngineer,
    DevOps,
    Tester,
    Security,
}

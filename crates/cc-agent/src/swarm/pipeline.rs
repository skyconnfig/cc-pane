use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString, Serialize, Deserialize)]
pub enum PipelineStage {
    #[strum(serialize = "analyze")]
    Analyze,
    #[strum(serialize = "architect")]
    Architect,
    #[strum(serialize = "decompose")]
    Decompose,
    #[strum(serialize = "develop_backend")]
    DevelopBackend,
    #[strum(serialize = "develop_frontend")]
    DevelopFrontend,
    #[strum(serialize = "develop_database")]
    DevelopDatabase,
    #[strum(serialize = "review_code")]
    ReviewCode,
    #[strum(serialize = "test_unit")]
    TestUnit,
    #[strum(serialize = "test_integration")]
    TestIntegration,
    #[strum(serialize = "fix")]
    Fix,
    #[strum(serialize = "integrate")]
    Integrate,
    #[strum(serialize = "test_e2e")]
    TestE2E,
    #[strum(serialize = "cleanup")]
    Cleanup,
    #[strum(serialize = "format")]
    Format,
    #[strum(serialize = "deploy_prep")]
    DeployPrep,
    #[strum(serialize = "docker")]
    Docker,
    #[strum(serialize = "git_init")]
    GitInit,
    #[strum(serialize = "git_commit")]
    GitCommit,
    #[strum(serialize = "github_push")]
    GitHubPush,
    #[strum(serialize = "summarize")]
    Summarize,
    #[strum(serialize = "learn")]
    Learn,
    #[strum(serialize = "done")]
    Done,
    #[strum(serialize = "failed")]
    Failed,
}

impl PipelineStage {
    pub fn dependencies(&self) -> Vec<PipelineStage> {
        use PipelineStage::*;
        match self {
            Analyze => vec![],
            Architect => vec![Analyze],
            Decompose => vec![Architect],
            DevelopBackend | DevelopFrontend | DevelopDatabase => vec![Decompose],
            ReviewCode => vec![DevelopBackend, DevelopFrontend, DevelopDatabase],
            TestUnit => vec![ReviewCode],
            TestIntegration => vec![TestUnit],
            Fix => vec![TestUnit, TestIntegration],
            Integrate => vec![TestIntegration],
            TestE2E => vec![Integrate],
            Cleanup => vec![TestE2E],
            Format => vec![Cleanup],
            DeployPrep => vec![Format],
            Docker => vec![DeployPrep],
            GitInit => vec![Docker],
            GitCommit => vec![GitInit],
            GitHubPush => vec![GitCommit],
            Summarize => vec![GitHubPush],
            Learn => vec![Summarize],
            Done => vec![Learn],
            Failed => vec![],
        }
    }

    pub fn is_parallel(&self) -> bool {
        matches!(
            self,
            Self::DevelopBackend | Self::DevelopFrontend | Self::DevelopDatabase
        )
    }

    pub fn has_self_fix(&self) -> bool {
        matches!(self, Self::TestUnit | Self::TestIntegration | Self::TestE2E)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProjectArtifacts {
    pub files: Vec<String>,
    pub reports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineResult {
    pub project_id: Uuid,
    pub success: bool,
    pub final_stage: PipelineStage,
    pub stages_completed: Vec<PipelineStage>,
    pub stages_failed: Vec<PipelineStage>,
    pub total_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
    pub retry_count: usize,
    pub duration_secs: u64,
    pub artifacts: ProjectArtifacts,
    pub error_summary: Option<String>,
}

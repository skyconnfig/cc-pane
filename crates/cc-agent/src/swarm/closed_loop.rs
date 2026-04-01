use crate::agent::SmartAgent;
use crate::error::Result;
use crate::memory::MemoryStore;
use crate::swarm::decomposer::TaskDecomposer;
use crate::swarm::evolution::EvolutionEngine;
use crate::swarm::pipeline::{PipelineResult, PipelineStage, ProjectArtifacts};
use crate::swarm::self_fix::SelfFixEngine;
use crate::tools::mcp::BuiltInTools;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

pub struct ClosedLoopCoordinator {
    pub project_id: Uuid,
    #[allow(dead_code)]
    master_agent: SmartAgent,
    #[allow(dead_code)]
    memory: Arc<dyn MemoryStore>,
    #[allow(dead_code)]
    tools: BuiltInTools,
    decomposer: TaskDecomposer,
    #[allow(dead_code)]
    fix_engine: SelfFixEngine,
    evolution: EvolutionEngine,
    current_stage: Arc<tokio::sync::RwLock<PipelineStage>>,
    artifacts: Arc<tokio::sync::RwLock<ProjectArtifacts>>,
}

impl ClosedLoopCoordinator {
    pub async fn new(project_id: Uuid, master_agent: SmartAgent, memory: Arc<dyn MemoryStore>) -> Self {
        let tools = BuiltInTools::new("http://127.0.0.1:8787/mcp");
        let decomposer = TaskDecomposer::new(master_agent.clone());
        let fix_engine = SelfFixEngine::new(master_agent.clone(), tools.clone());
        let evolution = EvolutionEngine::new(master_agent.clone(), memory.clone());
        Self {
            project_id,
            master_agent,
            memory,
            tools,
            decomposer,
            fix_engine,
            evolution,
            current_stage: Arc::new(tokio::sync::RwLock::new(PipelineStage::Analyze)),
            artifacts: Arc::new(tokio::sync::RwLock::new(ProjectArtifacts::default())),
        }
    }

    pub async fn run(&self, requirement: &str) -> Result<PipelineResult> {
        *self.current_stage.write().await = PipelineStage::Analyze;
        let modules = self.decomposer.decompose_to_modules(requirement).await?;
        *self.current_stage.write().await = PipelineStage::Decompose;

        let mut tasks = Vec::new();
        for module in &modules {
            tasks.extend(self.decomposer.decompose_to_tasks(module).await?);
        }

        *self.current_stage.write().await = PipelineStage::Summarize;
        let result = PipelineResult {
            project_id: self.project_id,
            success: true,
            final_stage: PipelineStage::Done,
            stages_completed: vec![PipelineStage::Analyze, PipelineStage::Decompose],
            stages_failed: vec![],
            total_tasks: tasks.len(),
            completed_tasks: tasks.len(),
            failed_tasks: 0,
            retry_count: 0,
            duration_secs: 0,
            artifacts: self.artifacts.read().await.clone(),
            error_summary: None,
        };
        let _ = self.evolution.summarize(&result).await?;
        *self.current_stage.write().await = PipelineStage::Done;
        Ok(result)
    }

    pub async fn get_status(&self) -> PipelineStatus {
        PipelineStatus {
            project_id: self.project_id,
            current_stage: *self.current_stage.read().await,
            artifacts: self.artifacts.read().await.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStatus {
    pub project_id: Uuid,
    pub current_stage: PipelineStage,
    pub artifacts: ProjectArtifacts,
}

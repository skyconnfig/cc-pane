use crate::agent::SmartAgent;
use crate::error::Result;
use crate::memory::MemoryStore;
use crate::swarm::pipeline::PipelineResult;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestPractice {
    pub id: String,
    pub category: PracticeCategory,
    pub title: String,
    pub description: String,
    pub context: String,
    pub solution: String,
    pub code_example: Option<String>,
    pub tags: Vec<String>,
    pub effectiveness_score: f32,
    pub usage_count: usize,
    pub created_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PracticeCategory {
    Architecture,
    CodeStyle,
    Testing,
    Security,
    Performance,
    Deployment,
    Debugging,
    Tooling,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperiencePattern {
    pub pattern_id: String,
    pub problem_signature: String,
    pub solution_template: String,
    pub success_rate: f32,
    pub applicable_scenarios: Vec<String>,
}

pub struct EvolutionEngine {
    #[allow(dead_code)]
    agent: SmartAgent,
    memory: Arc<dyn MemoryStore>,
}

impl EvolutionEngine {
    pub fn new(agent: SmartAgent, memory: Arc<dyn MemoryStore>) -> Self {
        Self { agent, memory }
    }

    pub async fn summarize(&self, result: &PipelineResult) -> Result<Vec<BestPractice>> {
        let analysis = self.analyze_project_execution(result).await?;
        let success_patterns = self.extract_success_patterns(&analysis).await?;
        let practices = self
            .generate_best_practices(&success_patterns, &[], result)
            .await?;
        for practice in &practices {
            self.memory
                .save_experience(
                    &format!("best_practice:{}", practice.id),
                    &serde_json::to_string(practice).unwrap_or_default(),
                    true,
                )
                .await?;
        }
        Ok(practices)
    }

    async fn analyze_project_execution(&self, result: &PipelineResult) -> Result<ProjectAnalysis> {
        Ok(ProjectAnalysis {
            total_duration: result.duration_secs,
            success_rate: if result.total_tasks == 0 {
                1.0
            } else {
                result.completed_tasks as f32 / result.total_tasks as f32
            },
            retry_count: result.retry_count,
            bottleneck_stages: vec![],
            success_factors: vec![],
            failure_factors: vec![],
        })
    }

    async fn extract_success_patterns(
        &self,
        _analysis: &ProjectAnalysis,
    ) -> Result<Vec<ExperiencePattern>> {
        Ok(vec![])
    }

    async fn generate_best_practices(
        &self,
        success_patterns: &[ExperiencePattern],
        _failure_lessons: &[ExperiencePattern],
        result: &PipelineResult,
    ) -> Result<Vec<BestPractice>> {
        Ok(success_patterns
            .iter()
            .enumerate()
            .map(|(i, pattern)| BestPractice {
                id: format!("bp-{}-{}", result.project_id, i),
                category: PracticeCategory::Tooling,
                title: format!("成功模式：{}", pattern.pattern_id),
                description: pattern.solution_template.clone(),
                context: pattern.applicable_scenarios.join(", "),
                solution: pattern.solution_template.clone(),
                code_example: None,
                tags: vec!["success".to_string()],
                effectiveness_score: pattern.success_rate,
                usage_count: 0,
                created_at: chrono::Utc::now().to_rfc3339(),
            })
            .collect())
    }

    pub async fn retrieve_practices(&self, context: &str) -> Result<Vec<BestPractice>> {
        let experiences = self.memory.search(context, 10).await?;
        Ok(experiences
            .into_iter()
            .filter_map(|exp| serde_json::from_str(&exp.solution).ok())
            .collect())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAnalysis {
    pub total_duration: u64,
    pub success_rate: f32,
    pub retry_count: usize,
    pub bottleneck_stages: Vec<String>,
    pub success_factors: Vec<String>,
    pub failure_factors: Vec<String>,
}

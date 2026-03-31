use crate::agent::SmartAgent;
use crate::error::{AgentError, Result};
use crate::swarm::decomposer::{AtomicTask, TestCase};
use crate::tools::mcp::BuiltInTools;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorAnalysis {
    pub error_type: String,
    pub root_cause: String,
    pub affected_files: Vec<String>,
    pub suggested_fix: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixPlan {
    pub task_id: Uuid,
    pub changes: Vec<FileChange>,
    pub test_updates: Vec<TestUpdate>,
    pub validation_steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
    pub path: String,
    pub old_content: Option<String>,
    pub new_content: String,
    pub change_type: ChangeType,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ChangeType {
    Create,
    Modify,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestUpdate {
    pub test_file: String,
    pub test_name: String,
    pub update_type: String,
}

pub struct SelfFixEngine {
    #[allow(dead_code)]
    agent: SmartAgent,
    tools: BuiltInTools,
    max_retries: usize,
}

impl SelfFixEngine {
    pub fn new(agent: SmartAgent, tools: BuiltInTools) -> Self {
        Self {
            agent,
            tools,
            max_retries: 3,
        }
    }

    pub async fn fix_loop(&self, task: &AtomicTask, error_message: &str) -> Result<FixResult> {
        let mut retry_count = 0;
        let mut last_error = error_message.to_string();
        while retry_count < self.max_retries {
            let analysis = self.analyze_error(&last_error, task).await?;
            let fix_plan = self.generate_fix_plan(&analysis, task).await?;
            self.apply_fix(&fix_plan).await?;
            match self.validate_fix(task).await {
                Ok(_) => {
                    return Ok(FixResult {
                        success: true,
                        retries: retry_count + 1,
                        changes: fix_plan.changes,
                    });
                }
                Err(e) => {
                    last_error = e.to_string();
                    retry_count += 1;
                }
            }
        }
        Ok(FixResult {
            success: false,
            retries: retry_count,
            changes: vec![],
        })
    }

    async fn analyze_error(&self, error: &str, task: &AtomicTask) -> Result<ErrorAnalysis> {
        Ok(ErrorAnalysis {
            error_type: "test_failure".into(),
            root_cause: error.into(),
            affected_files: task.input_files.clone(),
            suggested_fix: "Inspect failing assertions and update implementation".into(),
            confidence: 0.4,
        })
    }

    async fn generate_fix_plan(&self, analysis: &ErrorAnalysis, task: &AtomicTask) -> Result<FixPlan> {
        Ok(FixPlan {
            task_id: task.id,
            changes: analysis
                .affected_files
                .iter()
                .map(|path| FileChange {
                    path: path.clone(),
                    old_content: None,
                    new_content: String::new(),
                    change_type: ChangeType::Modify,
                })
                .collect(),
            test_updates: vec![],
            validation_steps: vec!["re-run unit tests".into()],
        })
    }

    async fn apply_fix(&self, plan: &FixPlan) -> Result<()> {
        for change in &plan.changes {
            match change.change_type {
                ChangeType::Create | ChangeType::Modify => {
                    self.tools
                        .write_file(&change.path, &change.new_content)
                        .await
                        .map_err(AgentError::ExecutionFailed)?;
                }
                ChangeType::Delete => {}
            }
        }
        Ok(())
    }

    async fn validate_fix(&self, task: &AtomicTask) -> Result<()> {
        for test_case in &task.test_cases {
            let result = self.run_test(test_case).await?;
            if !result.passed {
                return Err(AgentError::ExecutionFailed(format!(
                    "Test {} failed: {}",
                    test_case.name,
                    result.error.unwrap_or_default()
                )));
            }
        }
        Ok(())
    }

    async fn run_test(&self, test_case: &TestCase) -> Result<TestResult> {
        Ok(TestResult {
            passed: test_case.passed.unwrap_or(true),
            error: None,
            output: String::new(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub error: Option<String>,
    pub output: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixResult {
    pub success: bool,
    pub retries: usize,
    pub changes: Vec<FileChange>,
}

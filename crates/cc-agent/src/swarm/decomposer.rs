use crate::agent::SmartAgent;
use crate::error::{AgentError, Result};
use crate::swarm::roles::AgentRole;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureModule {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub priority: u8,
    pub dependencies: Vec<Uuid>,
    pub estimated_hours: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtomicTask {
    pub id: Uuid,
    pub module_id: Uuid,
    pub title: String,
    pub description: String,
    pub assigned_role: AgentRole,
    pub input_files: Vec<String>,
    pub output_files: Vec<String>,
    pub test_cases: Vec<TestCase>,
    pub status: TaskStatus,
    pub retry_count: usize,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub id: Uuid,
    pub name: String,
    pub input: String,
    pub expected_output: String,
    pub actual_output: Option<String>,
    pub passed: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

pub struct TaskDecomposer {
    #[allow(dead_code)]
    agent: SmartAgent,
}

impl TaskDecomposer {
    pub fn new(agent: SmartAgent) -> Self {
        Self { agent }
    }

    pub async fn decompose_to_modules(&self, _requirement: &str) -> Result<Vec<FeatureModule>> {
        Ok(vec![])
    }

    pub async fn decompose_to_tasks(&self, module: &FeatureModule) -> Result<Vec<AtomicTask>> {
        let task = AtomicTask {
            id: Uuid::new_v4(),
            module_id: module.id,
            title: format!("Implement {}", module.name),
            description: module.description.clone(),
            assigned_role: AgentRole::BackendCoder,
            input_files: vec![],
            output_files: vec![],
            test_cases: vec![],
            status: TaskStatus::Pending,
            retry_count: 0,
            error_message: None,
        };
        Ok(vec![task])
    }

    pub async fn build_dependency_graph(&self, tasks: &[AtomicTask]) -> Result<TaskGraph> {
        let mut graph = TaskGraph::new();
        for task in tasks {
            graph.add_node(task.id);
            for other_task in tasks {
                if other_task.id == task.id {
                    continue;
                }
                for input in &task.input_files {
                    if other_task.output_files.contains(input) {
                        graph.add_edge(other_task.id, task.id);
                    }
                }
            }
        }

        if graph.has_cycle() {
            return Err(AgentError::Internal("Circular dependency detected".to_string()));
        }

        Ok(graph)
    }

    pub async fn generate_execution_plan(&self, graph: &TaskGraph) -> Result<Vec<Vec<Uuid>>> {
        Ok(graph.topological_sort_parallel())
    }
}

pub struct TaskGraph {
    nodes: Vec<Uuid>,
    edges: HashMap<Uuid, Vec<Uuid>>,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, id: Uuid) {
        if !self.nodes.contains(&id) {
            self.nodes.push(id);
            self.edges.insert(id, Vec::new());
        }
    }

    pub fn add_edge(&mut self, from: Uuid, to: Uuid) {
        self.edges.entry(from).or_default().push(to);
    }

    pub fn has_cycle(&self) -> bool {
        let mut temp = HashSet::new();
        let mut perm = HashSet::new();
        self.nodes
            .iter()
            .any(|node| self.visit(*node, &mut temp, &mut perm))
    }

    fn visit(&self, node: Uuid, temp: &mut HashSet<Uuid>, perm: &mut HashSet<Uuid>) -> bool {
        if perm.contains(&node) {
            return false;
        }
        if !temp.insert(node) {
            return true;
        }
        if let Some(neighbors) = self.edges.get(&node)
            && neighbors
                .iter()
                .copied()
                .any(|next| self.visit(next, temp, perm))
        {
            return true;
        }
        temp.remove(&node);
        perm.insert(node);
        false
    }

    pub fn topological_sort_parallel(&self) -> Vec<Vec<Uuid>> {
        let mut in_degree: HashMap<Uuid, usize> = self.nodes.iter().map(|n| (*n, 0)).collect();
        for tos in self.edges.values() {
            for to in tos {
                *in_degree.entry(*to).or_insert(0) += 1;
            }
        }

        let mut queue: VecDeque<Uuid> = in_degree
            .iter()
            .filter_map(|(n, d)| (*d == 0).then_some(*n))
            .collect();

        let mut levels = Vec::new();
        while !queue.is_empty() {
            let mut batch = Vec::new();
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    batch.push(node);
                    if let Some(nexts) = self.edges.get(&node) {
                        for next in nexts {
                            if let Some(v) = in_degree.get_mut(next) {
                                *v -= 1;
                                if *v == 0 {
                                    queue.push_back(*next);
                                }
                            }
                        }
                    }
                }
            }
            levels.push(batch);
        }
        levels
    }
}

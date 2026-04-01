use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct MCPClient {
    server_url: String,
    tools: HashMap<String, MCPTool>,
}

impl MCPClient {
    pub fn new(server_url: String) -> Self {
        Self {
            server_url,
            tools: HashMap::new(),
        }
    }

    pub async fn initialize(&mut self) -> Result<(), String> {
        let tools = self.fetch_tools().await?;
        self.tools = tools;
        Ok(())
    }

    pub async fn fetch_tools(&self) -> Result<HashMap<String, MCPTool>, String> {
        let _ = &self.server_url;
        Ok(HashMap::new())
    }

    pub async fn call_tool(
        &self,
        tool_name: &str,
        arguments: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let _ = (&self.server_url, tool_name, arguments);
        Ok(json!({"ok": true}))
    }
}

#[derive(Debug, Clone)]
pub struct BuiltInTools {
    mcp: MCPClient,
}

impl BuiltInTools {
    pub fn new(server_url: impl Into<String>) -> Self {
        Self {
            mcp: MCPClient::new(server_url.into()),
        }
    }

    pub async fn read_file(&self, path: &str) -> Result<String, String> {
        self.mcp
            .call_tool("read_file", json!({ "path": path }))
            .await?
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Invalid response".to_string())
    }

    pub async fn write_file(&self, path: &str, content: &str) -> Result<(), String> {
        self.mcp
            .call_tool("write_file", json!({"path": path, "content": content}))
            .await?;
        Ok(())
    }

    pub async fn search_code(
        &self,
        pattern: &str,
        path: Option<&str>,
    ) -> Result<Vec<SearchResult>, String> {
        let result = self
            .mcp
            .call_tool("search_code", json!({"pattern": pattern, "path": path}))
            .await?;
        serde_json::from_value(result).map_err(|e| e.to_string())
    }

    pub async fn run_command(
        &self,
        command: &str,
        cwd: Option<&str>,
    ) -> Result<CommandResult, String> {
        let result = self
            .mcp
            .call_tool("run_command", json!({"command": command, "cwd": cwd}))
            .await?;
        serde_json::from_value(result).map_err(|e| e.to_string())
    }

    pub async fn call_skill(
        &self,
        skill_name: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        self.mcp
            .call_tool(
                "call_skill",
                json!({"skill_name": skill_name, "params": params}),
            )
            .await
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub file: String,
    pub line: usize,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

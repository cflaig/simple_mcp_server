use rust_mcp_sdk::schema::{schema_utils::CallToolError, CallToolResult, TextContent};
use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    tool_box,
};
use std::process::Command;
use std::fs;
use std::fmt;
use std::error::Error as StdError;
use tracing::{info, debug};

// Custom error type for tool errors
#[derive(Debug)]
struct ToolError {
    message: String,
}

impl ToolError {
    fn new<S: Into<String>>(message: S) -> Self {
        ToolError {
            message: message.into(),
        }
    }
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for ToolError {}


#[mcp_tool(
    name = "ls_command",
    description = "Executes the ls command with the provided path and options. Use the 'args' parameter to pass ls options like '-l', '-a', etc."
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct LsCommandTool {
    /// The path to the list (defaults to current directory if not specified)
    #[serde(default = "default_path")]
    path: String,
    /// Optional ls command options (e.g., "-l -a" for long listing including hidden files)
    #[serde(default)]
    args: Option<String>,
}

fn default_path() -> String {
    ".".to_string()
}

impl LsCommandTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        debug!("Executing ls command for path: {}", self.path);
        let mut ls_command = Command::new("ls");
        
        // Add any options if provided (before the path to ensure options are applied correctly)
        if let Some(args) = &self.args {
            // Handle options that start with a dash
            let command_args: Vec<&str> = args.split_whitespace()
                .filter(|arg| !arg.is_empty())
                .collect();
            
            debug!("Using ls command arguments: {:?}", command_args);
            for arg in command_args {
                ls_command.arg(arg);
            }
        }
        
        // Add the path (after options)
        ls_command.arg(&self.path);
        
        // Execute the command
        info!("Executing ls command for directory: {}", self.path);
        match ls_command.output() {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                    debug!("ls command executed successfully");
                    Ok(CallToolResult::text_content(vec![TextContent::from(stdout)]))
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                    info!("ls command failed: {}", stderr);
                    Err(CallToolError::new(ToolError::new(format!("ls command failed: {}", stderr))))
                }
            },
            Err(e) => {
                info!("Failed to execute ls command: {}", e);
                Err(CallToolError::new(ToolError::new(format!("Failed to execute ls command: {}", e))))
            },
        }
    }
}

#[mcp_tool(
    name = "read_file",
    description = "Reads the contents of a file and returns them"
)]
#[derive(Debug, ::serde::Deserialize, ::serde::Serialize, JsonSchema)]
pub struct ReadFileTool {
    /// The path to the file
    path: String,
}

impl ReadFileTool {
    pub fn call_tool(&self) -> Result<CallToolResult, CallToolError> {
        info!("Reading file content from: {}", self.path);
        
        match fs::read_to_string(&self.path) {
            Ok(content) => {
                let content_length = content.len();
                info!("Successfully read {} bytes from file: {}", content_length, self.path);
                Ok(CallToolResult::text_content(vec![TextContent::from(content)]))
            },
            Err(e) => {
                info!("Failed to read file {}: {}", self.path, e);
                Err(CallToolError::new(ToolError::new(format!("Failed to read file {}: {}", self.path, e))))
            }
        }
    }
}

tool_box!(FileSystemTools, [LsCommandTool, ReadFileTool]);
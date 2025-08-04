use async_trait::async_trait;
use rust_mcp_sdk::schema::{
    schema_utils::CallToolError, CallToolRequest, CallToolResult, ListToolsRequest,
    ListToolsResult, RpcError,
};
use rust_mcp_sdk::{mcp_server::ServerHandler, McpServer};
use tracing::info;

use crate::tools::FileSystemTools;

pub struct FileSystemServerHandler;

#[async_trait]
#[allow(unused)]
impl ServerHandler for FileSystemServerHandler {
    // Handle ListToolsRequest, return a list of available tools as ListToolsResult
    async fn handle_list_tools_request(
        &self,
        request: ListToolsRequest,
        runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            meta: None,
            next_cursor: None,
            tools: FileSystemTools::tools(),
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        // Attempt to convert request parameters into FileSystemTools enum
        let tool_params: FileSystemTools =
            FileSystemTools::try_from(request.params).map_err(CallToolError::new)?;

        // Match the tool variant and execute its corresponding logic
        match tool_params {
            FileSystemTools::LsCommandTool(ls_command_tool) => ls_command_tool.call_tool(),
            FileSystemTools::ReadFileTool(read_file_tool) => read_file_tool.call_tool(),
        }
    }

    async fn on_server_started(&self, runtime: &dyn McpServer) {
        info!("File System Server handler initialized and ready to process requests");
    }
}
//! {{project-name}}
//!
//! {{description}}

use async_trait::async_trait;
use mcp_protocol_sdk::{
    core::{
        error::{McpError, McpResult},
        tool::ToolHandler,
    },
    protocol::types::{Content, ToolResult},
    server::McpServer,
    transport::stdio::StdioServerTransport,
};
use serde_json::{json, Value};
use std::collections::HashMap;
use tracing::{info, warn};

// ============================================================================
// Tool Handlers
// ============================================================================

/// Example tool that demonstrates basic functionality
struct ExampleTool;

#[async_trait]
impl ToolHandler for ExampleTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        info!("ExampleTool called with arguments: {:?}", arguments);

        let message = arguments
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Hello from {{project-name}}!");

        Ok(ToolResult {
            content: vec![Content::text(format!("{{project-name}} says: {}", message))],
            is_error: None,
        })
    }
}

// ============================================================================
// Add your custom tool handlers here
// ============================================================================

// TODO: Implement your custom tools
// struct MyCustomTool;
// 
// #[async_trait]
// impl ToolHandler for MyCustomTool {
//     async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
//         // Your tool implementation here
//         Ok(ToolResult {
//             content: vec![Content::text("Custom tool result".to_string())],
//             is_error: None,
//         })
//     }
// }

// ============================================================================
// Server Configuration
// ============================================================================

async fn setup_server() -> McpResult<McpServer> {
    let mut server = McpServer::new("{{project-name}}".to_string(), "0.1.0".to_string());

    // Add the example tool
    server
        .add_tool(
            "example".to_string(),
            Some("An example tool that echoes a message".to_string()),
            json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "Message to echo back"
                    }
                }
            }),
            ExampleTool,
        )
        .await?;

    // TODO: Add your custom tools here
    // server
    //     .add_tool(
    //         "my_tool".to_string(),
    //         Some("Description of my tool".to_string()),
    //         json!({
    //             "type": "object",
    //             "properties": {
    //                 "param": {
    //                     "type": "string",
    //                     "description": "Parameter description"
    //                 }
    //             },
    //             "required": ["param"]
    //         }),
    //         MyCustomTool,
    //     )
    //     .await?;

    info!("Server configured with {} tools", server.tool_count());
    Ok(server)
}

// ============================================================================
// Main Function
// ============================================================================

#[tokio::main]
async fn main() -> McpResult<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("{{project-name}}=info,mcp_protocol_sdk=info")
        .init();

    info!("Starting {{project-name}} MCP server...");

    // Setup server
    let server = setup_server().await?;

    // Create transport (STDIO for Claude Desktop compatibility)
    let transport = StdioServerTransport::new();

    info!("Server ready! Listening for MCP requests...");

    // Start the server
    server.start(transport).await?;

    // This point is reached when the server shuts down
    info!("{{project-name}} server stopped");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_example_tool() {
        let tool = ExampleTool;
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("test message"));

        let result = tool.call(args).await.unwrap();
        assert!(!result.content.is_empty());
        assert!(result.is_error.is_none());
    }

    #[tokio::test]
    async fn test_server_setup() {
        let server = setup_server().await.unwrap();
        assert!(server.tool_count() > 0);
    }
}

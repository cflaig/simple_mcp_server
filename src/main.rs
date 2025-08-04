mod handler;
mod tools;

use std::time::Duration;

use rust_mcp_sdk::mcp_server::{hyper_server, HyperServerOptions};

use handler::FileSystemServerHandler;
use rust_mcp_sdk::schema::{
    Implementation, InitializeResult, ServerCapabilities, ServerCapabilitiesTools,
    LATEST_PROTOCOL_VERSION,
};

use rust_mcp_sdk::{error::SdkResult, mcp_server::ServerHandler};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct ServerState<H: ServerHandler> {
    pub server_details: InitializeResult,
    pub handler: H,
}

#[tokio::main]
async fn main() -> SdkResult<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=info", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let server_details = InitializeResult {
        server_info: Implementation {
            name: "File System MCP Server SSE".to_string(),
            version: "0.0.1".to_string(),
            title: Some("File System MCP Server SSE".to_string()),
        },
        capabilities: ServerCapabilities {
            // indicates that the server supports mcp tools
            tools: Some(ServerCapabilitiesTools { list_changed: None }),
            ..Default::default() // Using default values for other fields
        },
        meta: None,
        instructions: Some("server instructions...".to_string()),
        protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
    };

    let handler = FileSystemServerHandler {};

    let server_host = "127.0.0.1".to_string();
    let port = 8080;
    info!("Creating File System Server with host: {}:{}", server_host, port);

    let server = hyper_server::create_server(
        server_details,
        handler,
        HyperServerOptions {
            host: server_host,
            ping_interval: Duration::from_secs(5),
            port,
            ..Default::default()
        },
    );

    info!("Starting File System Server...");
    server.start().await?;
    
    info!("File System Server stopped");
    Ok(())
}

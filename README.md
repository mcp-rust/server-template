# {{project-name}}

{{description}}

## Quick Start

### Prerequisites

- Rust 1.75 or higher
- Cargo

### Installation

1. Clone this repository:
```bash
git clone https://github.com/{{github-username}}/{{project-name}}.git
cd {{project-name}}
```

2. Build the project:
```bash
cargo build --release
```

3. Run the server:
```bash
cargo run
```

## Usage

### With Claude Desktop

Add this server to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "{{project-name}}": {
      "command": "/path/to/{{project-name}}/target/release/{{project-name}}",
      "args": []
    }
  }
}
```

### Testing the Server

You can test the server using the MCP client tools:

```bash
# Install mcp-client (if available)
cargo install mcp-client

# Test the server
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list", "params": {}}' | ./target/release/{{project-name}}
```

## Available Tools

- **example** - An example tool that echoes a message

## Development

### Adding New Tools

1. Create a new tool handler struct implementing `ToolHandler`:

```rust
struct MyTool;

#[async_trait]
impl ToolHandler for MyTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        // Your tool implementation
        Ok(ToolResult {
            content: vec![Content::text("Tool result".to_string())],
            is_error: None,
        })
    }
}
```

2. Register the tool in the `setup_server()` function:

```rust
server.add_tool(
    "my_tool".to_string(),
    Some("Description of my tool".to_string()),
    json!({
        "type": "object",
        "properties": {
            "param": {
                "type": "string",
                "description": "Parameter description"
            }
        },
        "required": ["param"]
    }),
    MyTool,
).await?;
```

### Testing

Run the test suite:

```bash
cargo test
```

### Features

Enable additional features as needed:

```bash
# Build with HTTP transport
cargo build --features http

# Build with WebSocket transport  
cargo build --features websocket

# Build with database support
cargo build --features database

# Build with file system support
cargo build --features file-system
```

## Configuration

### Environment Variables

- `RUST_LOG` - Set logging level (e.g., `RUST_LOG=debug`)

### Transport Options

The server supports multiple transport protocols:

- **STDIO** (default) - For Claude Desktop integration
- **HTTP** - For web-based clients
- **WebSocket** - For real-time applications

To switch transports, modify the transport creation in `main.rs`:

```rust
// HTTP transport
let transport = HttpServerTransport::new("0.0.0.0:3000");

// WebSocket transport  
let transport = WebSocketServerTransport::new("0.0.0.0:8080");
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run `cargo test` and `cargo clippy`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Resources

- [MCP Specification](https://modelcontextprotocol.io/)
- [MCP Rust SDK Documentation](https://docs.rs/mcp-protocol-sdk)
- [Claude Desktop MCP Guide](https://docs.anthropic.com/claude/docs/mcp)

## Support

- Create an issue for bug reports or feature requests
- Check existing issues before creating new ones
- Provide minimal reproduction examples for bugs

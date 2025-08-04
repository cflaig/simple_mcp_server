# Simple MCP Server

A simple File System MCP (Model-Conext-Protocol) Server implementation using the Rust MCP SDK. This server provides file system operations through a standardized interface.

## Features

- HTTP server running on localhost:8080
- Server-Sent Events (SSE) for real-time communication
- File system operations:
  - Directory listing with customizable options
  - File content reading
- Asynchronous request handling with tokio
- Comprehensive logging with tracing

## Requirements

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

## Installing Rust

### On macOS, Linux, or Unix-like systems:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Alternatively, you can use a package manager such as `brew`, `pacman`, or similar:

- macOS:
  ```bash
  brew install rustup
  ```
- Arch-based Linux distributions:
  ```bash
  sudo pacman -S rustup
  ```


### On Windows:

1. Download the Rust installer from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Run the installer and follow the on-screen instructions

### Verifying the installation:

After installation, restart your terminal and run:

```bash
rustc --version
cargo --version
```

Both commands should display version information, confirming that Rust and Cargo are properly installed.

## Building and Running the Server

### Clone the repository:

```bash
git clone https://github.com/cflaig/simple_mcp_server
cd simple_mcp_server
```

### Build the project:

```bash
cargo build
```

For a release build with optimizations:

```bash
cargo build --release
```

### Run the server:

```bash
cargo run
```

For a release build:

```bash
cargo run --release
```

The server will start on `127.0.0.1:8080` by default.

## Usage Examples

The server provides two main tools:

### 1. List Directory Contents

This tool executes the `ls` command with customizable options.

Example request parameters:
```json
{
  "path": "/path/to/directory",
  "args": "-la"
}
```

### 2. Read File Contents

This tool reads and returns the contents of a specified file.

Example request parameters:
```json
{
  "path": "/path/to/file.txt"
}
```

## Development

### Environment Variables

- `RUST_LOG`: Controls the logging level (e.g., `info`, `debug`, `trace`)

Example:
```bash
RUST_LOG=debug cargo run
```

### Project Structure

- `src/main.rs`: Server initialization and configuration
- `src/handler.rs`: Request handling implementation
- `src/tools.rs`: File system tools implementation

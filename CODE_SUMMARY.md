# Code Summary for MCP Agent Tools

## Project Structure

This project provides a complete Rust-based MCP (Model Context Protocol) server that offers essential tools for managing Rust development workflows with built-in safety mechanisms and automated backups.

### Main Components

1. **MCP Server (`mcp-agent-tools`)**
   - Written in Rust using `tokio`, `serde`, `reqwest`, `scraper`, and `chrono`
   - Implements JSON-RPC 2.0 over stdio for MCP protocol
   - Provides full safety mechanisms including automatic backups and Git integration

2. **DeepSeek Harness Configuration**
   The configuration includes:
   - 7 agents with specialized roles in a Rust development team
   - Resource constraints (24GB VRAM)
   - Parallel execution limits (maximum 2 concurrent agents)
   - Safety policies and tool access controls

## Core Tools Exposed by the MCP Server

1. **`safe_write_file`** - Secure file writing with automatic backup and Git integration
2. **`restore_backup`** - Restore previous states from backups or Git commits
3. **`web_fetch_and_extract`** - Web scraping and content extraction with HTML cleaning
4. **`run_cargo_check`** - Execute Cargo commands (check, clippy, test, build) securely
5. **`run_automation_task`** - Execute automation scripts with safety snapshots

## Team Configuration Files

- `dsh-config/settings.yaml` - Main team configuration
- `dsh-config/orchestrator.yaml` - Orchestration agent settings
- `dsh-config/web_explorer.yaml` - Web scraping agent
- `dsh-config/analyst.yaml` - Technical analysis agent
- `dsh-config/rust_developer.yaml` - Rust code generation agent
- `dsh-config/tester.yaml` - Testing and debugging agent
- `dsh-config/automation_engineer.yaml` - Automation scripting agent
- `dsh-config/qa_controller.yaml` - Quality assurance agent

## Safety Features

- Automatic backups before file modifications
- Git integration with automatic commits
- All file changes must go through `safe_write_file`
- Resource monitoring to prevent overloading (24GB VRAM limit)
- Maximum 2 concurrent agents to respect hardware constraints

## Build Requirements

- Rust 1.70+
- Cargo
- Git

## Key Source Files

### Main Server Implementation:
- `mcp-agent-tools/src/main.rs` - Entry point for the MCP server
- `mcp-agent-tools/src/lib.rs` - Core library with tool implementations
- `mcp-agent-tools/src/tools/` - Individual tool implementations

### Tools Directory:
- `mcp-agent-tools/src/tools/safe_write_file.rs`
- `mcp-agent-tools/src/tools/restore_backup.rs`
- `mcp-agent-tools/src/tools/web_fetch_and_extract.rs`
- `mcp-agent-tools/src/tools/run_cargo_check.rs`
- `mcp-agent-tools/src/tools/run_automation_task.rs`

## Installation Instructions

1. Build the MCP server:
```bash
cargo build --release
```

2. The binary will be available at `target/release/mcp-agent-tools`

3. Configure DeepSeek Harness to use this binary as your MCP server

## License

This project is licensed under the MIT License.
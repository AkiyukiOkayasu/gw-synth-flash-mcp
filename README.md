# gw-synth-flash-mcp

[![gw-synth-flash-mcp](https://img.shields.io/crates/v/gw-synth-flash-mcp.svg)](https://crates.io/crates/gw-synth-flash-mcp)
[![gw-synth-flash-mcp](https://docs.rs/gw-synth-flash-mcp/badge.svg)](https://docs.rs/gw-synth-flash-mcp)

An unofficial MCP (Model Context Protocol) server that exposes a few Gowin IDE CLI workflows as tools.

- Backend: Rust + `rmcp`
- Provides MCP tools (tool names are kept as `gowin.*` for compatibility)
- Current target OS: macOS

Japanese README: [README_JA.md](README_JA.md)

## Repo layout (assumed)

This repository is intended to be a standalone Rust crate:

- `src/`: MCP server implementation
- `examples/`: MCP client configuration templates
- `target/`: build artifacts (generated; not tracked)

## Prerequisites

- Gowin IDE installed on macOS
  - Default location: `/Applications/GowinIDE.app`
  - If different: pass `gowin_ide_app_path` in tool parameters

## Build

```sh
cargo build --release
```

Optional: install into your `$PATH`:

```sh
cargo install --path .
```

## Run (stdio)

```sh
./target/release/gw-synth-flash-mcp
```

Or, if installed:

```sh
gw-synth-flash-mcp
```

This server resolves relative paths based on a `project_root`.

Priority order:

1. Per-tool parameter: `project_root`
2. Environment variable: `GOWIN_MCP_PROJECT_ROOT`
3. Auto-detect from `cwd` by searching upward for `run_gowin.tcl` or `*.gprj`
4. Fallback: `cwd`

Example:

```sh
export GOWIN_MCP_PROJECT_ROOT="/ABS/PATH/TO/your/gowin/project"
./target/release/gw-synth-flash-mcp
```

## Quick start

1) Build

```sh
cargo build --release
```

1) Point your MCP client (e.g., VS Code/Copilot) at the built binary

- Example: `${workspaceFolder}/target/release/gw-synth-flash-mcp`

1) Set `GOWIN_MCP_PROJECT_ROOT` (or pass `project_root` per tool call)

## VS Code (GitHub Copilot) template

Template: [examples/vscode.mcp.json](examples/vscode.mcp.json)

- Uses `${workspaceFolder}/target/release/gw-synth-flash-mcp` by default
- Set `GOWIN_MCP_PROJECT_ROOT` to your Gowin project directory

## Claude Code template

Template: [examples/claude-code.mcp.json](examples/claude-code.mcp.json)

- Absolute-path template (replace `/ABS/PATH/...` values)

## Tools

### `gowin.run_tcl`

- Runs Tcl via `gw_sh`
- Provide either `tcl_path` (file) or `tcl_inline` (string)
- If `project_root` is set, relative paths resolve under it

### `gowin.list_cables`

- Enumerates available programmer cables via `programmer_cli` (tries multiple listing patterns)

### `gowin.program_fs`

- Programs a `.fs` bitstream into SRAM via `programmer_cli`
- If `cable` is omitted, it auto-selects from `list_cables`
- If needed, it retries with different cable inference strategies

## Logs

Each tool call writes logs under `<project_root>/.gowin-mcp/logs/`:

- `*.log`: combined stdout/stderr
- `*.json`: execution metadata (exit code, duration, args, etc.)

## Safety / Disclaimer

- This is unofficial software and is not affiliated with Gowin.
- Programming hardware can affect your FPGA/board. Use at your own risk.

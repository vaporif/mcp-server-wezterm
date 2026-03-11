# WezTerm MCP: Rust/rmcp Rewrite

## Overview

Rewrite the wezterm-mcp server from TypeScript to Rust using the `rmcp` crate, mirroring patterns from the `mcp-server-youtube` reference project. All 22 existing tools are ported 1:1 with identical names and schemas.

## Transport

Stdio only. No HTTP/streamable transport.

## Project Structure

```
src/
├── main.rs              # Entry point, stdio transport, tracing init
├── lib.rs               # Module declarations
├── server.rs            # WezTermMcpServer struct, #[tool_router] impl, #[tool_handler] ServerHandler
├── errors.rs            # Error enum (thiserror) + McpError conversion
├── wezterm.rs           # async fn exec(args) → shell out to `wezterm cli`
├── tools.rs             # Re-exports tools/*
└── tools/
    ├── query.rs         # list_panes, list_clients, get_text (+ get_pane_direction)
    ├── pane.rs          # split_pane, spawn, send_text, activate_pane, activate_pane_direction,
    │                    # kill_pane, adjust_pane_size, zoom_pane, move_pane_to_new_tab
    ├── tab.rs           # activate_tab, set_tab_title
    └── window.rs        # set_window_title, rename_workspace
```

## Architecture

### `main.rs`

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let server = WezTermMcpServer::new();
    let service = server.serve(rmcp::transport::stdio()).await?;
    service.waiting().await?;
    Ok(())
}
```

### `server.rs`

- `WezTermMcpServer` struct holds `tool_router: ToolRouter<Self>`
- `#[tool_handler]` impl for `ServerHandler` (server info + capabilities)
- `#[tool_router]` impl block with all 22 `#[tool(...)]` methods
- Each tool method is a thin delegate: extract `Parameters<T>`, call `tools::domain::handler(params).await?`

### `wezterm.rs`

```rust
use tokio::process::Command;
use crate::errors::Error;

pub async fn exec(args: &[&str]) -> Result<String, Error> {
    let output = Command::new("wezterm")
        .arg("cli")
        .args(args)
        .output()
        .await?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::Cli(stderr.trim().to_string()));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
```

### `errors.rs`

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("wezterm cli error: {0}")]
    Cli(String),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl From<Error> for McpError {
    fn from(err: Error) -> Self {
        tracing::error!("{err}");
        McpError::internal_error(err.to_string(), None)
    }
}
```

### `tools/*.rs` pattern

Each file contains parameter structs and async handler functions.

```rust
// tools/pane.rs
use schemars::JsonSchema;
use serde::Deserialize;
use rmcp::model::{CallToolResult, Content};
use crate::errors::Error;
use crate::wezterm;

#[derive(Deserialize, JsonSchema)]
pub struct SplitPaneParams {
    /// Target pane ID. Defaults to the current pane (WEZTERM_PANE).
    pub pane_id: Option<u32>,
    /// Where to place the new pane: left, right, top, bottom. Default: bottom.
    pub direction: Option<SplitDirection>,
    /// Split the entire window instead of the active pane.
    pub top_level: Option<bool>,
    /// Number of cells for the new split.
    pub cells: Option<u32>,
    /// Percentage of available space for the new split.
    pub percent: Option<u32>,
    /// Working directory for the spawned program.
    pub cwd: Option<String>,
    /// Instead of spawning a new command, move this pane into the split.
    pub move_pane_id: Option<u32>,
    /// Command and args to run instead of the default shell.
    pub program: Option<Vec<String>>,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SplitDirection {
    Left,
    Right,
    Top,
    Bottom,
}

pub async fn split_pane(params: SplitPaneParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["split-pane"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    if let Some(ref dir) = params.direction {
        args.push(match dir {
            SplitDirection::Left => "--left",
            SplitDirection::Right => "--right",
            SplitDirection::Top => "--top",
            SplitDirection::Bottom => "--bottom",
        });
    }
    // ... remaining optional args
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}
```

## Tool Inventory (22 tools)

### Query (4)
| Tool | CLI subcommand | Required params |
|------|---------------|-----------------|
| `list_panes` | `list --format json` | none |
| `list_clients` | `list-clients --format json` | none |
| `get_text` | `get-text` | none |
| `get_pane_direction` | `get-pane-direction` | `direction` |

### Pane (9)
| Tool | CLI subcommand | Required params |
|------|---------------|-----------------|
| `split_pane` | `split-pane` | none |
| `spawn` | `spawn` | none |
| `send_text` | `send-text` | `text` |
| `activate_pane` | `activate-pane` | `pane_id` |
| `activate_pane_direction` | `activate-pane-direction` | `direction` |
| `kill_pane` | `kill-pane` | `pane_id` |
| `adjust_pane_size` | `adjust-pane-size` | `direction` |
| `zoom_pane` | `zoom-pane` | none |
| `move_pane_to_new_tab` | `move-pane-to-new-tab` | none |

### Tab (2)
| Tool | CLI subcommand | Required params |
|------|---------------|-----------------|
| `activate_tab` | `activate-tab` | none |
| `set_tab_title` | `set-tab-title` | `title` |

### Window (2)
| Tool | CLI subcommand | Required params |
|------|---------------|-----------------|
| `set_window_title` | `set-window-title` | `title` |
| `rename_workspace` | `rename-workspace` | `new_workspace` |

## Shared Enums

```rust
#[derive(Deserialize, JsonSchema)]
pub enum Direction {
    Up, Down, Left, Right, Next, Prev,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SplitDirection {
    Left, Right, Top, Bottom,
}

#[derive(Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ZoomMode {
    Zoom, Unzoom, Toggle,
}
```

`Direction` is used by `get_pane_direction`, `activate_pane_direction`, and `adjust_pane_size`.

## Dependencies

```toml
[package]
name = "wezterm-mcp"
edition = "2024"

[dependencies]
rmcp = { version = "1.1", features = ["server", "transport-io"] }
tokio = { version = "1", features = ["rt-multi-thread", "macros", "process"] }
thiserror = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
schemars = "1"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
```

## Build

Nix flake with fenix (Rust toolchain) + crane (build framework). Multi-platform: x86_64-linux, aarch64-linux, x86_64-darwin, aarch64-darwin. Dev shell includes clippy, rustfmt, rust-analyzer, cargo-nextest.

## Configuration

None. No CLI args, no env vars. Server starts and listens on stdio. `wezterm` binary is expected on PATH.

## Migration

1. Create fresh Rust project structure alongside existing TS files
2. Remove TS artifacts (`src/*.ts`, `package.json`, `tsconfig.json`, `biome.json`, `build/`, `node_modules/`) after Rust implementation is complete
3. Update `flake.nix` from Node to Rust/crane build
4. Update README

use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::Error;
use crate::tools::Direction;
use crate::wezterm;

// ── list_panes ──

pub async fn list_panes() -> Result<CallToolResult, Error> {
    let output = wezterm::exec(&["list", "--format", "json"]).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── list_clients ──

pub async fn list_clients() -> Result<CallToolResult, Error> {
    let output = wezterm::exec(&["list-clients", "--format", "json"]).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── get_text ──

#[derive(Deserialize, JsonSchema)]
pub struct GetTextParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Starting line. 0 = first screen line, negative = scrollback.
    pub start_line: Option<i32>,
    /// Ending line. 0 = first screen line, negative = scrollback.
    pub end_line: Option<i32>,
    /// Include color/style escape sequences.
    pub escapes: Option<bool>,
}

pub async fn get_text(params: GetTextParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["get-text"];

    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }

    let start_line_str;
    if let Some(n) = params.start_line {
        start_line_str = n.to_string();
        args.extend(["--start-line", &start_line_str]);
    }

    let end_line_str;
    if let Some(n) = params.end_line {
        end_line_str = n.to_string();
        args.extend(["--end-line", &end_line_str]);
    }

    if params.escapes.unwrap_or(false) {
        args.push("--escapes");
    }

    let output = wezterm::exec(&args).await?;
    // Do NOT trim — preserve raw terminal content
    Ok(CallToolResult::success(vec![Content::text(output)]))
}

// ── get_pane_direction ──

#[derive(Deserialize, JsonSchema)]
pub struct GetPaneDirectionParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Direction.
    pub direction: Direction,
}

pub async fn get_pane_direction(params: GetPaneDirectionParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["get-pane-direction"];

    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }

    args.push(params.direction.as_str());

    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

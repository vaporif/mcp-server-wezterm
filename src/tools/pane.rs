use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::Error;
use crate::tools::{Direction, SplitDirection, ZoomMode};
use crate::wezterm;

// ── split_pane ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SplitPaneParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Where to place the new pane relative to the target. Default: bottom.
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

pub async fn split_pane(params: SplitPaneParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["split-pane"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    if let Some(ref dir) = params.direction {
        args.push(dir.as_flag());
    }
    if params.top_level.unwrap_or(false) {
        args.push("--top-level");
    }
    let cells_str;
    if let Some(n) = params.cells {
        cells_str = n.to_string();
        args.extend(["--cells", &cells_str]);
    }
    let percent_str;
    if let Some(n) = params.percent {
        percent_str = n.to_string();
        args.extend(["--percent", &percent_str]);
    }
    if let Some(ref cwd) = params.cwd {
        args.extend(["--cwd", cwd]);
    }
    let move_pane_id_str;
    if let Some(id) = params.move_pane_id {
        move_pane_id_str = id.to_string();
        args.extend(["--move-pane-id", &move_pane_id_str]);
    }
    if let Some(ref program) = params.program
        && !program.is_empty()
    {
        args.push("--");
        args.extend(program.iter().map(String::as_str));
    }
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── spawn ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SpawnParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Domain name.
    pub domain_name: Option<String>,
    /// Window to spawn into. Omit for current window.
    pub window_id: Option<u32>,
    /// Spawn into a new window instead of a new tab.
    pub new_window: Option<bool>,
    /// Working directory for the spawned program.
    pub cwd: Option<String>,
    /// Workspace name for the new window. Requires `new_window`. Default: "default".
    pub workspace: Option<String>,
    /// Command and args to run instead of the default shell.
    pub program: Option<Vec<String>>,
}

pub async fn spawn(params: SpawnParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["spawn"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    if let Some(ref name) = params.domain_name {
        args.extend(["--domain-name", name]);
    }
    let window_id_str;
    if let Some(id) = params.window_id {
        window_id_str = id.to_string();
        args.extend(["--window-id", &window_id_str]);
    }
    if params.new_window.unwrap_or(false) {
        args.push("--new-window");
    }
    if let Some(ref cwd) = params.cwd {
        args.extend(["--cwd", cwd]);
    }
    if let Some(ref ws) = params.workspace {
        args.extend(["--workspace", ws]);
    }
    if let Some(ref program) = params.program
        && !program.is_empty()
    {
        args.push("--");
        args.extend(program.iter().map(String::as_str));
    }
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── send_text ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SendTextParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Text to send.
    pub text: String,
    /// Send directly instead of bracketed paste.
    pub no_paste: Option<bool>,
}

pub async fn send_text(params: SendTextParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["send-text"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    if params.no_paste.unwrap_or(false) {
        args.push("--no-paste");
    }
    args.extend(["--", &params.text]);
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── activate_pane ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActivatePaneParams {
    /// Target pane ID to activate (focus).
    pub pane_id: u32,
}

pub async fn activate_pane(params: ActivatePaneParams) -> Result<CallToolResult, Error> {
    let pane_id_str = params.pane_id.to_string();
    let output = wezterm::exec(&["activate-pane", "--pane-id", &pane_id_str]).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── activate_pane_direction ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActivatePaneDirectionParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Direction.
    pub direction: Direction,
}

pub async fn activate_pane_direction(
    params: ActivatePaneDirectionParams,
) -> Result<CallToolResult, Error> {
    let mut args = vec!["activate-pane-direction"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    args.push(params.direction.as_str());
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── kill_pane ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct KillPaneParams {
    /// Pane ID to kill.
    pub pane_id: u32,
}

pub async fn kill_pane(params: KillPaneParams) -> Result<CallToolResult, Error> {
    let pane_id_str = params.pane_id.to_string();
    let output = wezterm::exec(&["kill-pane", "--pane-id", &pane_id_str]).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── adjust_pane_size ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct AdjustPaneSizeParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Direction.
    pub direction: Direction,
    /// Number of cells to resize by. Default: 1.
    pub amount: Option<u32>,
}

pub async fn adjust_pane_size(params: AdjustPaneSizeParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["adjust-pane-size"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    let amount_str;
    if let Some(n) = params.amount {
        amount_str = n.to_string();
        args.extend(["--amount", &amount_str]);
    }
    args.push(params.direction.as_str());
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── zoom_pane ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ZoomPaneParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Zoom mode. Default: "toggle".
    pub mode: Option<ZoomMode>,
}

pub async fn zoom_pane(params: ZoomPaneParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["zoom-pane"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    let mode = params.mode.unwrap_or(ZoomMode::Toggle);
    args.push(mode.as_flag());
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

// ── move_pane_to_new_tab ──

#[derive(Debug, Deserialize, JsonSchema)]
pub struct MovePaneToNewTabParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Target window. Omit for current window.
    pub window_id: Option<u32>,
    /// Create the tab in a new window.
    pub new_window: Option<bool>,
    /// Workspace name if creating a new window.
    pub workspace: Option<String>,
}

pub async fn move_pane_to_new_tab(params: MovePaneToNewTabParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["move-pane-to-new-tab"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    let window_id_str;
    if let Some(id) = params.window_id {
        window_id_str = id.to_string();
        args.extend(["--window-id", &window_id_str]);
    }
    if params.new_window.unwrap_or(false) {
        args.push("--new-window");
    }
    if let Some(ref ws) = params.workspace {
        args.extend(["--workspace", ws]);
    }
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

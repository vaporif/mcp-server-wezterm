use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::Error;
use crate::wezterm;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetWindowTitleParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Target window ID.
    pub window_id: Option<u32>,
    /// New title.
    pub title: String,
}

pub async fn set_window_title(params: SetWindowTitleParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["set-window-title"];
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
    args.push(&params.title);
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct RenameWorkspaceParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Current workspace name to rename.
    pub workspace: Option<String>,
    /// The new name for the workspace.
    pub new_workspace: String,
}

pub async fn rename_workspace(params: RenameWorkspaceParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["rename-workspace"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    if let Some(ref ws) = params.workspace {
        args.extend(["--workspace", ws]);
    }
    args.push(&params.new_workspace);
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

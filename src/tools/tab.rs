use rmcp::model::{CallToolResult, Content};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::errors::Error;
use crate::wezterm;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ActivateTabParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Target tab ID.
    pub tab_id: Option<u32>,
    /// Tab index (0-based). Negative values count from the right (-1 = last).
    pub tab_index: Option<i32>,
    /// Relative offset. -1 = left, 1 = right, etc.
    pub tab_relative: Option<i32>,
    /// Prevent wrapping when using `tab_relative`.
    pub no_wrap: Option<bool>,
}

pub async fn activate_tab(params: ActivateTabParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["activate-tab"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    let tab_id_str;
    if let Some(id) = params.tab_id {
        tab_id_str = id.to_string();
        args.extend(["--tab-id", &tab_id_str]);
    }
    let tab_index_str;
    if let Some(n) = params.tab_index {
        tab_index_str = n.to_string();
        args.extend(["--tab-index", &tab_index_str]);
    }
    let tab_relative_str;
    if let Some(n) = params.tab_relative {
        tab_relative_str = n.to_string();
        args.extend(["--tab-relative", &tab_relative_str]);
    }
    if params.no_wrap.unwrap_or(false) {
        args.push("--no-wrap");
    }
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SetTabTitleParams {
    /// Target pane ID. Defaults to the current pane (`WEZTERM_PANE`).
    pub pane_id: Option<u32>,
    /// Target tab ID.
    pub tab_id: Option<u32>,
    /// New title.
    pub title: String,
}

pub async fn set_tab_title(params: SetTabTitleParams) -> Result<CallToolResult, Error> {
    let mut args = vec!["set-tab-title"];
    let pane_id_str;
    if let Some(id) = params.pane_id {
        pane_id_str = id.to_string();
        args.extend(["--pane-id", &pane_id_str]);
    }
    let tab_id_str;
    if let Some(id) = params.tab_id {
        tab_id_str = id.to_string();
        args.extend(["--tab-id", &tab_id_str]);
    }
    args.push(&params.title);
    let output = wezterm::exec(&args).await?;
    Ok(CallToolResult::success(vec![Content::text(output.trim())]))
}

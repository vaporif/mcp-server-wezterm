pub mod pane;
pub mod query;
pub mod tab;
pub mod window;

use schemars::JsonSchema;
use serde::Deserialize;

/// Direction for pane navigation. Title-case variants match wezterm CLI expectations.
/// No `rename_all` -- serde defaults match: "Up", "Down", "Left", "Right", "Next", "Prev".
#[derive(Debug, Deserialize, JsonSchema)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Next,
    Prev,
}

impl Direction {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Up => "Up",
            Self::Down => "Down",
            Self::Left => "Left",
            Self::Right => "Right",
            Self::Next => "Next",
            Self::Prev => "Prev",
        }
    }
}

/// Split direction. Lowercase because these map to CLI flags (--left, --right, etc.).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum SplitDirection {
    Left,
    Right,
    Top,
    Bottom,
}

impl SplitDirection {
    pub const fn as_flag(&self) -> &'static str {
        match self {
            Self::Left => "--left",
            Self::Right => "--right",
            Self::Top => "--top",
            Self::Bottom => "--bottom",
        }
    }
}

/// Zoom mode. Lowercase because these map to CLI flags (--zoom, --unzoom, --toggle).
#[derive(Debug, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ZoomMode {
    Zoom,
    Unzoom,
    Toggle,
}

impl ZoomMode {
    pub const fn as_flag(&self) -> &'static str {
        match self {
            Self::Zoom => "--zoom",
            Self::Unzoom => "--unzoom",
            Self::Toggle => "--toggle",
        }
    }
}

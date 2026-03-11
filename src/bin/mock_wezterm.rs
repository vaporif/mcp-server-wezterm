// Mock `wezterm` binary for E2E testing.
//
// Expects: wezterm cli <subcommand> [args...]
// - Logs received args to $MOCK_WEZTERM_LOG (tab-separated, one line per invocation)
// - Returns canned responses based on subcommand

use std::env;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    // Log invocation if requested
    if let Ok(log_path) = env::var("MOCK_WEZTERM_LOG") {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("failed to open log file");
        writeln!(file, "{}", args.join("\t")).expect("failed to write log");
    }

    if args.first().map(String::as_str) != Some("cli") {
        eprintln!("expected 'cli' as first argument");
        std::process::exit(1);
    }

    let subcommand = args.get(1).map(String::as_str).unwrap_or("");

    match subcommand {
        "list" => {
            print!(
                r#"[{{"window_id":0,"tab_id":0,"tab_title":"default","pane_id":0,"workspace":"default","size":"80x24","cursor_x":0,"cursor_y":0,"cursor_visibility":"Visible","cursor_shape":"Default","title":"mock","cwd":"/tmp"}}]"#
            );
        }
        "list-clients" => {
            print!(
                r#"[{{"username":"testuser","hostname":"testhost","pid":1234,"connected_since":"2025-01-01T00:00:00Z","idle_since":"2025-01-01T00:00:00Z","workspace":"default","focused_pane_id":0}}]"#
            );
        }
        "get-text" => {
            print!("$ hello world\n$ ");
        }
        "get-pane-direction" => {
            print!("1");
        }
        "split-pane" => {
            print!("1");
        }
        "spawn" => {
            print!("2");
        }
        "send-text"
        | "activate-pane"
        | "activate-pane-direction"
        | "kill-pane"
        | "adjust-pane-size"
        | "zoom-pane"
        | "move-pane-to-new-tab"
        | "activate-tab"
        | "set-tab-title"
        | "set-window-title"
        | "rename-workspace" => {}
        _ => {
            eprintln!("mock-wezterm: unknown subcommand: {subcommand}");
            std::process::exit(1);
        }
    }
}

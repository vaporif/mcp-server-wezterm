use std::path::PathBuf;

use rmcp::service::ServiceExt;
use rmcp::transport::TokioChildProcess;
use tempfile::TempDir;
use tokio::process::Command;

struct TestEnv {
    _tmp_dir: TempDir,
    log_path: PathBuf,
    mock_dir: PathBuf,
}

fn setup() -> TestEnv {
    let mock_bin = PathBuf::from(env!("CARGO_BIN_EXE_mock-wezterm"));
    let tmp_dir = TempDir::new().expect("create temp dir");
    let mock_dir = tmp_dir.path().to_path_buf();
    let wezterm_link = mock_dir.join("wezterm");

    std::os::unix::fs::symlink(&mock_bin, &wezterm_link).expect("create symlink");

    let log_path = mock_dir.join("wezterm.log");

    TestEnv {
        _tmp_dir: tmp_dir,
        log_path,
        mock_dir,
    }
}

async fn connect(
    env: &TestEnv,
) -> rmcp::service::RunningService<rmcp::service::RoleClient, ()> {
    let server_bin = env!("CARGO_BIN_EXE_mcp-server-wezterm");
    let path_var = format!(
        "{}:{}",
        env.mock_dir.display(),
        std::env::var("PATH").unwrap()
    );
    let log_str = env.log_path.display().to_string();

    let mut cmd = Command::new(server_bin);
    cmd.env("PATH", &path_var)
        .env("MOCK_WEZTERM_LOG", &log_str);

    let transport = TokioChildProcess::new(cmd).expect("create transport");

    <() as ServiceExt<rmcp::service::RoleClient>>::serve((), transport)
        .await
        .expect("initialize MCP client")
}

fn extract_text(result: &rmcp::model::CallToolResult) -> String {
    let json = serde_json::to_value(&result.content[0]).unwrap();
    json["text"].as_str().unwrap_or("").to_string()
}

fn read_log(env: &TestEnv) -> Vec<String> {
    std::fs::read_to_string(&env.log_path)
        .unwrap_or_default()
        .lines()
        .map(String::from)
        .collect()
}

fn call_params(
    name: &str,
    args: Option<serde_json::Value>,
) -> rmcp::model::CallToolRequestParams {
    let params = rmcp::model::CallToolRequestParams::new(name.to_string());
    match args.and_then(|v| v.as_object().cloned()) {
        Some(obj) => params.with_arguments(obj),
        None => params,
    }
}

// ── Tests ──

#[tokio::test]
async fn test_list_tools_returns_all_17() {
    let env = setup();
    let client = connect(&env).await;

    let tools = client.list_all_tools().await.expect("list tools");
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_ref()).collect();

    assert_eq!(tools.len(), 17, "expected 17 tools, got: {names:?}");

    let expected = [
        "list_panes",
        "list_clients",
        "get_text",
        "get_pane_direction",
        "split_pane",
        "spawn",
        "send_text",
        "activate_pane",
        "activate_pane_direction",
        "kill_pane",
        "adjust_pane_size",
        "zoom_pane",
        "move_pane_to_new_tab",
        "activate_tab",
        "set_tab_title",
        "set_window_title",
        "rename_workspace",
    ];
    for name in expected {
        assert!(names.contains(&name), "missing tool: {name}");
    }

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_list_panes() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params("list_panes", None))
        .await
        .expect("call list_panes");

    let text = extract_text(&result);
    assert!(text.contains("pane_id"), "response: {text}");
    assert!(text.contains("workspace"), "response: {text}");

    let log = read_log(&env);
    assert!(log[0].contains("list"), "log: {:?}", log[0]);
    assert!(log[0].contains("--format\tjson"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_list_clients() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params("list_clients", None))
        .await
        .expect("call list_clients");

    let text = extract_text(&result);
    assert!(text.contains("username"), "response: {text}");

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_get_text_with_params() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params(
            "get_text",
            Some(serde_json::json!({
                "pane_id": 5,
                "start_line": -10,
                "end_line": 0
            })),
        ))
        .await
        .expect("call get_text");

    let text = extract_text(&result);
    assert!(text.contains("hello world"), "response: {text}");

    let log = read_log(&env);
    assert!(log[0].contains("get-text"), "log: {:?}", log[0]);
    assert!(log[0].contains("--pane-id\t5"), "log: {:?}", log[0]);
    assert!(
        log[0].contains("--start-line\t-10"),
        "log: {:?}",
        log[0]
    );
    assert!(log[0].contains("--end-line\t0"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_get_pane_direction() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params(
            "get_pane_direction",
            Some(serde_json::json!({
                "pane_id": 0,
                "direction": "Right"
            })),
        ))
        .await
        .expect("call get_pane_direction");

    let text = extract_text(&result);
    assert_eq!(text.trim(), "1");

    let log = read_log(&env);
    assert!(log[0].contains("get-pane-direction"), "log: {:?}", log[0]);
    assert!(log[0].contains("Right"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_split_pane() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params(
            "split_pane",
            Some(serde_json::json!({
                "pane_id": 0,
                "direction": "left",
                "percent": 30
            })),
        ))
        .await
        .expect("call split_pane");

    let text = extract_text(&result);
    assert_eq!(text.trim(), "1");

    let log = read_log(&env);
    assert!(log[0].contains("split-pane"), "log: {:?}", log[0]);
    assert!(log[0].contains("--left"), "log: {:?}", log[0]);
    assert!(log[0].contains("--percent\t30"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_spawn_with_program() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params(
            "spawn",
            Some(serde_json::json!({
                "new_window": true,
                "cwd": "/tmp",
                "program": ["bash", "-c", "echo hi"]
            })),
        ))
        .await
        .expect("call spawn");

    let text = extract_text(&result);
    assert_eq!(text.trim(), "2");

    let log = read_log(&env);
    assert!(log[0].contains("spawn"), "log: {:?}", log[0]);
    assert!(log[0].contains("--new-window"), "log: {:?}", log[0]);
    assert!(log[0].contains("--cwd\t/tmp"), "log: {:?}", log[0]);
    assert!(
        log[0].contains("--\tbash\t-c\techo hi"),
        "log: {:?}",
        log[0]
    );

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_send_text() {
    let env = setup();
    let client = connect(&env).await;

    let result = client
        .call_tool(call_params(
            "send_text",
            Some(serde_json::json!({
                "pane_id": 3,
                "text": "ls -la\n",
                "no_paste": true
            })),
        ))
        .await
        .expect("call send_text");

    assert!(result.content.is_empty() || extract_text(&result).is_empty());

    let log = read_log(&env);
    assert!(log[0].contains("send-text"), "log: {:?}", log[0]);
    assert!(log[0].contains("--pane-id\t3"), "log: {:?}", log[0]);
    assert!(log[0].contains("--no-paste"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_activate_pane() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "activate_pane",
            Some(serde_json::json!({"pane_id": 7})),
        ))
        .await
        .expect("call activate_pane");

    let log = read_log(&env);
    assert!(log[0].contains("activate-pane"), "log: {:?}", log[0]);
    assert!(log[0].contains("--pane-id\t7"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_kill_pane() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "kill_pane",
            Some(serde_json::json!({"pane_id": 2})),
        ))
        .await
        .expect("call kill_pane");

    let log = read_log(&env);
    assert!(log[0].contains("kill-pane"), "log: {:?}", log[0]);
    assert!(log[0].contains("--pane-id\t2"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_zoom_pane() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "zoom_pane",
            Some(serde_json::json!({"pane_id": 1, "mode": "zoom"})),
        ))
        .await
        .expect("call zoom_pane");

    let log = read_log(&env);
    assert!(log[0].contains("zoom-pane"), "log: {:?}", log[0]);
    assert!(log[0].contains("--zoom"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_activate_tab() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "activate_tab",
            Some(serde_json::json!({"tab_index": 2, "no_wrap": true})),
        ))
        .await
        .expect("call activate_tab");

    let log = read_log(&env);
    assert!(log[0].contains("activate-tab"), "log: {:?}", log[0]);
    assert!(log[0].contains("--tab-index\t2"), "log: {:?}", log[0]);
    assert!(log[0].contains("--no-wrap"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_set_tab_title() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "set_tab_title",
            Some(serde_json::json!({"title": "my-tab", "tab_id": 1})),
        ))
        .await
        .expect("call set_tab_title");

    let log = read_log(&env);
    assert!(log[0].contains("set-tab-title"), "log: {:?}", log[0]);
    assert!(log[0].contains("--tab-id\t1"), "log: {:?}", log[0]);
    assert!(log[0].contains("my-tab"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_set_window_title() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "set_window_title",
            Some(serde_json::json!({"title": "my-window", "window_id": 0})),
        ))
        .await
        .expect("call set_window_title");

    let log = read_log(&env);
    assert!(log[0].contains("set-window-title"), "log: {:?}", log[0]);
    assert!(log[0].contains("my-window"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

#[tokio::test]
async fn test_rename_workspace() {
    let env = setup();
    let client = connect(&env).await;

    client
        .call_tool(call_params(
            "rename_workspace",
            Some(serde_json::json!({
                "workspace": "old-name",
                "new_workspace": "new-name"
            })),
        ))
        .await
        .expect("call rename_workspace");

    let log = read_log(&env);
    assert!(
        log[0].contains("rename-workspace"),
        "log: {:?}",
        log[0]
    );
    assert!(
        log[0].contains("--workspace\told-name"),
        "log: {:?}",
        log[0]
    );
    assert!(log[0].contains("new-name"), "log: {:?}", log[0]);

    client.cancel().await.expect("shutdown");
}

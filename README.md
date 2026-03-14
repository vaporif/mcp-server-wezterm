# mcp-server-wezterm

MCP server that exposes [WezTerm](https://wezfurlong.org/wezterm/) terminal control via the [Model Context Protocol](https://modelcontextprotocol.io/).

## Usage

### Claude Desktop / Claude Code

**With [uvx](https://docs.astral.sh/uv/) (recommended):**

```json
{
  "mcpServers": {
    "wezterm": {
      "command": "uvx",
      "args": ["mcp-server-wezterm"]
    }
  }
}
```

```bash
claude mcp add wezterm -- uvx mcp-server-wezterm
```

**With [rvx](https://github.com/vaporif/rvx):**

```json
{
  "mcpServers": {
    "wezterm": {
      "command": "rvx",
      "args": ["mcp-server-wezterm"]
    }
  }
}
```

<details>
<summary>Other installation methods</summary>

**With Nix:**

```sh
nix run github:vaporif/mcp-server-wezterm
```

**With cargo:**

```sh
cargo install mcp-server-wezterm
```

</details>

## Tools

### Pane Management

| Tool | Description |
|---|---|
| `list_panes` | List all windows, tabs and panes |
| `get_text` | Read terminal screen/scrollback content |
| `get_pane_direction` | Get adjacent pane ID in a direction |
| `split_pane` | Split a pane (left/right/top/bottom) |
| `activate_pane` | Focus a pane by ID |
| `activate_pane_direction` | Focus adjacent pane by direction |
| `kill_pane` | Kill a pane |
| `adjust_pane_size` | Resize a pane directionally |
| `zoom_pane` | Zoom/unzoom/toggle a pane |
| `move_pane_to_new_tab` | Move a pane into a new tab |

### Tab & Window

| Tool | Description |
|---|---|
| `activate_tab` | Activate a tab by ID, index, or relative offset |
| `set_tab_title` | Change tab title |
| `set_window_title` | Change window title |

### Other

| Tool | Description |
|---|---|
| `list_clients` | List connected clients |
| `spawn` | Spawn a command in a new window or tab |
| `send_text` | Send text to a pane (bracketed paste) |
| `rename_workspace` | Rename a workspace |

## Development

Requires [just](https://github.com/casey/just), [taplo](https://taplo.tamasfe.dev/), and [typos](https://github.com/crate-ci/typos).

```bash
just check    # clippy + test + fmt + taplo + typos
just build    # cargo build
just test     # cargo test
just fmt      # format code and TOML
just lint     # cargo clippy
just deny     # cargo deny check
```

## License

MIT

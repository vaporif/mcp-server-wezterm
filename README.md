# wezterm-mcp

MCP server that exposes [WezTerm](https://wezfurlong.org/wezterm/) terminal control via the [Model Context Protocol](https://modelcontextprotocol.io/).

## Requirements

- Node.js v24+
- WezTerm with `wezterm cli` available on PATH

## Usage

Add to your MCP client configuration (e.g. Claude Code `~/.claude.json`):

```json
{
  "mcpServers": {
    "wezterm": {
      "command": "npx",
      "args": ["-y", "@vaporif/wezterm-mcp"]
    }
  }
}
```

### Nix

```bash
nix run github:vaporif/wezterm-mcp
```

## Tools

| Tool | Description |
|---|---|
| `list_panes` | List all windows, tabs and panes |
| `list_clients` | List connected clients |
| `get_text` | Read terminal screen/scrollback content |
| `get_pane_direction` | Get adjacent pane ID in a direction |
| `split_pane` | Split a pane (left/right/top/bottom) |
| `spawn` | Spawn a command in a new window or tab |
| `send_text` | Send text to a pane (bracketed paste) |
| `activate_pane` | Focus a pane by ID |
| `activate_pane_direction` | Focus adjacent pane by direction |
| `kill_pane` | Kill a pane |
| `adjust_pane_size` | Resize a pane directionally |
| `zoom_pane` | Zoom/unzoom/toggle a pane |
| `move_pane_to_new_tab` | Move a pane into a new tab |
| `activate_tab` | Activate a tab by ID, index, or relative offset |
| `set_tab_title` | Change tab title |
| `set_window_title` | Change window title |
| `rename_workspace` | Rename a workspace |

## Development

```bash
npm run dev    # Run directly via tsx
npm run build  # Compile to build/
npm start      # Run compiled output
nix develop    # Nix devShell with node + wezterm
```

## License

MIT

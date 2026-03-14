# Template Integration Design

Bring production-ready CI/CD, distribution, and developer tooling from
`template-rust-mcp-server` into `mcp-server-wezterm`. Stdio-only — no HTTP,
no Smithery, no Docker.

## Scope

### Add

| File | Purpose |
|---|---|
| `.github/workflows/check.yml` | CI: clippy, test, fmt, cross-compile, maturin wheels, typos, taplo, gitleaks, nix, cargo-deny |
| `.github/workflows/release.yaml` | Release: GitHub binaries, crates.io, PyPI wheels on `v*` tags |
| `justfile` | Dev tasks: build, check, fmt, test, lint, deny, release-build |
| `pyproject.toml` | Maturin config for uvx/PyPI distribution |
| `deny.toml` | cargo-deny configuration |
| `typos.toml` | Spell checker configuration |

### Modify

| File | Change |
|---|---|
| `Cargo.toml` | Add license, keywords, description, repository metadata for crates.io |
| `README.md` | Rewrite: install methods (uvx, cargo, nix), Claude Desktop config, tool list |

### Remove

| File | Reason |
|---|---|
| `.github/workflows/ci.yml` | Replaced by check.yml |
| `.github/workflows/publish.yml` | Replaced by release.yaml |

## Excluded

- HTTP transport / `axum` / `transport-streamable-http-server` — not needed
- `smithery.yaml` — requires HTTP
- `Dockerfile` / `.dockerignore` — not needed
- `setup.sh` — template-specific
- `CONTRIBUTING.md` — not needed
- Any changes to `src/` — server code stays as-is

## Notes

- Existing CI (`ci.yml`, `publish.yml`) is Node.js-based leftover from pre-Rust rewrite.
  There is no working Rust CI — the new workflows are built from scratch.
- The project has two binaries: `mcp-server-wezterm` (default) and `mock-wezterm` (test
  helper). Cross-compile and release jobs must only ship `mcp-server-wezterm`.
  Maturin `pyproject.toml` must specify the correct binary name.
- PyPI name `mcp-server-wezterm` availability must be verified before first release.
  Trusted publisher must be configured in PyPI project settings.
- Release workflow uses `v*` tag push trigger (not GitHub Release events). The workflow
  creates the GitHub Release automatically.

## Details

### CI (check.yml)

Adapted from template. Triggers on push to main and PRs.

Jobs:
- **clippy**: `cargo clippy --workspace -- -D warnings`
- **test**: `cargo test --workspace`
- **fmt**: `cargo fmt --all -- --check`
- **cross-compile**: matrix of 5 targets (linux musl x86/arm, macOS x86/arm, windows x86).
  Only builds `mcp-server-wezterm` binary (exclude `mock-wezterm`).
- **maturin**: build wheels for `mcp-server-wezterm` binary only
- **typos**: spell check
- **taplo**: TOML validation
- **gitleaks**: secret scanning
- **nix**: `nix flake check` + `nix build`. Use `DeterminateSystems/magic-nix-cache-action`
  for binary caching to avoid timeouts.
- **cargo-deny**: dependency audit

### Release (release.yaml)

Triggered on `v*` tag push. Jobs:
- Cross-compiled binary uploads to GitHub Releases (auto-created)
- `cargo publish` to crates.io
- Maturin wheel builds + PyPI publish (trusted publishing)

Only ships `mcp-server-wezterm` binary.

### Justfile

```
build       cargo build
check       clippy + test + fmt + taplo + typos
fmt         cargo fmt + taplo fmt
test        cargo test --workspace
lint        cargo clippy -- -D warnings
deny        cargo deny check
release-build  cargo build --release
```

### pyproject.toml

Maturin build backend with `bindings = "bin"`. Package name: `mcp-server-wezterm`.
Must specify `[tool.maturin] binaries = ["mcp-server-wezterm"]` to exclude `mock-wezterm`.

### deny.toml

Permissive license allowlist: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Zlib.
Advisory database enabled. No explicit bans.

### Cargo.toml additions

- `license = "MIT"`
- `description = "MCP server for WezTerm terminal"`
- `repository = "https://github.com/vaporif/mcp-server-wezterm"`
- `keywords = ["mcp", "wezterm"]`
- Release profile: `strip = true`, `lto = true`, `opt-level = "z"`, `codegen-units = 1`

### README.md

Structure:
1. Title + one-line description
2. Install methods: uvx, cargo install, nix
3. Usage with Claude Desktop (JSON config snippet)
4. Available tools (all 17, grouped by category)
5. Development (just commands)
6. License

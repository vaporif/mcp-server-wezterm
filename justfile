default:
    @just --list

build:
    cargo build

check: lint test fmt-check taplo-check typos-check

fmt:
    cargo fmt --all
    taplo fmt

fmt-check:
    cargo fmt --all -- --check

taplo-check:
    taplo check

typos-check:
    typos

test:
    cargo test --workspace --features test-mock

lint:
    cargo clippy --workspace --features test-mock -- -D warnings

deny:
    cargo deny check

release-build:
    cargo build --release

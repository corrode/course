# Convenience targets for working on the course locally.
# The course is a regular Cargo project, so every target here is a
# thin wrapper. `make help` shows the full list.

.PHONY: help dev run build test fmt clippy check clean examples typos links ci fmt-check

help:
	@echo "make dev      - run the server with auto-reload (needs cargo-watch)"
	@echo "make run      - run the server once"
	@echo "make build    - cargo build"
	@echo "make test     - cargo test (library + binaries)"
	@echo "make check    - cargo check"
	@echo "make fmt      - cargo fmt"
	@echo "make clippy   - cargo clippy"
	@echo "make examples - verify every exercise chapter (needs clippy)"
	@echo "make typos    - spell check (needs typos-cli)"
	@echo "make links    - link check (needs lychee)"
	@echo "make ci       - run the full CI suite locally"
	@echo "make clean    - cargo clean"

# Rebuild + restart the server on every change. Install once with:
#   cargo install cargo-watch
# The -q flag keeps cargo-watch's own banner quiet so the actual
# server logs stay readable.
dev:
	cargo watch -q -c -x 'run --bin server'

run:
	cargo run --bin server

build:
	cargo build

# The exercise examples include deliberately-broken teaching files, so we
# scope the default test/clippy targets to the library and binaries (CI
# checks the examples separately via `make examples`).
test:
	cargo test --lib --bins

check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy --lib --bins -- -D warnings

# Verify every exercise chapter is in its intended state (compiles + lints
# clean, except the chapters that are meant to fail to compile).
examples:
	./scripts/check-examples.sh

# Spell check. Install once with: cargo install typos-cli
typos:
	typos

# Link check. Install once with: cargo install lychee
links:
	lychee --root-dir . --config ./lychee.toml README.md docs examples static/cheatsheet.md

# Everything CI runs, in one shot.
ci: fmt-check clippy build test examples typos links

fmt-check:
	cargo fmt --all --check

clean:
	cargo clean

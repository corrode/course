# Convenience targets for working on the course locally.
# The course is a regular Cargo project, so every target here is a
# thin wrapper. `make help` shows the full list.

.PHONY: help dev run build test fmt clippy check clean examples solutions js-check typos links ci fmt-check workspace-check

help:
	@echo "make dev      - run the server with auto-reload (needs cargo-watch)"
	@echo "make run      - run the server once"
	@echo "make build    - cargo build --workspace"
	@echo "make test     - workspace library, binary, and doc tests"
	@echo "make check    - cargo check --workspace"
	@echo "make fmt      - cargo fmt --all"
	@echo "make clippy   - workspace library + binary lints"
	@echo "make examples - verify every exercise chapter (needs clippy)"
	@echo "make solutions - verify every solution (needs rustc)"
	@echo "make workspace-check - build + smoke-test CLI/server (needs python3, rustfmt, clippy)"
	@echo "make js-check  - rebuild and verify current JavaScript bundles"
	@echo "make typos     - spell check (needs typos-cli)"
	@echo "make links     - link check (needs lychee)"
	@echo "make ci        - run all locally reproducible CI checks"
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
	cargo build --workspace --lib --bins

# The exercise examples include deliberately-broken teaching files, so we
# scope infrastructure checks to workspace libraries and binaries, plus doc
# tests. CI checks the examples separately via `make examples`.
test:
	cargo test --workspace --lib --bins
	cargo test --workspace --doc

check:
	cargo check --workspace --lib --bins

fmt:
	cargo fmt --all

clippy:
	cargo clippy --workspace --lib --bins -- -D warnings

workspace-check: build
	python3 scripts/check-workspace.py

# Verify every exercise chapter is in its intended state (compiles + lints
# clean, except the chapters that are meant to fail to compile).
examples:
	./scripts/check-examples.sh

# Verify every solution in solutions/ compiles and its tests pass.
# REQUIRE_COMPLETE=1 also fails if any exercise step lacks a solution.
solutions:
	REQUIRE_COMPLETE=1 ./scripts/check-solutions.sh

# Snapshot browser assets, rebuild them, and fail if the generated output
# differs. This works before or after the changes are committed.
js-check:
	@tmpdir=$$(mktemp -d); \
	trap 'rm -rf "$$tmpdir"' EXIT; \
	cp -R static/dist "$$tmpdir/dist" && \
	cp static/js/htmx.min.js "$$tmpdir/htmx.min.js" && \
	npm test && \
	npm run build:js && \
	diff -ru "$$tmpdir/dist" static/dist && \
	cmp "$$tmpdir/htmx.min.js" static/js/htmx.min.js

# Spell check. Install once with: cargo install typos-cli
typos:
	typos

# Link check. Install once with: cargo install lychee
links:
	lychee --root-dir . --config ./lychee.toml README.md docs examples static/cheatsheet.md

# Everything from CI that is reproducible locally without publishing or
# deploying.
ci: fmt-check clippy build test workspace-check examples solutions js-check typos links

fmt-check:
	cargo fmt --all --check

clean:
	cargo clean

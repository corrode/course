# Convenience targets for working on the course locally.
# The course is a regular Cargo project, so every target here is a
# thin wrapper. `make help` shows the full list.

.PHONY: help dev run build test fmt clippy check clean

help:
	@echo "make dev     - run the server with auto-reload (needs cargo-watch)"
	@echo "make run     - run the server once"
	@echo "make build   - cargo build"
	@echo "make test    - cargo test"
	@echo "make check   - cargo check"
	@echo "make fmt     - cargo fmt"
	@echo "make clippy  - cargo clippy"
	@echo "make clean   - cargo clean"

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

test:
	cargo test

check:
	cargo check

fmt:
	cargo fmt

clippy:
	cargo clippy

clean:
	cargo clean

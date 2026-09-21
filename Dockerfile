# Multi-stage Dockerfile for the course server.
#
# Stage 1 builds the `server` binary against Debian's glibc.
# Stage 2 is a slim Debian image with just the binary and the runtime
# assets it needs (migrations, examples, solutions, static files).
#
# We use Debian (not Alpine/musl) so we can avoid cross-compilation
# headaches and keep dynamic linking with the same glibc the runtime has.

FROM rust:1.98.1-trixie AS builder

WORKDIR /app

# Build-time git metadata for the footer. The build context excludes `.git`
# (see .dockerignore), so CI passes the branch and full revision explicitly.
# The server still accepts runtime `SOURCE_COMMIT` / `COOLIFY_BRANCH` values
# for compatibility, then falls back to these baked-in values. Production only
# deploys `main`, so use that as the branch default for local builds.
ARG GIT_BRANCH=main
ARG GIT_HASH=
ENV GIT_BRANCH=${GIT_BRANCH} \
    GIT_HASH=${GIT_HASH}

# Cache dependencies separately from source. Copy just the manifests
# first, build dummy workspace sources so cargo compiles the server's deps,
# then overwrite with the real source. Failures must fail the image build.
COPY Cargo.toml Cargo.lock ./
COPY crates/cli/Cargo.toml ./crates/cli/Cargo.toml
COPY crates/course-types/Cargo.toml ./crates/course-types/Cargo.toml
COPY crates/server/Cargo.toml ./crates/server/Cargo.toml
RUN mkdir -p examples crates/cli/src crates/course-types/src crates/server/src \
 && echo "fn main() {}" > examples/workspace_stub.rs \
 && echo "fn main() {}" > crates/cli/src/main.rs \
 && echo "fn main() {}" > build.rs \
 && echo "" > crates/course-types/src/lib.rs \
 && echo "fn main() {}" > crates/server/src/main.rs \
 && echo "" > crates/server/src/lib.rs \
 && echo "fn main() {}" > crates/server/build.rs \
 && cargo build --locked --release --bin server \
 && rm examples/workspace_stub.rs

# Real source.
COPY build.rs ./
COPY crates ./crates
COPY examples ./examples
COPY solutions ./solutions
COPY templates ./templates
COPY migrations ./migrations
COPY static ./static

# Remove dummy workspace artifacts, keeping compiled third-party dependencies.
# This also reruns the server build script with the git metadata above.
RUN cargo clean --release -p course-exercises -p cargo-course -p course-types -p course-server \
 && cargo build --locked --release --bin server

FROM debian:trixie-slim AS runtime

# `ca-certificates` is needed for outbound HTTPS to play.rust-lang.org.
# `curl` is used by Coolify / Docker healthchecks against `/health`.
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/server /app/server
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/examples   /app/examples
COPY --from=builder /app/solutions  /app/solutions
COPY --from=builder /app/static     /app/static

# Persistent data lives here; Coolify mounts a host directory over it.
RUN mkdir -p /app/data

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -fsS "http://127.0.0.1:${PORT:-3000}/health" || exit 1

CMD ["/app/server"]

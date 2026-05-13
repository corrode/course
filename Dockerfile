# Multi-stage Dockerfile for the course server.
#
# Stage 1 builds the `server` binary against Debian's glibc.
# Stage 2 is a slim Debian image with just the binary and the runtime
# assets it needs (migrations, examples, static files).
#
# We use Debian (not Alpine/musl) so we can avoid cross-compilation
# headaches and keep dynamic linking with the same glibc the runtime has.

FROM rust:1-trixie AS builder

WORKDIR /app

# Cache dependencies separately from source. Copy just the manifests
# first, build a dummy main so cargo downloads + compiles deps, then
# overwrite with the real source.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src/bin examples \
 && echo "fn main() {}" > src/bin/server.rs \
 && echo "fn main() {}" > src/bin/cli.rs \
 && echo "" > src/lib.rs \
 && echo "" > build.rs \
 && cargo build --release --bin server || true \
 && rm -rf src

# Real source.
COPY build.rs ./
COPY src ./src
COPY examples ./examples
COPY templates ./templates
COPY migrations ./migrations
COPY static ./static

# Bust cargo's incremental cache for our own crate so the real source
# actually gets compiled (the dummy-main step above leaves stale
# fingerprints otherwise).
RUN touch src/bin/server.rs \
 && cargo build --release --bin server

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
COPY --from=builder /app/static     /app/static

# Persistent data lives here; Coolify mounts a host directory over it.
RUN mkdir -p /app/data

EXPOSE 3000

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
  CMD curl -fsS http://127.0.0.1:3000/health || exit 1

CMD ["/app/server"]

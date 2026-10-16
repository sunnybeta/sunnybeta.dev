# syntax=docker/dockerfile:1

# =============================================================================
#  Sunny Béta — portfolio server
#
#  Multi-stage build: the release binary is compiled against musl so it is
#  fully static, then dropped into a tiny Alpine runtime alongside the
#  hand-written `static/` assets. Askama templates are compiled into the
#  binary, so nothing else needs to ship.
#
#  Build:  docker build -t sunnybeta .
#  Run:    docker run -p 2382:2382 sunnybeta     (PORT env to change)
# =============================================================================

# ---------------------------------------------------------------------------
# Stage 1 — build the release binary (static, musl)
# ---------------------------------------------------------------------------
FROM rust:1.99-alpine AS build

# musl linker + headers, needed for static linking
RUN apk add --no-cache musl-dev

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates

# The cache mounts keep the cargo registry and incremental state between
# builds; `cp` runs inside the same step because neither mount survives
# into the image layer.
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/build/target,sharing=locked \
    cargo build --release \
 && cp target/release/sunnybeta-dev /build/sunnybeta-dev

# ---------------------------------------------------------------------------
# Stage 2 — minimal runtime
# ---------------------------------------------------------------------------
FROM alpine:3.22

# Unprivileged runtime user (own -H: no home dir needed)
RUN adduser -D -H -u 10001 site

WORKDIR /app

COPY --from=build /build/sunnybeta-dev /app/sunnybeta-dev
COPY static /app/static

# The server resolves `static/` relative to its working directory, and
# templates are baked into the binary at build time — /app is self-contained.
USER site

EXPOSE 2382
ENV PORT=2382

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s \
	CMD wget -q --spider "http://127.0.0.1:${PORT}/retro" || exit 1

ENTRYPOINT ["/app/sunnybeta-dev"]

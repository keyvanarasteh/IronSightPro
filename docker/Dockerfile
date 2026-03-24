# ─── Stage 1: Builder ────────────────────────────────────────────────────────
FROM rust:1.85-slim-bookworm AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /build
COPY . .

RUN cargo build --release --bin ironsight \
    && strip target/release/ironsight

# ─── Stage 2: Runtime ────────────────────────────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    procps \
    && rm -rf /var/lib/apt/lists/*

# Create report directory
RUN mkdir -p /var/lib/ironsight/reports

# Copy binary
COPY --from=builder /build/target/release/ironsight /usr/local/bin/ironsight

# Copy default config
COPY ironsight.example.toml /etc/ironsight/config.toml

# Security: don't run as root inside container by default
# Override with --user root or --privileged for full features
RUN useradd -r -s /bin/false ironsight

ENTRYPOINT ["ironsight"]
CMD ["--top", "20"]

# ============================================
# IronSight EDR — Multi-Stage Dockerfile
# ============================================
# Stage 1: Builder  — Rust derlemesi
# Stage 2: Runtime  — Minimal production image
# ============================================

# ── Stage 1: Builder ──────────────────────────
FROM rust:1.94-bookworm AS builder

WORKDIR /build

# Sistem bağımlılıkları (build-time)
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    cmake \
    && rm -rf /var/lib/apt/lists/*

# Dependency cache layer — sadece Cargo dosyaları kopyala
COPY Cargo.toml Cargo.lock ./
COPY crates/ironsight-core/Cargo.toml crates/ironsight-core/Cargo.toml
COPY crates/ironsight-security/Cargo.toml crates/ironsight-security/Cargo.toml
COPY crates/ironsight-network/Cargo.toml crates/ironsight-network/Cargo.toml
COPY crates/ironsight-memory/Cargo.toml crates/ironsight-memory/Cargo.toml
COPY crates/ironsight-kernel/Cargo.toml crates/ironsight-kernel/Cargo.toml
COPY crates/ironsight-heuristic/Cargo.toml crates/ironsight-heuristic/Cargo.toml
COPY crates/ironsight-response/Cargo.toml crates/ironsight-response/Cargo.toml
COPY crates/ironsight-report/Cargo.toml crates/ironsight-report/Cargo.toml
COPY crates/ironsight-service/Cargo.toml crates/ironsight-service/Cargo.toml

# Dummy lib.rs dosyaları oluştur (dependency cache)
RUN mkdir -p crates/ironsight-core/src && echo "pub fn _dummy() {}" > crates/ironsight-core/src/lib.rs && \
    mkdir -p crates/ironsight-security/src && echo "pub fn _dummy() {}" > crates/ironsight-security/src/lib.rs && \
    mkdir -p crates/ironsight-network/src && echo "pub fn _dummy() {}" > crates/ironsight-network/src/lib.rs && \
    mkdir -p crates/ironsight-memory/src && echo "pub fn _dummy() {}" > crates/ironsight-memory/src/lib.rs && \
    mkdir -p crates/ironsight-kernel/src && echo "pub fn _dummy() {}" > crates/ironsight-kernel/src/lib.rs && \
    mkdir -p crates/ironsight-heuristic/src && echo "pub fn _dummy() {}" > crates/ironsight-heuristic/src/lib.rs && \
    mkdir -p crates/ironsight-response/src && echo "pub fn _dummy() {}" > crates/ironsight-response/src/lib.rs && \
    mkdir -p crates/ironsight-report/src && echo "pub fn _dummy() {}" > crates/ironsight-report/src/lib.rs && \
    mkdir -p crates/ironsight-service/src && echo "fn main() {}" > crates/ironsight-service/src/main.rs

# Dependency ön-derleme (cache edilir)
RUN cargo build --release --package ironsight-service 2>/dev/null || true
RUN rm -rf crates/*/src

# Gerçek kaynak kodunu kopyala
COPY crates/ crates/

# Final derleme
RUN cargo build --release --package ironsight-service && \
    strip target/release/ironsight-service

# ── Stage 2: Runtime ──────────────────────────
FROM debian:bookworm-slim AS runtime

LABEL maintainer="Keyvan Arasteh"
LABEL description="IronSight EDR — Endpoint Detection & Response"
LABEL version="0.1.0"

# Runtime bağımlılıkları
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    procps \
    iproute2 \
    && rm -rf /var/lib/apt/lists/*

# Non-root kullanıcı (privilege escalation için capability verilecek)
RUN groupadd -r ironsight && useradd -r -g ironsight -d /opt/ironsight -s /sbin/nologin ironsight

# Dizin yapısı
RUN mkdir -p /opt/ironsight/bin \
             /etc/ironsight \
             /var/lib/ironsight/reports \
             /var/lib/ironsight/dumps \
             /var/log/ironsight && \
    chown -R ironsight:ironsight /var/lib/ironsight /var/log/ironsight

# Binary kopyala
COPY --from=builder /build/target/release/ironsight-service /opt/ironsight/bin/ironsight

# Default config
COPY --chmod=644 docker/ironsight.toml /etc/ironsight/config.toml

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep -f ironsight || exit 1

# Capabilities (container runtime'da --cap-add gerekir)
# CAP_SYS_PTRACE: Process bellek okuma
# CAP_KILL: Process sinyal gönderme
# CAP_NET_RAW: Ağ tarama

ENV IRONSIGHT_CONFIG=/etc/ironsight/config.toml
ENV IRONSIGHT_LOG_LEVEL=info
ENV RUST_LOG=ironsight=info

WORKDIR /opt/ironsight

ENTRYPOINT ["/opt/ironsight/bin/ironsight"]
CMD ["--config", "/etc/ironsight/config.toml"]

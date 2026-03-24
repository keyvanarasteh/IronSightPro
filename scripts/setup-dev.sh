#!/usr/bin/env bash
# ============================================
# IronSight EDR — Geliştirme Ortamı Kurulumu
# ============================================
# Kullanım: ./scripts/setup-dev.sh
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
err()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Dev Ortam Kurulumu      ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

# ── OS tespiti ─────────────────────────────────
OS="unknown"
if [[ -f /etc/os-release ]]; then
    . /etc/os-release
    OS="$ID"
elif [[ "$(uname)" == "Darwin" ]]; then
    OS="macos"
fi

info "İşletim sistemi: $OS"

# ── Sistem bağımlılıkları ──────────────────────
info "Sistem bağımlılıkları kontrol ediliyor..."

case "$OS" in
    ubuntu|debian|pop)
        info "Debian/Ubuntu paketleri kuruluyor..."
        sudo apt update -qq
        sudo apt install -y --no-install-recommends \
            build-essential pkg-config libssl-dev cmake git \
            clang llvm libbpf-dev
        ok "APT paketleri kuruldu"
        ;;
    arch|manjaro|endeavouros)
        info "Arch paketleri kuruluyor..."
        sudo pacman -Sy --needed --noconfirm \
            base-devel openssl pkg-config cmake git \
            clang llvm libbpf
        ok "Pacman paketleri kuruldu"
        ;;
    fedora|rhel|centos)
        info "Fedora/RHEL paketleri kuruluyor..."
        sudo dnf install -y \
            gcc gcc-c++ openssl-devel pkg-config cmake git \
            clang llvm libbpf-devel
        ok "DNF paketleri kuruldu"
        ;;
    macos)
        info "macOS bağımlılıkları kuruluyor..."
        if ! command -v brew &>/dev/null; then
            warn "Homebrew bulunamadı. Kurulum: https://brew.sh"
        else
            brew install openssl@3 pkg-config cmake
            export OPENSSL_DIR=$(brew --prefix openssl@3)
            export PKG_CONFIG_PATH="$OPENSSL_DIR/lib/pkgconfig"
            ok "Homebrew paketleri kuruldu"
            echo ""
            warn "Shell profile'ınıza ekleyin:"
            echo "  export OPENSSL_DIR=\$(brew --prefix openssl@3)"
            echo "  export PKG_CONFIG_PATH=\"\$OPENSSL_DIR/lib/pkgconfig\""
        fi
        ;;
    *)
        warn "Bilinmeyen OS: $OS — bağımlılıkları manuel kurun"
        ;;
esac

# ── Rust kontrolü ──────────────────────────────
info "Rust kontrolü..."
if ! command -v rustc &>/dev/null; then
    warn "Rust bulunamadı. Kuruluyor..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    ok "Rust kuruldu"
else
    RUST_VER=$(rustc --version | awk '{print $2}')
    ok "Rust: $RUST_VER"
fi

# ── Rust araçları ──────────────────────────────
info "Rust geliştirme araçları kuruluyor..."

# cargo-watch (hot reload)
if ! command -v cargo-watch &>/dev/null; then
    cargo install cargo-watch
    ok "cargo-watch kuruldu"
else
    ok "cargo-watch zaten kurulu"
fi

# cargo-nextest (hızlı test runner)
if ! command -v cargo-nextest &>/dev/null; then
    cargo install cargo-nextest
    ok "cargo-nextest kuruldu"
else
    ok "cargo-nextest zaten kurulu"
fi

# cargo-audit (güvenlik denetimi)
if ! command -v cargo-audit &>/dev/null; then
    cargo install cargo-audit
    ok "cargo-audit kuruldu"
else
    ok "cargo-audit zaten kurulu"
fi

# ── İlk derleme ────────────────────────────────
info "İlk derleme yapılıyor..."
cargo build --workspace 2>&1 | tail -3
ok "Workspace başarıyla derlendi"

# ── Local config ───────────────────────────────
if [[ ! -f "ironsight.toml" ]]; then
    if [[ -f "docker/ironsight.toml" ]]; then
        cp docker/ironsight.toml ironsight.toml
        sed -i 's|/host/proc|/proc|g' ironsight.toml 2>/dev/null || true
        sed -i 's|daemon_mode = true|daemon_mode = false|g' ironsight.toml 2>/dev/null || true
        ok "Local config: ironsight.toml (geliştirme varsayılanları)"
    fi
fi

# ── Sonuç ──────────────────────────────────────
echo ""
echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✅ Geliştirme ortamı hazır!         ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""
echo -e "  Derle:     ${YELLOW}cargo build --workspace${NC}"
echo -e "  Test:      ${YELLOW}cargo test --workspace${NC}"
echo -e "  Çalıştır:  ${YELLOW}sudo cargo run -p ironsight-service${NC}"
echo -e "  Watch:     ${YELLOW}cargo watch -x 'build --workspace'${NC}"
echo -e "  Lint:      ${YELLOW}cargo clippy --workspace${NC}"
echo -e "  Audit:     ${YELLOW}cargo audit${NC}"
echo ""

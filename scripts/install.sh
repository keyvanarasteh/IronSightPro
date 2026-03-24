#!/usr/bin/env bash
# ============================================
# IronSight EDR — Kurulum Scripti
# ============================================
# Kullanım: sudo ./scripts/install.sh
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

INSTALL_DIR="/usr/local/bin"
CONFIG_DIR="/etc/ironsight"
DATA_DIR="/var/lib/ironsight"
LOG_DIR="/var/log/ironsight"
SERVICE_USER="ironsight"

info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
warn()  { echo -e "${YELLOW}[WARN]${NC} $1"; }
err()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# ── Root kontrolü ─────────────────────────────
if [[ $EUID -ne 0 ]]; then
    err "Bu script root olarak çalıştırılmalı: sudo $0"
fi

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     IronSight EDR — Kurulum          ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

# ── Binary kontrolü ───────────────────────────
BINARY="target/release/ironsight-service"
if [[ ! -f "$BINARY" ]]; then
    BINARY="target/release/ironsight"
fi
if [[ ! -f "$BINARY" ]]; then
    warn "Derlenmiş binary bulunamadı. Derleniyor..."
    if ! command -v cargo &>/dev/null; then
        err "Rust/Cargo kurulu değil. Önce: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    fi
    cargo build --release --package ironsight-service
    BINARY="target/release/ironsight"
    [[ ! -f "$BINARY" ]] && BINARY="target/release/ironsight-service"
fi

# ── Dizin oluşturma ───────────────────────────
info "Dizinler oluşturuluyor..."
mkdir -p "$CONFIG_DIR"
mkdir -p "$DATA_DIR/reports"
mkdir -p "$DATA_DIR/dumps"
mkdir -p "$LOG_DIR"
ok "Dizinler oluşturuldu"

# ── Binary kopyalama ──────────────────────────
info "Binary kopyalanıyor..."
cp "$BINARY" "$INSTALL_DIR/ironsight"
chmod 755 "$INSTALL_DIR/ironsight"
strip "$INSTALL_DIR/ironsight" 2>/dev/null || true
ok "Binary: $INSTALL_DIR/ironsight"

# ── Config ─────────────────────────────────────
if [[ ! -f "$CONFIG_DIR/config.toml" ]]; then
    info "Default config oluşturuluyor..."
    if [[ -f "docker/ironsight.toml" ]]; then
        cp docker/ironsight.toml "$CONFIG_DIR/config.toml"
    elif [[ -f "config/ironsight.toml" ]]; then
        cp config/ironsight.toml "$CONFIG_DIR/config.toml"
    else
        warn "Config template bulunamadı, generate-config kullanılacak"
        "$INSTALL_DIR/ironsight" --generate-config > "$CONFIG_DIR/config.toml" 2>/dev/null || true
    fi
    # Docker path'leri düzelt
    sed -i 's|/host/proc|/proc|g' "$CONFIG_DIR/config.toml" 2>/dev/null || true
    ok "Config: $CONFIG_DIR/config.toml"
else
    warn "Config zaten mevcut, dokunulmadı: $CONFIG_DIR/config.toml"
fi

# ── İzinler ────────────────────────────────────
info "İzinler ayarlanıyor..."
chmod 644 "$CONFIG_DIR/config.toml"
chmod 700 "$DATA_DIR/dumps"  # Forensik dump'lar sadece root
chmod 755 "$DATA_DIR/reports"
chmod 755 "$LOG_DIR"
ok "İzinler ayarlandı"

# ── Capabilities ───────────────────────────────
info "Linux capabilities ayarlanıyor..."
if command -v setcap &>/dev/null; then
    setcap cap_sys_ptrace,cap_kill,cap_dac_read_search+ep "$INSTALL_DIR/ironsight"
    ok "Capabilities: CAP_SYS_PTRACE, CAP_KILL, CAP_DAC_READ_SEARCH"
else
    warn "setcap bulunamadı, root olarak çalıştırmanız gerekecek"
fi

# ── Systemd service ────────────────────────────
info "Systemd service oluşturuluyor..."
cat > /etc/systemd/system/ironsight.service << 'EOF'
[Unit]
Description=IronSight EDR Scanner
After=network.target
Documentation=https://github.com/keyvanarasteh/IronSight

[Service]
Type=simple
ExecStart=/usr/local/bin/ironsight --config /etc/ironsight/config.toml
Restart=always
RestartSec=5
User=root
AmbientCapabilities=CAP_SYS_PTRACE CAP_KILL CAP_DAC_READ_SEARCH CAP_NET_RAW
ProtectSystem=strict
ReadWritePaths=/var/lib/ironsight /var/log/ironsight
PrivateTmp=true
NoNewPrivileges=true
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload
ok "Systemd service: ironsight.service"

# ── Sonuç ──────────────────────────────────────
echo ""
echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║     ✅ Kurulum tamamlandı!           ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""
echo -e "  Binary:   ${BLUE}$INSTALL_DIR/ironsight${NC}"
echo -e "  Config:   ${BLUE}$CONFIG_DIR/config.toml${NC}"
echo -e "  Reports:  ${BLUE}$DATA_DIR/reports/${NC}"
echo -e "  Dumps:    ${BLUE}$DATA_DIR/dumps/${NC}"
echo -e "  Logs:     ${BLUE}$LOG_DIR/${NC}"
echo ""
echo -e "  Başlat:   ${YELLOW}sudo systemctl start ironsight${NC}"
echo -e "  Durum:    ${YELLOW}sudo systemctl status ironsight${NC}"
echo -e "  Loglar:   ${YELLOW}sudo journalctl -u ironsight -f${NC}"
echo -e "  Tek sefer:${YELLOW}sudo ironsight${NC}"
echo ""

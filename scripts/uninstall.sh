#!/usr/bin/env bash
# ============================================
# IronSight EDR — Kaldırma Scripti
# ============================================
# Kullanım: sudo ./scripts/uninstall.sh [--purge]
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

PURGE=false
[[ "${1:-}" == "--purge" ]] && PURGE=true

if [[ $EUID -ne 0 ]]; then
    echo -e "${RED}[ERROR]${NC} Root olarak çalıştırın: sudo $0"
    exit 1
fi

echo ""
echo -e "${RED}╔══════════════════════════════════════╗${NC}"
echo -e "${RED}║  IronSight — Kaldırma                ║${NC}"
echo -e "${RED}╚══════════════════════════════════════╝${NC}"
echo ""

if [[ "$PURGE" == true ]]; then
    warn "PURGE modu: Config, raporlar ve dump'lar DA silinecek!"
else
    info "Normal mod: Sadece binary ve service kaldırılacak"
    info "Veriler kalacak. Tamamen silmek için: $0 --purge"
fi
echo ""

read -p "Devam etmek istiyor musunuz? [y/N] " -n 1 -r
echo ""
[[ ! $REPLY =~ ^[Yy]$ ]] && { echo "İptal edildi."; exit 0; }

# ── Service durdur ─────────────────────────────
info "Service durduruluyor..."
if systemctl is-active --quiet ironsight 2>/dev/null; then
    systemctl stop ironsight
    ok "Service durduruldu"
fi
if systemctl is-enabled --quiet ironsight 2>/dev/null; then
    systemctl disable ironsight
    ok "Service devre dışı bırakıldı"
fi
rm -f /etc/systemd/system/ironsight.service
systemctl daemon-reload 2>/dev/null || true
ok "Systemd service kaldırıldı"

# ── Binary kaldır ──────────────────────────────
info "Binary kaldırılıyor..."
rm -f /usr/local/bin/ironsight
ok "Binary kaldırıldı"

# ── Purge: config + data ──────────────────────
if [[ "$PURGE" == true ]]; then
    info "Config kaldırılıyor..."
    rm -rf /etc/ironsight
    ok "Config kaldırıldı"

    info "Raporlar kaldırılıyor..."
    rm -rf /var/lib/ironsight/reports
    ok "Raporlar kaldırıldı"

    info "Forensik dump'lar kaldırılıyor..."
    rm -rf /var/lib/ironsight/dumps
    ok "Dump'lar kaldırıldı"

    info "Data dizini kaldırılıyor..."
    rm -rf /var/lib/ironsight
    ok "Data dizini kaldırıldı"

    info "Loglar kaldırılıyor..."
    rm -rf /var/log/ironsight
    ok "Loglar kaldırıldı"
else
    warn "Config korundu:   /etc/ironsight/"
    warn "Raporlar korundu: /var/lib/ironsight/reports/"
    warn "Dump'lar korundu: /var/lib/ironsight/dumps/"
    warn "Loglar korundu:   /var/log/ironsight/"
fi

# ── Docker temizlik ────────────────────────────
if command -v docker &>/dev/null; then
    if docker ps -a --format '{{.Names}}' 2>/dev/null | grep -q "ironsight"; then
        info "Docker container'lar durduruluyor..."
        docker compose down 2>/dev/null || true
        ok "Docker container'lar kaldırıldı"
    fi
fi

echo ""
echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✅ IronSight kaldırıldı             ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""

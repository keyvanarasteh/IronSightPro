#!/usr/bin/env bash
# ============================================
# IronSight EDR — Yedekleme Scripti
# ============================================
# Kullanım: sudo ./scripts/backup.sh [hedef_dizin]
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

BACKUP_DIR="${1:-/tmp/ironsight-backup}"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
ARCHIVE_NAME="ironsight-backup-${TIMESTAMP}.tar.gz"

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Yedekleme               ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

STAGING="$BACKUP_DIR/staging-$TIMESTAMP"
mkdir -p "$STAGING"

# ── Config ─────────────────────────────────────
info "Config yedekleniyor..."
if [[ -d /etc/ironsight ]]; then
    cp -r /etc/ironsight "$STAGING/config"
    ok "Config: $(ls /etc/ironsight/ | wc -l) dosya"
else
    warn "Config dizini bulunamadı"
fi

# ── Raporlar ───────────────────────────────────
info "Raporlar yedekleniyor..."
if [[ -d /var/lib/ironsight/reports ]]; then
    COUNT=$(find /var/lib/ironsight/reports -type f | wc -l)
    cp -r /var/lib/ironsight/reports "$STAGING/reports"
    ok "Raporlar: $COUNT dosya"
else
    warn "Rapor dizini bulunamadı"
fi

# ── Forensik Dump'lar ──────────────────────────
info "Forensik dump'lar yedekleniyor..."
if [[ -d /var/lib/ironsight/dumps ]]; then
    COUNT=$(find /var/lib/ironsight/dumps -type f | wc -l)
    SIZE=$(du -sh /var/lib/ironsight/dumps | awk '{print $1}')
    if [[ $COUNT -gt 0 ]]; then
        cp -r /var/lib/ironsight/dumps "$STAGING/dumps"
        ok "Dump'lar: $COUNT dosya ($SIZE)"
    else
        warn "Dump dizini boş"
    fi
else
    warn "Dump dizini bulunamadı"
fi

# ── Loglar ─────────────────────────────────────
info "Loglar yedekleniyor..."
if [[ -d /var/log/ironsight ]]; then
    cp -r /var/log/ironsight "$STAGING/logs"
    ok "Loglar yedeklendi"
fi

# Systemd journal
if command -v journalctl &>/dev/null; then
    journalctl -u ironsight --since "30 days ago" --no-pager > "$STAGING/journal.log" 2>/dev/null || true
    ok "Systemd journal (son 30 gün)"
fi

# ── Arşivle ────────────────────────────────────
info "Arşivleniyor..."
cd "$BACKUP_DIR"
tar -czf "$ARCHIVE_NAME" -C "$STAGING" .
rm -rf "$STAGING"

FINAL_SIZE=$(du -sh "$BACKUP_DIR/$ARCHIVE_NAME" | awk '{print $1}')

echo ""
echo -e "${GREEN}╔══════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✅ Yedekleme tamamlandı!            ║${NC}"
echo -e "${GREEN}╚══════════════════════════════════════╝${NC}"
echo ""
echo -e "  Arşiv: ${BLUE}$BACKUP_DIR/$ARCHIVE_NAME${NC}"
echo -e "  Boyut: ${YELLOW}$FINAL_SIZE${NC}"
echo ""
echo -e "  Geri yükleme: ${YELLOW}tar -xzf $ARCHIVE_NAME -C /tmp/restore${NC}"
echo ""

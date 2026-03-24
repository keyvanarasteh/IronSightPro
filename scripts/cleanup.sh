#!/usr/bin/env bash
# ============================================
# IronSight EDR — Rapor Temizleme
# ============================================
# Kullanım: sudo ./scripts/cleanup.sh [gün_sayısı]
# Default: 30 günden eski raporları sil
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

MAX_AGE_DAYS="${1:-30}"
REPORT_DIR="/var/lib/ironsight/reports"
DUMP_DIR="/var/lib/ironsight/dumps"
LOG_DIR="/var/log/ironsight"

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Temizlik                ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""
echo -e "  ${YELLOW}$MAX_AGE_DAYS günden${NC} eski dosyalar silinecek"
echo ""

TOTAL_FREED=0

# ── Eski raporlar ──────────────────────────────
echo -e "${BLUE}[Raporlar]${NC}"
if [[ -d "$REPORT_DIR" ]]; then
    BEFORE=$(du -sb "$REPORT_DIR" | awk '{print $1}')
    OLD_REPORTS=$(find "$REPORT_DIR" -type f -mtime +"$MAX_AGE_DAYS" 2>/dev/null)
    COUNT=$(echo "$OLD_REPORTS" | grep -c . 2>/dev/null || echo 0)
    
    if [[ $COUNT -gt 0 ]]; then
        echo "$OLD_REPORTS" | xargs rm -f
        AFTER=$(du -sb "$REPORT_DIR" | awk '{print $1}')
        FREED=$(( BEFORE - AFTER ))
        TOTAL_FREED=$(( TOTAL_FREED + FREED ))
        echo -e "  ${GREEN}✅${NC} $COUNT rapor silindi ($(numfmt --to=iec $FREED 2>/dev/null || echo "${FREED}B"))"
    else
        echo -e "  ${GREEN}✅${NC} Temizlenecek rapor yok"
    fi
fi

# ── Eski dump'lar ──────────────────────────────
echo -e "${BLUE}[Forensik Dump'lar]${NC}"
if [[ -d "$DUMP_DIR" ]]; then
    BEFORE=$(du -sb "$DUMP_DIR" | awk '{print $1}')
    OLD_DUMPS=$(find "$DUMP_DIR" -type f -mtime +"$MAX_AGE_DAYS" 2>/dev/null)
    COUNT=$(echo "$OLD_DUMPS" | grep -c . 2>/dev/null || echo 0)
    
    if [[ $COUNT -gt 0 ]]; then
        echo "$OLD_DUMPS" | xargs rm -f
        AFTER=$(du -sb "$DUMP_DIR" | awk '{print $1}')
        FREED=$(( BEFORE - AFTER ))
        TOTAL_FREED=$(( TOTAL_FREED + FREED ))
        echo -e "  ${GREEN}✅${NC} $COUNT dump silindi ($(numfmt --to=iec $FREED 2>/dev/null || echo "${FREED}B"))"
    else
        echo -e "  ${GREEN}✅${NC} Temizlenecek dump yok"
    fi
fi

# ── Eski loglar ────────────────────────────────
echo -e "${BLUE}[Loglar]${NC}"
if [[ -d "$LOG_DIR" ]]; then
    BEFORE=$(du -sb "$LOG_DIR" | awk '{print $1}')
    find "$LOG_DIR" -name "*.log" -mtime +"$MAX_AGE_DAYS" -delete 2>/dev/null
    find "$LOG_DIR" -name "*.log.gz" -mtime +"$MAX_AGE_DAYS" -delete 2>/dev/null
    AFTER=$(du -sb "$LOG_DIR" | awk '{print $1}')
    FREED=$(( BEFORE - AFTER ))
    TOTAL_FREED=$(( TOTAL_FREED + FREED ))
    echo -e "  ${GREEN}✅${NC} Eski loglar temizlendi ($(numfmt --to=iec $FREED 2>/dev/null || echo "${FREED}B"))"
fi

# ── Systemd journal ────────────────────────────
echo -e "${BLUE}[Systemd Journal]${NC}"
if command -v journalctl &>/dev/null; then
    journalctl --vacuum-time="${MAX_AGE_DAYS}d" -u ironsight --quiet 2>/dev/null || true
    echo -e "  ${GREEN}✅${NC} Journal temizlendi (>${MAX_AGE_DAYS} gün)"
fi

echo ""
echo -e "${GREEN}  Toplam temizlenen: $(numfmt --to=iec $TOTAL_FREED 2>/dev/null || echo "${TOTAL_FREED}B")${NC}"
echo ""

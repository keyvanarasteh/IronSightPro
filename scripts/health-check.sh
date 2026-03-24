#!/usr/bin/env bash
# ============================================
# IronSight EDR — Sağlık Kontrolü
# ============================================
# Kullanım: ./scripts/health-check.sh
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Sağlık Kontrolü         ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

# ── Service durumu ─────────────────────────────
echo -e "${BLUE}[Service Durumu]${NC}"

# Systemd
if systemctl is-active --quiet ironsight 2>/dev/null; then
    echo -e "  ${GREEN}✅${NC} systemd service: aktif"
    echo -e "     Uptime: $(systemctl show ironsight --property=ActiveEnterTimestamp --value 2>/dev/null || echo 'N/A')"
elif pgrep -f "ironsight" &>/dev/null; then
    echo -e "  ${YELLOW}⚠️${NC}  ironsight çalışıyor (systemd dışı)"
    pgrep -af "ironsight" | head -3 | while read -r line; do
        echo -e "     PID: $line"
    done
else
    echo -e "  ${RED}❌${NC} ironsight çalışmıyor"
fi

# Docker
if docker ps --format '{{.Names}}' 2>/dev/null | grep -q "ironsight"; then
    echo -e "  ${GREEN}✅${NC} Docker container: çalışıyor"
    docker ps --filter "name=ironsight" --format "  {{.Names}}: {{.Status}}" 2>/dev/null
fi

# Sentinel
if pgrep -f "ironsight.*sentinel" &>/dev/null; then
    echo -e "  ${GREEN}✅${NC} Sentinel (watchdog): aktif"
else
    echo -e "  ${YELLOW}⚠️${NC}  Sentinel çalışmıyor"
fi

# ── Disk kullanımı ─────────────────────────────
echo ""
echo -e "${BLUE}[Disk Kullanımı]${NC}"

for DIR in /var/lib/ironsight/reports /var/lib/ironsight/dumps /var/log/ironsight; do
    if [[ -d "$DIR" ]]; then
        SIZE=$(du -sh "$DIR" 2>/dev/null | awk '{print $1}')
        COUNT=$(find "$DIR" -type f 2>/dev/null | wc -l)
        echo -e "  📁 $DIR: ${YELLOW}$SIZE${NC} ($COUNT dosya)"
    fi
done

# ── Son raporlar ───────────────────────────────
echo ""
echo -e "${BLUE}[Son Raporlar]${NC}"

REPORT_DIR="/var/lib/ironsight/reports"
if [[ -d "$REPORT_DIR" ]]; then
    LATEST=$(ls -t "$REPORT_DIR"/*.json 2>/dev/null | head -5)
    if [[ -n "$LATEST" ]]; then
        echo "$LATEST" | while read -r f; do
            MODIFIED=$(stat -c '%y' "$f" 2>/dev/null | cut -d. -f1)
            SIZE=$(stat -c '%s' "$f" 2>/dev/null)
            echo -e "  📊 $(basename "$f") — ${YELLOW}${SIZE}B${NC} — $MODIFIED"
        done
    else
        echo -e "  ${YELLOW}Henüz rapor yok${NC}"
    fi
fi

# ── Son dump'lar ───────────────────────────────
echo ""
echo -e "${BLUE}[Forensik Dump'lar]${NC}"

DUMP_DIR="/var/lib/ironsight/dumps"
if [[ -d "$DUMP_DIR" ]]; then
    DUMPS=$(find "$DUMP_DIR" -type f 2>/dev/null | wc -l)
    DUMP_SIZE=$(du -sh "$DUMP_DIR" 2>/dev/null | awk '{print $1}')
    echo -e "  💾 Toplam: ${YELLOW}$DUMPS dump${NC} ($DUMP_SIZE)"
    
    if [[ $DUMPS -gt 0 ]]; then
        ls -t "$DUMP_DIR" 2>/dev/null | head -3 | while read -r f; do
            echo -e "  └── $f"
        done
    fi
fi

# ── Sistem kaynakları ──────────────────────────
echo ""
echo -e "${BLUE}[Sistem Kaynakları]${NC}"

if pgrep -f "ironsight" &>/dev/null; then
    PID=$(pgrep -f "ironsight" | head -1)
    if [[ -f "/proc/$PID/status" ]]; then
        RSS=$(grep "VmRSS" "/proc/$PID/status" 2>/dev/null | awk '{print $2, $3}')
        THREADS=$(grep "Threads" "/proc/$PID/status" 2>/dev/null | awk '{print $2}')
        echo -e "  🧠 Bellek (RSS): ${YELLOW}${RSS:-N/A}${NC}"
        echo -e "  🧵 Thread sayısı: ${YELLOW}${THREADS:-N/A}${NC}"
    fi
    CPU=$(ps -p "$PID" -o %cpu= 2>/dev/null | tr -d ' ')
    echo -e "  ⚡ CPU kullanımı: ${YELLOW}${CPU:-N/A}%${NC}"
fi

# ── Config kontrolü ────────────────────────────
echo ""
echo -e "${BLUE}[Yapılandırma]${NC}"

for CFG in /etc/ironsight/config.toml "$HOME/.config/ironsight/config.toml" ./ironsight.toml; do
    if [[ -f "$CFG" ]]; then
        echo -e "  ${GREEN}✅${NC} Config: $CFG"
        break
    fi
done

echo ""

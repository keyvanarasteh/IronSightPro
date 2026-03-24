#!/usr/bin/env bash
# ============================================
# IronSight EDR — Benchmark Scripti
# ============================================
# Kullanım: sudo ./scripts/benchmark.sh
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }

IRONSIGHT="target/release/ironsight"
[[ ! -f "$IRONSIGHT" ]] && IRONSIGHT="target/release/ironsight-service"
[[ ! -f "$IRONSIGHT" ]] && IRONSIGHT=$(which ironsight 2>/dev/null || echo "")

if [[ -z "$IRONSIGHT" || ! -f "$IRONSIGHT" ]]; then
    echo -e "${RED}[ERROR]${NC} ironsight binary bulunamadı. Önce: cargo build --release"
    exit 1
fi

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Performance Benchmark   ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

PROC_COUNT=$(ls -d /proc/[0-9]* 2>/dev/null | wc -l)
info "Aktif process sayısı: $PROC_COUNT"
echo ""

# ── Derleme benchmark ──────────────────────────
echo -e "${BLUE}[1/4] Derleme Süresi${NC}"
info "Release derlemesi..."
COMPILE_START=$(date +%s%N)
cargo build --release --package ironsight-service 2>/dev/null
COMPILE_END=$(date +%s%N)
COMPILE_MS=$(( (COMPILE_END - COMPILE_START) / 1000000 ))
ok "Derleme: ${COMPILE_MS}ms"
echo ""

# ── Tek tarama benchmark ──────────────────────
echo -e "${BLUE}[2/4] Tek Tarama Süresi${NC}"
info "Top 20 process taranıyor..."
SCAN_START=$(date +%s%N)
sudo "$IRONSIGHT" --top 20 > /dev/null 2>&1 || true
SCAN_END=$(date +%s%N)
SCAN_MS=$(( (SCAN_END - SCAN_START) / 1000000 ))
ok "Tarama (top 20): ${SCAN_MS}ms"

info "Top 100 process taranıyor..."
SCAN_START=$(date +%s%N)
sudo "$IRONSIGHT" --top 100 > /dev/null 2>&1 || true
SCAN_END=$(date +%s%N)
SCAN_MS100=$(( (SCAN_END - SCAN_START) / 1000000 ))
ok "Tarama (top 100): ${SCAN_MS100}ms"
echo ""

# ── Binary boyutu ──────────────────────────────
echo -e "${BLUE}[3/4] Binary Boyutu${NC}"
BINARY_SIZE=$(stat -c %s "$IRONSIGHT" 2>/dev/null || stat -f %z "$IRONSIGHT" 2>/dev/null)
BINARY_MB=$(echo "scale=2; $BINARY_SIZE / 1048576" | bc)
ok "Binary: ${BINARY_MB} MB"

STRIPPED_SIZE=$BINARY_SIZE
if command -v strip &>/dev/null; then
    TMP_BIN=$(mktemp)
    cp "$IRONSIGHT" "$TMP_BIN"
    strip "$TMP_BIN" 2>/dev/null || true
    STRIPPED_SIZE=$(stat -c %s "$TMP_BIN" 2>/dev/null || stat -f %z "$TMP_BIN" 2>/dev/null)
    STRIPPED_MB=$(echo "scale=2; $STRIPPED_SIZE / 1048576" | bc)
    rm -f "$TMP_BIN"
    ok "Stripped: ${STRIPPED_MB} MB"
fi
echo ""

# ── Test süresi ────────────────────────────────
echo -e "${BLUE}[4/4] Test Suite Süresi${NC}"
info "Tüm testler çalıştırılıyor..."
TEST_START=$(date +%s%N)
cargo test --workspace 2>/dev/null | tail -3
TEST_END=$(date +%s%N)
TEST_MS=$(( (TEST_END - TEST_START) / 1000000 ))
ok "Testler: ${TEST_MS}ms"
echo ""

# ── Özet ───────────────────────────────────────
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "  ${YELLOW}Benchmark Sonuçları${NC}"
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "  Process sayısı:     $PROC_COUNT"
echo -e "  Derleme:            ${COMPILE_MS}ms"
echo -e "  Tarama (top 20):    ${SCAN_MS}ms"
echo -e "  Tarama (top 100):   ${SCAN_MS100}ms"
echo -e "  Binary:             ${BINARY_MB}MB"
echo -e "  Testler:            ${TEST_MS}ms"
echo ""

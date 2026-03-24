#!/usr/bin/env bash
# ============================================
# IronSight EDR — Cross-Compile
# ============================================
# Kullanım: ./scripts/cross-build.sh [hedef]
# Hedefler: linux-x86, linux-arm, linux-musl
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

info()  { echo -e "${BLUE}[INFO]${NC} $1"; }
ok()    { echo -e "${GREEN}[OK]${NC} $1"; }
err()   { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

TARGET="${1:-all}"

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Cross Compile           ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

OUTPUT_DIR="dist"
mkdir -p "$OUTPUT_DIR"

VERSION=$(grep '^version' Cargo.toml | head -1 | sed 's/.*"\(.*\)".*/\1/')
info "Version: $VERSION"

build_target() {
    local RUST_TARGET="$1"
    local LABEL="$2"
    local BINARY_NAME="ironsight-${VERSION}-${LABEL}"
    
    info "[$LABEL] Hedef: $RUST_TARGET"
    
    # Toolchain kur
    rustup target add "$RUST_TARGET" 2>/dev/null || true
    
    # Derle
    if cargo build --release --target "$RUST_TARGET" --package ironsight-service 2>/dev/null; then
        local BIN="target/${RUST_TARGET}/release/ironsight"
        [[ ! -f "$BIN" ]] && BIN="target/${RUST_TARGET}/release/ironsight-service"
        
        if [[ -f "$BIN" ]]; then
            cp "$BIN" "$OUTPUT_DIR/$BINARY_NAME"
            strip "$OUTPUT_DIR/$BINARY_NAME" 2>/dev/null || true
            local SIZE=$(du -sh "$OUTPUT_DIR/$BINARY_NAME" | awk '{print $1}')
            ok "[$LABEL] → $OUTPUT_DIR/$BINARY_NAME ($SIZE)"
        else
            echo -e "  ${RED}❌${NC} [$LABEL] Binary bulunamadı"
        fi
    else
        echo -e "  ${RED}❌${NC} [$LABEL] Derleme başarısız (cross-compiler gerekli olabilir)"
    fi
}

case "$TARGET" in
    linux-x86|all)
        build_target "x86_64-unknown-linux-gnu" "linux-x86_64"
        ;;&
    linux-musl|all)
        build_target "x86_64-unknown-linux-musl" "linux-x86_64-musl"
        ;;&
    linux-arm|all)
        build_target "aarch64-unknown-linux-gnu" "linux-aarch64"
        ;;&
    all)
        ;;  # tüm hedefler yukarıda çalıştı
    *)
        err "Bilinmeyen hedef: $TARGET (linux-x86, linux-arm, linux-musl, all)"
        ;;
esac

echo ""
echo -e "${BLUE}[Sonuç]${NC}"
ls -lh "$OUTPUT_DIR"/ironsight-* 2>/dev/null || echo "Henüz binary yok"
echo ""

# SHA256 checksum
info "SHA256 checksum'lar oluşturuluyor..."
cd "$OUTPUT_DIR"
sha256sum ironsight-* > SHA256SUMS 2>/dev/null || true
ok "SHA256SUMS oluşturuldu"
cat SHA256SUMS 2>/dev/null || true
echo ""

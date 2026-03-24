#!/usr/bin/env bash
# ============================================
# IronSight EDR — Yetki Kontrol Scripti
# ============================================
# Kullanım: ./scripts/check-privileges.sh
# ============================================
set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

echo ""
echo -e "${BLUE}╔══════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  IronSight — Yetki Kontrolü          ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════╝${NC}"
echo ""

PASS=0
FAIL=0
WARN=0

check_pass() { echo -e "  ${GREEN}✅${NC} $1"; ((PASS++)); }
check_fail() { echo -e "  ${RED}❌${NC} $1"; ((FAIL++)); }
check_warn() { echo -e "  ${YELLOW}⚠️${NC}  $1"; ((WARN++)); }

# ── Root / Sudo ────────────────────────────────
echo -e "${BLUE}[Root Erişimi]${NC}"
if [[ $EUID -eq 0 ]]; then
    check_pass "Root olarak çalışıyor"
else
    check_warn "Root değil (bazı özellikler kısıtlı olabilir)"
fi

# ── /proc erişimi ──────────────────────────────
echo ""
echo -e "${BLUE}[Proc Filesystem]${NC}"

if [[ -d /proc ]]; then
    check_pass "/proc mevcut"
else
    check_fail "/proc bulunamadı (Linux değil mi?)"
fi

if [[ -r /proc/1/maps ]] 2>/dev/null; then
    check_pass "/proc/PID/maps okunabiliyor"
else
    check_fail "/proc/PID/maps okunamıyor — CAP_SYS_PTRACE gerekli"
fi

if [[ -r /proc/1/fd ]] 2>/dev/null; then
    check_pass "/proc/PID/fd okunabiliyor"
else
    check_fail "/proc/PID/fd okunamıyor"
fi

if [[ -r /proc/net/tcp ]]; then
    check_pass "/proc/net/tcp okunabiliyor"
else
    check_fail "/proc/net/tcp okunamıyor"
fi

# ── Linux Capabilities ─────────────────────────
echo ""
echo -e "${BLUE}[Linux Capabilities]${NC}"

IRONSIGHT_BIN=$(which ironsight 2>/dev/null || echo "")
if [[ -n "$IRONSIGHT_BIN" && -f "$IRONSIGHT_BIN" ]]; then
    if command -v getcap &>/dev/null; then
        CAPS=$(getcap "$IRONSIGHT_BIN" 2>/dev/null || echo "")
        if echo "$CAPS" | grep -q "cap_sys_ptrace"; then
            check_pass "CAP_SYS_PTRACE (process bellek okuma)"
        else
            check_fail "CAP_SYS_PTRACE eksik"
        fi
        if echo "$CAPS" | grep -q "cap_kill"; then
            check_pass "CAP_KILL (sinyal gönderme)"
        else
            check_fail "CAP_KILL eksik"
        fi
        if echo "$CAPS" | grep -q "cap_dac_read_search"; then
            check_pass "CAP_DAC_READ_SEARCH (dosya okuma)"
        else
            check_warn "CAP_DAC_READ_SEARCH eksik (root ile çalışırsa sorun değil)"
        fi
    else
        check_warn "getcap bulunamadı — capability kontrolü yapılamadı"
    fi
else
    check_warn "ironsight binary'si PATH'te bulunamadı"
fi

# ── Sinyal gönderme ────────────────────────────
echo ""
echo -e "${BLUE}[Sinyal Yetkisi]${NC}"

if kill -0 1 2>/dev/null; then
    check_pass "PID 1'e sinyal gönderilebilir (SIGSTOP/SIGKILL)"
else
    check_warn "PID 1'e sinyal gönderilemez (root/CAP_KILL gerekli)"
fi

# ── Dizin erişimi ──────────────────────────────
echo ""
echo -e "${BLUE}[Dizin Erişimi]${NC}"

for DIR in /etc/ironsight /var/lib/ironsight/reports /var/lib/ironsight/dumps /var/log/ironsight; do
    if [[ -d "$DIR" ]]; then
        if [[ -w "$DIR" ]]; then
            check_pass "$DIR (yazılabilir)"
        else
            check_fail "$DIR (yazılamaz)"
        fi
    else
        check_warn "$DIR mevcut değil (install.sh ile oluşturulur)"
    fi
done

# ── Kernel modülleri (opsiyonel) ───────────────
echo ""
echo -e "${BLUE}[Kernel / eBPF (Opsiyonel)]${NC}"

if [[ -d /sys/kernel/debug/tracing ]]; then
    check_pass "debugfs/tracing mevcut"
else
    check_warn "debugfs/tracing bulunamadı (eBPF sınırlı)"
fi

if command -v bpftool &>/dev/null; then
    check_pass "bpftool kurulu"
else
    check_warn "bpftool eksik (kernel izleme için gerekli)"
fi

if [[ -f /proc/config.gz ]]; then
    if zcat /proc/config.gz 2>/dev/null | grep -q "CONFIG_BPF=y"; then
        check_pass "Kernel BPF desteği aktif"
    else
        check_warn "Kernel BPF desteği bulunamadı"
    fi
elif [[ -f "/boot/config-$(uname -r)" ]]; then
    if grep -q "CONFIG_BPF=y" "/boot/config-$(uname -r)" 2>/dev/null; then
        check_pass "Kernel BPF desteği aktif"
    else
        check_warn "Kernel BPF desteği bulunamadı"
    fi
else
    check_warn "Kernel config dosyası bulunamadı"
fi

# ── Özet ───────────────────────────────────────
echo ""
echo -e "${BLUE}═══════════════════════════════════════${NC}"
echo -e "  ${GREEN}✅ Geçen: $PASS${NC}   ${RED}❌ Başarısız: $FAIL${NC}   ${YELLOW}⚠️  Uyarı: $WARN${NC}"
echo ""

if [[ $FAIL -eq 0 ]]; then
    echo -e "  ${GREEN}Sistem IronSight için hazır!${NC}"
elif [[ $FAIL -le 2 ]]; then
    echo -e "  ${YELLOW}Bazı yetkiler eksik — root olarak çalıştırın veya capabilities ayarlayın${NC}"
    echo -e "  ${YELLOW}sudo setcap cap_sys_ptrace,cap_kill,cap_dac_read_search+ep \$(which ironsight)${NC}"
else
    echo -e "  ${RED}Ciddi yetki eksiklikleri var — kurulum talimatlarını kontrol edin${NC}"
fi
echo ""

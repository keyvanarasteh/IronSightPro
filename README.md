<p align="center">
  <img src="audit/media/01_ironsight_cover.png" alt="IronSight EDR" width="600" />
</p>

<h1 align="center">IronSight</h1>
<p align="center">
  <strong>Endpoint Detection & Response — Rust ile yazılmış modüler EDR sistemi</strong>
</p>

<p align="center">
  <a href="#kurulum"><img src="https://img.shields.io/badge/Kurulum-blue?style=for-the-badge" /></a>
  <a href="#mimari"><img src="https://img.shields.io/badge/Mimari-purple?style=for-the-badge" /></a>
  <a href="#docker"><img src="https://img.shields.io/badge/Docker-blue?style=for-the-badge&logo=docker" /></a>
  <img src="https://img.shields.io/badge/Rust-2024_Edition-orange?style=for-the-badge&logo=rust" />
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" />
</p>

---

## İçindekiler

- [Hakkında](#hakkında)
- [Mimari](#mimari)
- [Gereksinimler](#gereksinimler)
- [Kurulum](#kurulum)
  - [Linux](#linux-debianubuntuarch)
  - [macOS](#macos)
  - [Windows](#windows)
  - [Docker](#docker)
- [Yapılandırma](#yapılandırma)
- [Kullanım](#kullanım)
- [Geliştirme](#geliştirme)
- [Test](#test)
- [Lisans](#lisans)

---

## Hakkında

**IronSight**, Rust ile yazılmış modüler bir Endpoint Detection & Response (EDR) sistemidir. Sistem process'leri gerçek zamanlı izler, davranışsal analiz yapar, ağ bağlantılarını haritalandırır, bellek anomalilerini tespit eder ve otomatik müdahale gerçekleştirir.

### Temel Özellikler

| Özellik | Açıklama |
|---------|----------|
| 🔍 **Process İzleme** | Tüm process'lerin CPU, bellek, ağ, dosya yolu analizi |
| 🎯 **Heuristic Engine** | 6 kategori × çoklu sinyal → composite tehdit skoru |
| 🧠 **Bellek Forensics** | W^X ihlal tespiti, anonymous executable bölge tarama |
| 🌐 **Ağ İstihbaratı** | Socket-PID haritalama, şüpheli port tespiti |
| 🔒 **Binary Güvenlik** | SHA-256 hash, Shannon entropy, imza doğrulama |
| ⚡ **Kernel İzleme** | eBPF tracepoint'ler ile syscall izleme (Linux) |
| 🛡️ **Otomatik Müdahale** | Forensik sıra: Suspend → Dump → Kill |
| 📊 **Raporlama** | JSON/Text çıktı, SIEM export (Splunk/Sentinel) |
| 🐕 **Watchdog** | Anti-tamper sentinel process |

---

## Mimari

```
ironsight/
├── crates/
│   ├── ironsight-core       # Process spy, snapshot, control
│   ├── ironsight-heuristic  # Threat scoring, decay engine
│   ├── ironsight-kernel     # eBPF/ETW kernel monitoring
│   ├── ironsight-memory     # /proc/PID/maps, W^X detection
│   ├── ironsight-network    # Socket mapping, DNS enrichment
│   ├── ironsight-report     # Incident reports, SIEM export
│   ├── ironsight-response   # Suspend → Dump → Kill
│   ├── ironsight-security   # Hash, entropy, signature audit
│   ├── ironsight-service    # Main orchestrator, config, CLI
│   └── ironsight-ui         # Dioxus desktop dashboard
├── config/                  # User configuration
├── docker/                  # Docker config templates
├── audit/                   # Architecture audit & UX specs
├── Dockerfile
├── docker-compose.yml
└── Makefile
```

### Veri Akışı

```
Config → Privilege Check → Process Snapshot
                              ├── Security Audit (hash, entropy, signature)
                              ├── Network Audit (sockets, ports, DNS)
                              └── Memory Audit (maps, W^X, patterns)
                                        ↓
                              Heuristic Engine (scoring)
                                        ↓
                              ┌── Clean → Log only
                              └── Threat → Report + Response
                                            (Suspend → Dump → Kill)
```

---

## Gereksinimler

### Tüm Platformlar — Ortak

| Gereksinim | Minimum | Önerilen |
|-----------|---------|----------|
| **Rust** | 1.82.0 | 1.83.0+ (edition 2024) |
| **Cargo** | Rust ile gelir | — |
| **Git** | 2.0+ | Son sürüm |
| **Bellek** | 128 MB | 512 MB |
| **Disk** | 100 MB | 500 MB (dump'lar dahil) |

### Linux (Debian/Ubuntu/Arch)

| Gereksinim | Paket | Amaç |
|-----------|-------|------|
| **GCC/Clang** | `build-essential` | Rust native derleme |
| **pkg-config** | `pkg-config` | Kütüphane keşfi |
| **OpenSSL** | `libssl-dev` | TLS/HTTPS (reqwest) |
| **procfs** | Kernel built-in | `/proc` filesystem erişimi |
| **CAP_SYS_PTRACE** | — | Process bellek okuma |
| **CAP_KILL** | — | Process sinyal gönderme |
| **root veya sudo** | — | Tam process erişimi için |

**Opsiyonel (Kernel modülü için):**

| Gereksinim | Paket | Amaç |
|-----------|-------|------|
| **clang** | `clang` | eBPF derleme |
| **llvm** | `llvm` | eBPF araçları |
| **libbpf** | `libbpf-dev` | eBPF kütüphanesi |
| **linux-headers** | `linux-headers-$(uname -r)` | Kernel header'ları |

### macOS

| Gereksinim | Nasıl | Amaç |
|-----------|-------|------|
| **Xcode CLT** | `xcode-select --install` | C compiler, linker |
| **Homebrew** | [brew.sh](https://brew.sh) | Paket yöneticisi |
| **OpenSSL** | `brew install openssl@3` | TLS/HTTPS |
| **pkg-config** | `brew install pkg-config` | Kütüphane keşfi |

> ⚠️ **Not:** macOS'ta `/proc` yoktur. Bellek ve ağ analizi `sysinfo` crate üzerinden çalışır. Kernel izleme `dtrace`/`Endpoint Security Framework` gerektirir.

### Windows

| Gereksinim | Nasıl | Amaç |
|-----------|-------|------|
| **Visual Studio Build Tools** | [vs.dev](https://visualstudio.microsoft.com/downloads/) | MSVC compiler |
| **Rust (MSVC)** | `rustup default stable-x86_64-pc-windows-msvc` | Windows ABI |
| **OpenSSL** | `vcpkg install openssl` veya `choco install openssl` | TLS/HTTPS |
| **Git for Windows** | [git-scm.com](https://git-scm.com) | Kaynak kontrolü |

> ⚠️ **Not:** Windows'ta `/proc` yoktur. Process ve ağ bilgileri `sysinfo` + Windows API üzerinden alınır. Kernel izleme ETW (Event Tracing for Windows) gerektirir.

**Opsiyonel (ETW desteği):**

| Gereksinim | Amaç |
|-----------|------|
| Windows SDK | ETW API erişimi |
| Administrator privileges | Kernel event trace oturumu |

---

## Kurulum

### Rust Kurulumu (Tüm Platformlar)

```bash
# rustup ile Rust kur
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Minimum sürüm kontrolü
rustc --version  # >= 1.82.0 olmalı

# Nightly gerekebilir (edition 2024 için)
rustup toolchain install nightly
rustup default nightly
```

---

### Linux (Debian/Ubuntu)

```bash
# 1. Sistem bağımlılıkları
sudo apt update
sudo apt install -y build-essential pkg-config libssl-dev cmake git

# 2. (Opsiyonel) eBPF geliştirme için
sudo apt install -y clang llvm libbpf-dev linux-headers-$(uname -r)

# 3. Projeyi klonla
git clone https://github.com/keyvanarasteh/IronSight.git
cd IronSight

# 4. Derle
cargo build --release

# 5. Binary'yi yükle
sudo cp target/release/ironsight /usr/local/bin/
sudo chmod +x /usr/local/bin/ironsight

# 6. Config dizini oluştur
sudo mkdir -p /etc/ironsight
sudo cp docker/ironsight.toml /etc/ironsight/config.toml

# 7. Rapor ve dump dizinleri
sudo mkdir -p /var/lib/ironsight/{reports,dumps}
sudo chmod 700 /var/lib/ironsight/dumps

# 8. Çalıştır (root gerekir)
sudo ironsight --config /etc/ironsight/config.toml

# 9. (Opsiyonel) Privilege capabilities
# Root olmadan çalıştırmak için:
sudo setcap cap_sys_ptrace,cap_kill,cap_dac_read_search+ep /usr/local/bin/ironsight
ironsight --config /etc/ironsight/config.toml
```

### Linux (Arch/Manjaro)

```bash
# 1. Sistem bağımlılıkları
sudo pacman -S base-devel openssl pkg-config cmake git

# 2. (Opsiyonel) eBPF
sudo pacman -S clang llvm libbpf linux-headers

# 3-9. Yukarıdaki adımların aynısı
git clone https://github.com/keyvanarasteh/IronSight.git
cd IronSight
cargo build --release
sudo cp target/release/ironsight /usr/local/bin/
```

### Linux (Fedora/RHEL)

```bash
# 1. Sistem bağımlılıkları
sudo dnf groupinstall "Development Tools"
sudo dnf install openssl-devel pkg-config cmake git

# 2. (Opsiyonel) eBPF
sudo dnf install clang llvm libbpf-devel kernel-devel

# 3-9. Yukarıdaki adımların aynısı
```

---

### macOS

```bash
# 1. Xcode Command Line Tools
xcode-select --install

# 2. Homebrew ile bağımlılıklar
brew install openssl@3 pkg-config cmake

# 3. OpenSSL'i Rust'a tanıt
export OPENSSL_DIR=$(brew --prefix openssl@3)
export PKG_CONFIG_PATH="$OPENSSL_DIR/lib/pkgconfig"

# 4. Projeyi klonla ve derle
git clone https://github.com/keyvanarasteh/IronSight.git
cd IronSight
cargo build --release

# 5. Kur
sudo cp target/release/ironsight /usr/local/bin/

# 6. Config
mkdir -p ~/.config/ironsight
cp docker/ironsight.toml ~/.config/ironsight/config.toml

# 7. Çalıştır (sudo gerekir)
sudo ironsight --config ~/.config/ironsight/config.toml
```

> 💡 **macOS Tip:** `.zshrc` veya `.bashrc` dosyanıza ekleyin:
> ```bash
> export OPENSSL_DIR=$(brew --prefix openssl@3)
> export PKG_CONFIG_PATH="$OPENSSL_DIR/lib/pkgconfig"
> ```

---

### Windows

```powershell
# 1. Visual Studio Build Tools kur
# https://visualstudio.microsoft.com/downloads/ → Build Tools
# "Desktop development with C++" workload'unu seç

# 2. Chocolatey ile bağımlılıklar (PowerShell Admin)
choco install git cmake openssl

# 3. Rust kur (MSVC)
# https://rustup.rs → rustup-init.exe
rustup default stable-x86_64-pc-windows-msvc

# 4. OpenSSL ortam değişkeni
$env:OPENSSL_DIR = "C:\Program Files\OpenSSL-Win64"

# 5. Projeyi klonla ve derle
git clone https://github.com/keyvanarasteh/IronSight.git
cd IronSight
cargo build --release

# 6. Config
mkdir "$env:APPDATA\ironsight"
copy docker\ironsight.toml "$env:APPDATA\ironsight\config.toml"

# 7. Çalıştır (Administrator olarak)
.\target\release\ironsight.exe --config "$env:APPDATA\ironsight\config.toml"
```

---

### Docker

En hızlı başlangıç yöntemi:

```bash
# 1. Klonla
git clone https://github.com/keyvanarasteh/IronSight.git
cd IronSight

# 2. Config oluştur
make config
# → config/ironsight.toml dosyasını düzenle

# 3. Derle ve başlat
make build
make up

# 4. (Opsiyonel) Grafana monitoring ile
make monitor
# → http://localhost:3000 (admin/ironsight)

# 5. Logları izle
make logs

# 6. Durdur
make down
```

**Manuel Docker komutları:**

```bash
# Derle
docker compose build

# Başlat (scanner + sentinel)
docker compose up -d ironsight sentinel

# Monitoring stack ile
docker compose --profile monitoring up -d

# Durum
docker compose ps

# Loglar
docker compose logs -f ironsight
```

---

## Yapılandırma

IronSight, TOML formatında yapılandırma dosyası kullanır. Arama sırası:

1. `--config <path>` CLI argümanı
2. `./ironsight.toml` (çalışma dizini)
3. `/etc/ironsight/config.toml` (Linux)
4. `~/.config/ironsight/config.toml` (kullanıcı)

### Örnek Config

```toml
[general]
log_level = "info"                          # trace, debug, info, warn, error
report_dir = "/var/lib/ironsight/reports"
dump_dir = "/var/lib/ironsight/dumps"

[scan]
interval_secs = 300         # Daemon modunda tarama aralığı
top_n = 50                  # En fazla X process raporla
daemon_mode = false         # true: sürekli çalış

[thresholds]
low_score = 10              # 0-10: Clean, 11-30: Low
medium_score = 30           # 31-50: Medium
high_score = 50             # 51-70: High
critical_score = 70         # 71-100: Critical
export_min_score = 10       # Rapor oluşturma minimum skoru

[response]
auto_response = false       # Otomatik müdahale
auto_response_min_score = 70

[exclusions]
names = ["init", "systemd", "sshd"]
paths = ["/usr/sbin/"]

[network]
suspicious_ports = [4444, 5555, 6667, 9050, 31337]

[watchdog]
enabled = true
heartbeat_interval_secs = 10
max_restarts = 3
```

---

## Kullanım

### CLI Komutları

```bash
# Temel tarama (tek sefer)
sudo ironsight

# Belirli PID tara
sudo ironsight --pid 666

# En yüksek N threat'i göster
sudo ironsight --top 10

# Özel config ile
sudo ironsight --config /path/to/config.toml

# Yetki kontrolü
ironsight --check-privileges

# Örnek config oluştur
ironsight --generate-config > ironsight.toml

# Sentinel (watchdog) modu
sudo ironsight --sentinel
```

### Systemd Service (Linux)

```bash
# /etc/systemd/system/ironsight.service
sudo tee /etc/systemd/system/ironsight.service << 'EOF'
[Unit]
Description=IronSight EDR Scanner
After=network.target

[Service]
Type=simple
ExecStart=/usr/local/bin/ironsight --config /etc/ironsight/config.toml
Restart=always
RestartSec=5
User=root
AmbientCapabilities=CAP_SYS_PTRACE CAP_KILL CAP_DAC_READ_SEARCH
ProtectSystem=strict
ReadWritePaths=/var/lib/ironsight /var/log/ironsight
PrivateTmp=true

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl daemon-reload
sudo systemctl enable --now ironsight
sudo systemctl status ironsight
```

---

## Geliştirme

```bash
# Tüm crate'leri derle
cargo build --workspace

# Sadece service binary
cargo build --release --package ironsight-service

# Tek crate testi
cargo test --package ironsight-heuristic

# Tüm testler
cargo test --workspace

# Lint
cargo clippy --workspace -- -W warnings

# Format
cargo fmt --all

# Hot-reload geliştirme (cargo-watch)
cargo install cargo-watch
cargo watch -x "run --package ironsight-service"
```

---

## Test

```bash
# Tüm workspace testleri
cargo test --workspace

# Belirli bir crate
cargo test --package ironsight-core
cargo test --package ironsight-heuristic
cargo test --package ironsight-memory
cargo test --package ironsight-network
cargo test --package ironsight-security

# Belirli test fonksiyonu
cargo test --package ironsight-heuristic -- decay

# Verbose çıktı
cargo test --workspace -- --nocapture
```

---

## Proje Durumu

| Crate | Durum | Not |
|-------|-------|-----|
| ironsight-core | ✅ Çalışıyor | Process spy, snapshot, control |
| ironsight-heuristic | ✅ Çalışıyor | Scoring + decay engine |
| ironsight-memory | ✅ Çalışıyor | maps parsing, W^X detection |
| ironsight-network | ✅ Çalışıyor | Socket mapping |
| ironsight-report | ✅ Çalışıyor | JSON/Text raporlama |
| ironsight-response | ✅ Çalışıyor | Suspend → Dump → Kill |
| ironsight-security | ✅ Çalışıyor | Hash, entropy, path analysis |
| ironsight-service | ✅ Çalışıyor | CLI orchestrator |
| ironsight-kernel | 🚧 Stub | eBPF implementasyon bekliyor |
| ironsight-ui | 🚧 Devam | Dioxus desktop dashboard |

---

## Lisans

MIT License — [LICENSE](LICENSE)

**Yazar:** Keyvan Arasteh · İstinye Üniversitesi

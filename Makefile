# ============================================
# IronSight EDR — Makefile
# ============================================
# Hızlı komutlar:
#   make build    — Docker image derle
#   make up       — Başlat
#   make down     — Durdur
#   make logs     — Logları izle
#   make monitor  — Grafana + Loki ile başlat
# ============================================

.PHONY: build up down logs monitor clean test dev status

# ── Derleme ───────────────────────────────────
build:
	docker compose build

build-no-cache:
	docker compose build --no-cache

# ── Başlatma ──────────────────────────────────
up:
	docker compose up -d ironsight sentinel
	@echo "✅ IronSight başlatıldı"
	@echo "📊 Monitoring için: make monitor"

monitor:
	docker compose --profile monitoring up -d
	@echo "✅ IronSight + Monitoring başlatıldı"
	@echo "📊 Grafana: http://localhost:3000"

# ── Durdurma ──────────────────────────────────
down:
	docker compose --profile monitoring down
	@echo "⏹️  IronSight durduruldu"

# ── Log İzleme ────────────────────────────────
logs:
	docker compose logs -f ironsight sentinel

logs-all:
	docker compose --profile monitoring logs -f

# ── Durum ─────────────────────────────────────
status:
	@echo "=== Container Durumu ==="
	docker compose ps
	@echo ""
	@echo "=== Son Loglar ==="
	docker compose logs --tail=5 ironsight 2>/dev/null || true

# ── Test ──────────────────────────────────────
test:
	cargo test --workspace

# ── Dev (local derleme) ───────────────────────
dev:
	cargo run --release --package ironsight-service

# ── Temizlik ──────────────────────────────────
clean:
	docker compose --profile monitoring down -v
	docker image rm ironsight-ironsight ironsight-sentinel 2>/dev/null || true
	@echo "🗑️  Temizlendi"

# ── Config ────────────────────────────────────
config:
	mkdir -p config
	@if [ ! -f config/ironsight.toml ]; then \
		cp docker/ironsight.toml config/ironsight.toml; \
		echo "📝 config/ironsight.toml oluşturuldu — düzenleyin"; \
	else \
		echo "⚠️  config/ironsight.toml zaten var"; \
	fi

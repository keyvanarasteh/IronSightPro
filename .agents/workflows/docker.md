---
description: how to run the Docker infrastructure (EDR service, monitoring, Grafana)
---
// turbo-all

## Docker Workflow

1. Build Docker images:
```
docker compose build
```

2. Start IronSight EDR + Sentinel:
```
docker compose up -d ironsight sentinel
```

3. Start with full monitoring (Grafana + Loki):
```
docker compose --profile monitoring up -d
```

4. Check status:
```
docker compose ps
```

5. View logs:
```
docker compose logs -f ironsight sentinel
```

6. Stop everything:
```
docker compose --profile monitoring down
```

7. Clean up (remove volumes + images):
```
docker compose --profile monitoring down -v
docker image rm ironsight-ironsight ironsight-sentinel 2>/dev/null || true
```

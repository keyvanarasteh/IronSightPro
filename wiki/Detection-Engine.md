# The Detection Engine

Unlike signature-based antiviruses, IronSight heavily relies on a composite heuristic engine (`ironsight-heuristic`). The system tracks behavior over time to identify anomalies indicative of malicious intent.

## The Threat Score

Every monitored process starts with a threat score of `0.0`. 
As various subsystems pipe events to the heuristic engine, the score is mathematically adjusted using specific weights:

- **High Entropy / Unsigned Binary:** `+20.0`
- **W^X Memory Violation:** `+40.0`
- **Suspicious Outbound Port (4444, etc):** `+30.0`
- **CPU Spiking / Miner profile:** `+15.0`
- **Known clean signer (Microsoft, Apple):** `-50.0`

## Time Decay Mechanism

To prevent processes from slowly accumulating points from false positive behavioral quirks over months of uptime, IronSight implements a temporal decay mechanism.

Every configurable cycle (e.g., `interval_secs = 300`), active processes lose a percentage of their non-critical threat score. If a process spikes to a score of `40` by downloading a file and generating heavy CPU load, but acts normally for the next 24 hours, its threat score will smoothly decay back down to `0.0`.

## Threshold Categories

Scores are mapped into four distinct operational thresholds defined in `ironsight.toml`:

```toml
[thresholds]
low_score = 10              # Clean range: 0-10
medium_score = 30           # Monitored intensely: 11-30
high_score = 50             # Alert / Reported: 31-50
critical_score = 70         # Mitigation Triggered: 51-100+
```

# IronSight Architecture Deep Dive

IronSight is explicitly designed with a highly modular Cargo workspace to isolate capabilities, limit blast radius, and ease cross-platform compilation challenges.

## Workspace Crates

The workspace is composed of 10 primary crates:

1. **`ironsight-core`**: The foundational component responsible for process enumeration, snapshotting, and OS control abstractions.
2. **`ironsight-memory`**: Specialized component for reading `/proc/PID/maps`, memory scraping, and identifying runtime execution anomalies like `W^X` violations.
3. **`ironsight-network`**: Implements socket-to-PID mapping, DNS query enrichment, and tracking suspicious inbound/outbound connection events.
4. **`ironsight-security`**: Handles static file audits: SHA-256 hashing, Shannon entropy calculation for packers, and OS-specific signature/certificate verification.
5. **`ironsight-heuristic`**: The heart of the detection pipeline. Aggregates data from Core, Memory, Network, and Security crates to generate a composite threat score based on temporal decay algorithms.
6. **`ironsight-response`**: The mitigation engine. When heuristically triggered, it initiates a forensics pipeline (`SIGSTOP` -> Dump Memory -> `SIGKILL`).
7. **`ironsight-report`**: Formats incidents into standard SIEM logs (Splunk HEC, Syslog) and outputs actionable JSON payloads.
8. **`ironsight-kernel`**: Low-level kernel auditing via eBPF (Linux) and ETW (Windows).
9. **`ironsight-service`**: Th orchestrator binary. Loads TOML configs, initializes channels, and manages the main execution loop.
10. **`ironsight-ui`**: A Tauri/Dioxus-based cross-platform GUI for local analysts.

## Data Flow

Data flows linearly through the orchestrator. The pipeline sequence is as follows:

1. **Discovery (Core):** New processes are detected or existing ones are scheduled for rescanning.
2. **Enrichment (Memory, Network, Security):** Subsystems audit the process concurrently if possible, gathering context.
3. **Scoring (Heuristics):** The collected context is fed into the rules engine.
4. **Action (Response & Report):** Based on thresholds defined in the user's config (`ironsight.toml`), the process is either mitigated, flagged for reporting, or ignored.

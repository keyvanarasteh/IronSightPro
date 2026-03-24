---
description: how to add a new crate to the IronSight workspace
---

## Adding a New Crate

1. Create the crate directory:
```
mkdir -p crates/ironsight-<name>/src
```

2. Create `crates/ironsight-<name>/Cargo.toml` with:
```toml
[package]
name = "ironsight-<name>"
version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true

[dependencies]
ironsight-core.workspace = true
serde.workspace = true
tracing.workspace = true
anyhow.workspace = true
```

3. Create `crates/ironsight-<name>/src/lib.rs` with the module structure.

4. Register in the root `Cargo.toml` workspace members:
```toml
members = [
    # ... existing
    "crates/ironsight-<name>",
]
```

5. If other crates need it, add the workspace dependency:
```toml
[workspace.dependencies]
ironsight-<name> = { path = "crates/ironsight-<name>" }
```

6. Build and test:
```
cargo build -p ironsight-<name>
cargo test -p ironsight-<name>
```

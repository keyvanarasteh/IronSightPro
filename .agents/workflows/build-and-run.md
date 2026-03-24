---
description: how to build and run the IronSight project locally
---
// turbo-all

## Build & Run

1. Install Rust toolchain (if not installed):
```
rustup update stable
```

2. Build the entire workspace:
```
cargo build --workspace
```

3. Run the EDR service:
```
cargo run --release --package ironsight-service
```

4. Run the Dioxus desktop UI:
```
cargo run --package ironsight-ui
```

5. Run all tests:
```
cargo test --workspace
```

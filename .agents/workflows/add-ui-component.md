---
description: how to add a new UI component or page to the Dioxus dashboard
---

## Adding a UI Component

1. Create a new component file under the appropriate module:
   - General: `crates/ironsight-ui/src/components/<name>.rs`
   - EDR widgets: `crates/ironsight-ui/src/components/edr_widgets/<name>.rs`
   - Q-Static ports: `crates/ironsight-ui/src/components/qstatic/<module>.rs`

2. Register the module in the parent `mod.rs`:
```rust
pub mod <name>;
```

3. For Q-Static ported components: prefix with `Qs` (e.g., `QsButton`, `QsDialog`).

4. Use Q-Static CSS custom properties (`--qs-*`) for theme integration.

5. **IMPORTANT**: Dioxus rsx! format strings cannot contain function calls like `t("key")`.
   Always extract to `let` bindings before the `rsx!` block:
```rust
let title = t("page.title");  // ✅ OK
rsx! { h1 { "{title}" } }     // ✅ OK
// rsx! { h1 { "{t("page.title")}" } }  // ❌ FAILS
```

## Adding a Page/Route

1. Create a new view file: `crates/ironsight-ui/src/views/<name>.rs`
2. Register in `crates/ironsight-ui/src/views/mod.rs`
3. Add route variant to `Route` enum in `main.rs`:
```rust
#[route("/your-path")]
YourPage {},
```
4. Add sidebar navigation link in `NavBar` with translated label.
5. Add translation keys to `i18n.rs` for both EN and TR.
6. Build to verify: `cargo build -p ironsight-ui`

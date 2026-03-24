---
description: how to add translations for the multi-language i18n system
---

## Adding Translations

1. Open `crates/ironsight-ui/src/i18n.rs`

2. Add translation keys to BOTH `english()` and `turkish()` functions:
```rust
// In english():
m.insert("your.key", "English text");

// In turkish():
m.insert("your.key", "Türkçe metin");
```

3. Use in components:
```rust
let t = use_i18n();
let my_text = t("your.key");  // extract BEFORE rsx!
rsx! { span { "{my_text}" } }
```

4. Key naming conventions:
   - Navigation: `nav.<item>`, `nav.group.<group>`
   - Dashboard: `dash.<item>`
   - Pages: `page.<name>`
   - Common: `common.<item>`
   - EDR widgets: `edr.<item>`
   - Theme: `theme.<item>`

5. Adding a new language:
   - Add variant to `Locale` enum
   - Implement `label()` and `flag()` for the variant
   - Add it to `Locale::all()`
   - Create a new translation function (e.g., `german()`)
   - Add case to `translations_for()`

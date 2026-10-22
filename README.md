# krizaka-tauri-kit

What every Krizaka **macOS app** (Tauri 2) shares, as a Rust crate with no dependency on Tauri itself, so it is tested
without a window. Decision: [ADR-005](https://github.com/krizaka/.github/blob/main/docs/adr/ADR-005-tauri-app-kit.md).

| Module | What it does |
| :-- | :-- |
| `scope` | Folder grants, stored as canonical paths. A path is readable only inside a folder the user granted: `..` and symlinks that leave a grant are refused, and the home folder, `/`, `/Users`, `/Volumes`, the system folders, `~/.ssh`, `~/.aws`, `~/Library`… can never be granted |
| `files` | Listing (folders first, hidden and partial files skipped, links never followed) and previewing (text, image, PDF within limits) inside a grant |
| `nav_guard` | A window that shows a hosted web app never leaves its origin: HTTPS (loopback may use http), no credentials, look-alike hosts refused |

```toml
[dependencies]
krizaka-tauri-kit = { git = "https://github.com/krizaka/krizaka-tauri-kit", tag = "v0.1.0" }
```

```rust
let mut scope = krizaka_tauri_kit::scope::ScopeStore::load(&settings_dir.join("scope.json"));
let root = scope.grant(&folder_chosen_with_the_native_picker)?;
let file = scope.resolve(&path_from_the_ui)?;          // the one gate every read goes through
let listing = krizaka_tauri_kit::files::list_dir(&scope.resolve(&dir)?)?;
```

Extracted from Orazaka Desktop (`krizaka/orazaka`, `apps/desktop`), whose tests came with it. macOS first (Apple Silicon, macOS 14+).

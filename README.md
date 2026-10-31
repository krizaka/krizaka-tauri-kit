# krizaka-tauri-kit

[![Quality Gate](https://sonarcloud.io/api/project_badges/measure?project=krizaka_krizaka-tauri-kit&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=krizaka_krizaka-tauri-kit) [![Security Rating](https://sonarcloud.io/api/project_badges/measure?project=krizaka_krizaka-tauri-kit&metric=security_rating)](https://sonarcloud.io/summary/new_code?id=krizaka_krizaka-tauri-kit) [![Reliability Rating](https://sonarcloud.io/api/project_badges/measure?project=krizaka_krizaka-tauri-kit&metric=reliability_rating)](https://sonarcloud.io/summary/new_code?id=krizaka_krizaka-tauri-kit) [![Maintainability Rating](https://sonarcloud.io/api/project_badges/measure?project=krizaka_krizaka-tauri-kit&metric=sqale_rating)](https://sonarcloud.io/summary/new_code?id=krizaka_krizaka-tauri-kit)

What every Krizaka **macOS app** (Tauri 2) shares, written once. Decision:
[ADR-005](https://github.com/krizaka/.github/blob/main/docs/adr/ADR-005-tauri-app-kit.md).

Two crates in this repository:

- **`krizaka-tauri-kit`** (the root): the rules, in pure Rust with no dependency on Tauri, so they are tested without a window.
- **`tauri-plugin-krizaka`** (`plugin/`): the Tauri glue that applies them in an app — one `.plugin(...)` line. Its page side,
  typed, is [`@krizaka/app-desktop`](https://ui.krizaka.com/packages/app-desktop) on npm.

| Module (`krizaka-tauri-kit`) | What it does |
| :-- | :-- |
| `scope` | Folder grants, stored as canonical paths. A path is readable only inside a folder the user granted: `..` and symlinks that leave a grant are refused, and the home folder, `/`, `/Users`, `/Volumes`, the system folders, `~/.ssh`, `~/.aws`, `~/Library`… can never be granted |
| `files` | Listing (folders first, hidden and partial files skipped, links never followed) and previewing (text, image, PDF within limits) inside a grant |
| `access` | The one gate: a granted folder's file, or a file the OS reported as dropped on the window this session (one file, never its neighbours) |
| `nav_guard` | A window stays on the app's own places (its local page `tauri://localhost`, a hosted app's HTTPS origin); a web link elsewhere opens in the browser; `file:`, `javascript:`, `data:`, other schemes, look-alike hosts are refused |
| `deep_link` | `yomizaka://app/x?y` and the product's own web links become a route inside the app (`/app/x?y`), or are refused: other hosts, `//host`, `..`, credentials, routes outside the product's prefixes |
| `menu` | The native menu bar the page describes in its own words: ids, labels and accelerators checked; the standard shortcuts (⌘Q, ⌘C, ⌘V…) reserved for macOS |
| `secrets` | Small secrets (the session token): key and value rules every store follows, an in-memory store for tests |
| `settings` | Small JSON files read leniently (a damaged file reads as the default) and written atomically |
| `update` | The update channel, `stable` or `beta`, and the HTTPS feed that names it |

| The plugin brings | Page side (`@krizaka/app-desktop`) |
| :-- | :-- |
| Folder commands (grant with the native picker, list, revoke, read, watch); `krizaka://dropped`, `krizaka://dragging`, `krizaka://folder-changed` | `createFolders` |
| The macOS Keychain (service = the bundle id, account = the key) | `keychainStorage`, the `KeyValueStorage` of `@krizaka/app-client` |
| The menu bar built from the page's spec; `krizaka://menu` | `setNativeMenu` |
| `RunEvent::Opened` → `krizaka://open` with the route; the links that opened the app kept for `take_links` | `followLinks` |
| The navigation guard on every webview | — |
| The update channel, stored on the Mac | `createUpdates` |

```toml
[dependencies]
tauri-plugin-krizaka = { git = "https://github.com/krizaka/krizaka-tauri-kit", tag = "v0.2.0" }
```

```rust
use tauri_plugin_krizaka::KitExt;

#[tauri::command]
fn read_document(app: tauri::AppHandle, path: String) -> Result<Vec<u8>, String> {
    let file = app.kit().resolve(std::path::Path::new(&path))?; // the one gate every read goes through
    std::fs::read(file).map_err(|e| e.to_string())
}

tauri::Builder::default()
    .plugin(tauri_plugin_krizaka::dialog::init())                   // the folder picker, re-exported by the kit
    .plugin(
        tauri_plugin_krizaka::Builder::new("yomizaka")              // the scheme the Info.plist declares
            .link_hosts(["www.yomizaka.com", "dev.yomizaka.com"])
            .link_prefixes(["/app"])
            .build(),
    )
    .invoke_handler(tauri::generate_handler![read_document])
```

The window's capability lists `krizaka:default` (and `core:window:allow-start-dragging` for `AppShell desktop`'s title
bar); never `fs`, `shell` or `opener`. The app registers the folder picker with `tauri_plugin_krizaka::dialog::init()` (re-exported: no dependency of its
own); a plugin registered from inside another plugin's setup deadlocks Tauri. Start an app from [`template-tauri-app`](https://github.com/krizaka/template-tauri-app).

Extracted from Orazaka Desktop (`krizaka/orazaka`, `apps/desktop`), whose tests came with it. macOS first (Apple Silicon, macOS 14+).

## Film tooling (`film/`, `.github/actions/film`)

The journeys of the Mac apps (Yomizaka, Orazaka) are filmed the same way, with one implementation: WebDriver screenshots of the
app window (`film.mjs`, no screen recording, no macOS permission), collapsed (blank start cut, idle stretches held at most 1.2 s),
joined into one mp4 (`join-cli.mjs`, refuses under 30 frames), the web clips trimmed (`trim-cli.mjs`), and a small client of the
app's embedded WebDriver (`webdriver.mjs`). No dependency; `node --test film/test/*.test.mjs`. The composite action installs
ffmpeg and outputs the directory.

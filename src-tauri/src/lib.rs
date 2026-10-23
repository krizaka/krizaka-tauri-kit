//! __Name__ for the Mac. Everything a Krizaka Mac app does the same way — the folders the person grants and the files they
//! drop (every path through the folder scope), the session in the Keychain, the native menu, deep links, the navigation
//! guard, the update channel — is `tauri-plugin-krizaka` (krizaka-tauri-kit). Only the product's own commands go here, each
//! resolving its paths through the kit's one gate:
//!
//! ```ignore
//! use tauri_plugin_krizaka::KitExt;
//!
//! #[tauri::command]
//! fn read_document(app: tauri::AppHandle, path: String) -> Result<Vec<u8>, String> {
//!     let file = app.kit().resolve(std::path::Path::new(&path))?;
//!     std::fs::read(file).map_err(|e| e.to_string())
//! }
//! ```

pub fn run() {
    tauri::Builder::default()
        // The API calls go through Rust's HTTP client (no CORS), to the addresses the capability allows.
        .plugin(tauri_plugin_http::init())
        // The folder picker the kit's commands open (re-exported by the kit).
        .plugin(tauri_plugin_krizaka::dialog::init())
        .plugin(
            tauri_plugin_krizaka::Builder::new("__slug__")
                .link_hosts(["www.__slug__.com", "dev.__slug__.com"])
                .link_prefixes(["/app"])
                .build(),
        )
        .run(tauri::generate_context!())
        .expect("error while running __Name__");
}

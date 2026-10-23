//! What every Krizaka macOS app (Tauri 2) shares, with no dependency on Tauri itself so it is tested without a window.
//! The Tauri glue that wires these rules into an app is the `tauri-plugin-krizaka` crate of this repository.
//!
//! - [`scope`]: the folder grants. A path is readable only inside a folder the user granted, compared on canonical paths.
//! - [`files`]: listing and previewing inside a grant.
//! - [`access`]: who may read what: the granted folders, and the files dropped on the window this session.
//! - [`nav_guard`]: a window stays on the app's own places; a web link elsewhere opens in the browser.
//! - [`deep_link`]: `product://…` and the product's web links become a route inside the app, or are refused.
//! - [`menu`]: the native menu bar the page describes, checked before it is built.
//! - [`secrets`]: small secrets (the session token) and their rules; the Keychain store is in the plugin.
//! - [`settings`]: small JSON files written atomically.
//! - [`update`]: the update channel (`stable`, `beta`) and its feed.

pub mod access;
pub mod deep_link;
pub mod files;
pub mod menu;
pub mod nav_guard;
pub mod scope;
pub mod secrets;
pub mod settings;
pub mod update;

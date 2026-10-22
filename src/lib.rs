//! What every Krizaka macOS app (Tauri 2) shares, with no dependency on Tauri itself so it is tested without a window.
//!
//! - [`scope`]: the folder grants. A path is readable only inside a folder the user granted, compared on canonical paths.
//! - [`files`]: listing and previewing inside a grant.
//! - [`access`]: who may read what: the granted folders, and the files dropped on the window this session.
//! - [`nav_guard`]: a window that shows a hosted web app never leaves its origin.

pub mod access;
pub mod files;
pub mod nav_guard;
pub mod scope;

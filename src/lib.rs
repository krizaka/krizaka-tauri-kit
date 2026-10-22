//! What every Krizaka macOS app (Tauri 2) shares, with no dependency on Tauri itself so it is tested without a window.
//!
//! - [`scope`]: the folder grants. A path is readable only inside a folder the user granted, compared on canonical paths.
//! - [`files`]: listing and previewing inside a grant.
//! - [`nav_guard`]: a window that shows a hosted web app never leaves its origin.

pub mod files;
pub mod nav_guard;
pub mod scope;

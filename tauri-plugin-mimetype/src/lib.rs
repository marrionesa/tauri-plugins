// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Map a file extension or file path to a MIME type, from Rust.
//!
//! ## Why this plugin exists
//!
//! Browsers expose **no API** for mapping a path or extension to a MIME type.
//! The closest equivalent is the [`mime-types`](https://www.npmjs.com/package/mime-types)
//! npm package (~10 KB minified+gzipped), which re-implements the lookup
//! table in JavaScript. There is no built-in `navigator.mimeFromPath()` or
//! similar.
//!
//! This plugin fills that gap by routing through the pure-Rust
//! [`mime_guess`](https://crates.io/crates/mime_guess) crate, which has no
//! platform bindings, performs no I/O, and runs entirely in-memory. The result
//! is **0 KB of app JS** beyond the tiny `invoke` wrapper.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_mimetype::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { fromExtension, fromPath } from '@marrionesa/plugin-mimetype'
//! console.log(await fromExtension('json'))      // "application/json"
//! console.log(await fromPath('/x/y.html'))      // "text/html"
//! ```
//!
//! ## Use cases
//!
//! - Setting the `Content-Type` header for an upload from a Tauri app.
//! - Picking an icon or viewer based on a downloaded file's extension.
//! - Validating that a user-supplied filename maps to an expected MIME family.
//! - Replacing the [`mime-types`](https://www.npmjs.com/package/mime-types)
//!   npm package in Tauri apps to save ~10 KB of bundle.

#![doc(
    html_logo_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png",
    html_favicon_url = "https://github.com/tauri-apps/tauri/raw/dev/app-icon.png"
)]

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

mod commands;
mod error;

pub use error::{Error, Result};

/// Initializes the plugin.
///
/// Register on your `tauri::Builder::default().plugin(...)` chain.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("mimetype")
        .invoke_handler(tauri::generate_handler![
            commands::from_extension,
            commands::from_path,
        ])
        .build()
}

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Generate v4 (random) and v7 (time-ordered) UUIDs natively from Rust.
//!
//! ## Why this plugin exists
//!
//! Browsers expose [`crypto.randomUUID()`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/randomUUID)
//! which only produces v4 UUIDs. **v7 UUIDs** (Unix-millisecond-timestamp
//! prefixed, sortable, collision-resistant) are not available in any browser
//! API and require either a JS polyfill (~6 KB bundle) or this plugin.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_uuid::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { v4, v7 } from '@marrionesa/plugin-uuid'
//! console.log(await v4()) // "f0e1d2c3-b4a5-4987-9abc-def012345678"
//! console.log(await v7()) // "0192f5c7-2b8d-7abc-9def-0123456789ab" (sortable!)
//! ```
//!
//! ## Use cases
//!
//! - Database primary keys (v7 keeps index locality).
//! - Distributed-systems event IDs (v7 sorts by insertion time).
//! - Anonymous client IDs (v4 suffices when ordering is irrelevant).
//! - Replacing the [`uuid`](https://www.npmjs.com/package/uuid) npm package
//!   in Tauri apps to save bundle size.

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
    Builder::new("uuid")
        .invoke_handler(tauri::generate_handler![
            commands::generate_v4,
            commands::generate_v7,
        ])
        .build()
}

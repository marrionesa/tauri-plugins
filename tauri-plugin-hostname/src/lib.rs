// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Read the system hostname and the current username from Rust, exposed to
//! JavaScript.
//!
//! ## Why this plugin exists
//!
//! The official [`tauri-plugin-os`](https://v2.tauri.app/plugin/os/) crate ships
//! a Rust-only `os::hostname()` function — but it is **NOT exposed to
//! JavaScript by default**. The os plugin's default permission set does **not**
//! include an `allow-hostname` permission (you can verify this by inspecting
//! `plugins/os/permissions/` in the upstream `tauri-apps/plugins-workspace`
//! repo), so the function is callable only from Rust code that links against
//! the os plugin directly.
//!
//! This plugin fills that JS-visible gap by re-exporting hostname via its own
//! `plugin:hostname|hostname` IPC channel — and additionally exposes
//! `username()`, which the os plugin does not provide at all.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_hostname::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { hostname, username } from '@marrionesa/plugin-hostname'
//! console.log(await hostname()) // "my-laptop"
//! console.log(await username())  // "alice"
//! ```
//!
//! ## Relationship with `tauri-plugin-os`
//!
//! The os plugin's `hostname()` is a Rust-only function: it is not exposed to
//! JavaScript by default (its default permission set has no `allow-hostname`),
//! and it has no `username()` counterpart at all. This plugin exposes both over
//! its own IPC channel, so the same code works from any Tauri context.
//!
//! If the os plugin later adds `allow-hostname` and a `username()` command,
//! this plugin can be deprecated in their favour: the JavaScript signatures are
//! identical, so no consumer code would need to change.

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
    Builder::new("hostname")
        .invoke_handler(tauri::generate_handler![
            commands::hostname,
            commands::username,
        ])
        .build()
}

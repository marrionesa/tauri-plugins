// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Read the system's IANA timezone and the current UTC offset in seconds.
//!
//! ## Why this plugin exists
//!
//! The official [`tauri-plugin-os`](https://v2.tauri.app/plugin/os/) exposes
//! `locale()` but **not** timezone. Browsers expose
//! [`Intl.DateTimeFormat().resolvedOptions().timeZone`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#timezone)
//! which returns the IANA name only — but the DST-aware UTC offset must be
//! obtained via [`Date.prototype.getTimezoneOffset()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset),
//! which returns **minutes west of UTC** (signed inverted: positive west, negative east)
//! — an error-prone convention that has bitten countless developers.
//!
//! This plugin returns both pieces in the correct units:
//!
//! - `get_timezone()` → IANA identifier (e.g. `"Europe/Madrid"`), or `None` if
//!   the OS cannot determine it.
//! - `get_utc_offset_seconds()` → current DST-aware UTC offset in **seconds**,
//!   **east of UTC positive** (e.g. `7200` for `Europe/Madrid` in summer).
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_timezone::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { getTimezone, getUtcOffsetSeconds } from '@marrionesa/plugin-timezone'
//! const tz = await getTimezone()        // "Europe/Madrid"
//! const offset = await getUtcOffsetSeconds() // 7200 (UTC+2 in summer, DST-aware)
//! ```
//!
//! ## Use cases
//!
//! - Pre-populating a timezone picker in a settings UI.
//! - Sending the user's local timezone to a backend for scheduling/cron jobs.
//! - Rendering timestamps server-side with the correct DST offset without
//!   relying on the inverted `getTimezoneOffset()` sign convention.

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
    Builder::new("timezone")
        .invoke_handler(tauri::generate_handler![
            commands::get_timezone,
            commands::get_utc_offset_seconds,
        ])
        .build()
}

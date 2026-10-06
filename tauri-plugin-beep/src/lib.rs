// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Play the **system alert sound** (the OS "beep") from Rust.
//!
//! ## Why this plugin exists
//!
//! The existing [`tauri-plugin-sound-player`](https://crates.io/crates/tauri-plugin-sound-player)
//! plays mp3 / ogg / wav files, which means you have to **bundle an audio
//! file** with your app and ship a media decoder. That's overkill when all you
//! want is a one-shot attention sound — e.g. a terminal emulator alerting the
//! user on bell, or a tray app nudging the user on an unread notification.
//!
//! This plugin instead asks the **operating system** to play its native alert
//! sound — `MessageBeep(MB_OK)` on Windows, `afplay` of the system Tink sound
//! on macOS, `paplay` of the freedesktop bell on Linux. Zero audio files
//! bundled; zero decoder code; zero extra weight on the IPC layer.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_beep::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { beep } from '@marrionesa/plugin-beep'
//! await beep()  // plays the OS alert sound
//! ```
//!
//! ## Use cases
//!
//! - Terminal-emulator bell (replaces the JS `\x07` byte that only fires in a
//!   real TTY — this fires in the GUI webview too).
//! - Tray-app attention sound on a new notification.
//! - Form-validation error feedback.
//! - Replacing hand-rolled `new Audio('/ding.mp3').play()` with a no-asset
//!   alternative that respects the user's system sound theme.

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
    Builder::new("beep")
        .invoke_handler(tauri::generate_handler![commands::beep])
        .build()
}

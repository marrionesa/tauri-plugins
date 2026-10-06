// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// Errors the plugin can return to JavaScript.
///
/// The current `beep` command is fallible in only a handful of cases:
///
/// - Windows: `MessageBeep` returned `0` (no default sound configured / audio
///   service unavailable).
/// - macOS:   `afplay` failed to spawn (extremely rare — it ships with macOS).
/// - Linux:   never returned directly — the `paplay` failure path falls back to
///   the terminal bell and returns `Ok(())`.
/// - Mobile:  the platform is unsupported.
///
/// The `Anyhow(String)` variant carries the platform-specific diagnostic;
/// `Tauri` is reserved for future IPC-layer errors that `tauri::Error` exposes.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error("{0}")]
    Anyhow(String),
}

pub type Result<T> = std::result::Result<T, Error>;

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

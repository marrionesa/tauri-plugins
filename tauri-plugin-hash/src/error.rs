// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// Errors the plugin can return to JavaScript.
///
/// - `Io` covers filesystem failures from `hash_file` (file missing, permission
///   denied, I/O error mid-stream, etc.).
/// - `Tauri` covers IPC / runtime failures from the Tauri framework itself.
/// - `Anyhow` covers unsupported-algorithm errors raised by either command.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

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

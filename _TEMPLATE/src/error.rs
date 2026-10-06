// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// All errors your plugin can return to JavaScript. Implement `From<T>` for each
/// underlying error type you add. The `serde::Serialize` impl is required so the
/// error can be returned through the Tauri IPC bridge.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    // Add more variants as your plugin grows:
    // #[error(transparent)]
    // SomeCrate(#[from] some_crate::Error),
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

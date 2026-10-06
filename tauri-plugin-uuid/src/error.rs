// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// Errors the plugin can return to JavaScript.
///
/// In practice, the current `generate_v4` / `generate_v7` commands never fail —
/// the `uuid` crate's randomness is sourced from the OS CSPRNG and is
/// infallible on every supported platform. The enum is kept here so that
/// future commands (e.g. parsing, encoding) can extend it without breaking the
/// public API.
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

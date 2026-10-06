// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::Serialize;

/// Errors the plugin can return to JavaScript.
///
/// In practice, the current `get_timezone` / `get_utc_offset_seconds`
/// commands never fail — `get_utc_offset_seconds` is infallible (chrono's
/// `Local::now()` reads the system clock and always returns an offset), and
/// `get_timezone` returns `Option<String>` so a missing timezone is
/// represented as `None` rather than as an `Err`. The enum is kept here so
/// that future commands (e.g. parsing a specific timezone string, listing
/// all zones) can extend it without breaking the public API.
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

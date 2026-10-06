// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Stream-compute SHA-256, SHA-512, and MD5 hashes of strings and files from Rust.
//!
//! ## Why this plugin exists
//!
//! Browsers expose [`SubtleCrypto.digest()`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest)
//! which can compute SHA-256 / SHA-384 / SHA-512 — but it requires loading the
//! **entire input into memory as an `ArrayBuffer`** before hashing begins. For
//! files larger than the available RAM (or the browser's per-tab memory limit,
//! typically 2 GB on 64-bit Chromium) this throws `QuotaExceededError` or simply
//! crashes the renderer.
//!
//! This plugin fills that gap by streaming the file in **8 KB chunks** through a
//! Rust [`tokio::fs::File`](https://docs.rs/tokio/latest/tokio/fs/struct.File.html)
//! reader, feeding each chunk into an incremental hasher. Memory usage is
//! **constant** (~8 KB + the hash state) regardless of file size — multi-GB ISOs,
//! database dumps, and video archives hash without breaking a sweat.
//!
//! ## Quick start
//!
//! ```rust,ignore
//! tauri::Builder::default()
//!     .plugin(tauri_plugin_hash::init())
//!     .run(tauri::generate_context!())
//!     .expect("error while running tauri application");
//! ```
//!
//! ## JavaScript usage
//!
//! ```typescript,ignore
//! import { hashString, hashFile } from '@marrionesa/plugin-hash'
//! // Hash a UTF-8 string
//! const docHash = await hashString('hello world', 'sha256')
//! // "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
//! // Stream a 4 GB Ubuntu ISO without loading it into memory
//! const isoHash = await hashFile('/path/to/ubuntu-24.04.iso', 'sha256')
//! ```
//!
//! ## Use cases
//!
//! - Verifying downloaded ISOs / installers against publisher checksums.
//! - Content-addressing user uploads before they hit the network.
//! - Deduplicating large blobs (same hash → same content).
//! - Computing git-style object IDs client-side.
//!
//! ## Algorithms
//!
//! | Algorithm | Crate        | Output length (hex chars) |
//! |-----------|--------------|---------------------------|
//! | `sha256`  | `sha2`       | 64                        |
//! | `sha512`  | `sha2`       | 128                       |
//! | `md5`     | `md-5`       | 32                        |
//!
//! MD5 is included for legacy / non-security checksums only — never use it for
//! passwords, signatures, or any adversarial context.

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
    Builder::new("hash")
        .invoke_handler(tauri::generate_handler![
            commands::hash_string,
            commands::hash_file,
        ])
        .build()
}

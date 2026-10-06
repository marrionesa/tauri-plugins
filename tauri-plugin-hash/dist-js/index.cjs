'use strict';

var core = require('@tauri-apps/api/core');

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/**
 * JavaScript bindings for the `tauri-plugin-hash` plugin.
 *
 * Exposes two async functions backed by Rust `#[tauri::command]` handlers
 * registered in `src/lib.rs`:
 *
 *   - `hashString(input, algorithm)` -> hex digest of a UTF-8 string
 *   - `hashFile(path, algorithm)`    -> hex digest of a file's contents,
 *                                        streamed in 8 KB chunks (constant memory)
 *
 * Browsers' `SubtleCrypto.digest()` requires loading the entire file into
 * memory as an ArrayBuffer — this fails for files larger than available RAM.
 * This plugin streams, so it handles multi-GB files with constant memory.
 * See https://docs.rs/sha2 and https://docs.rs/md-5 for the underlying crates.
 *
 * @example
 * ```ts
 * import { hashString, hashFile } from '@marrionesa/plugin-hash'
 *
 * const docHash = await hashString('hello world', 'sha256')
 * // "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
 *
 * const isoHash = await hashFile('/path/to/ubuntu-24.04.iso', 'sha256')
 * ```
 */
/** Compute the hash of a UTF-8 string. Returns the hex-encoded digest. */
async function hashString(input, algorithm) {
    return await core.invoke('plugin:hash|hash_string', { input, algorithm });
}
/** Compute the hash of a file's contents (streamed in 8 KB chunks). Returns the hex-encoded digest. */
async function hashFile(path, algorithm) {
    return await core.invoke('plugin:hash|hash_file', { path, algorithm });
}

exports.hashFile = hashFile;
exports.hashString = hashString;
//# sourceMappingURL=index.cjs.map

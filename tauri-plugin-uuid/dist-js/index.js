import { invoke } from '@tauri-apps/api/core';

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/**
 * JavaScript bindings for the `tauri-plugin-uuid` plugin.
 *
 * Exposes two async functions backed by Rust `#[tauri::command]` handlers
 * registered in `src/lib.rs`:
 *
 *   - `v4()` -> random v4 UUID (RFC 4122)
 *   - `v7()` -> time-ordered v7 UUID (Unix-ms prefixed, sortable)
 *
 * The `v7` variant has **no browser equivalent** — that is the gap this plugin
 * fills. See https://github.com/uuid-rs/uuid for the underlying Rust crate.
 *
 * @example
 * ```ts
 * import { v4, v7 } from '@marrionesa/plugin-uuid'
 *
 * const userId = await v4()   // "f0e1d2c3-b4a5-4987-9abc-def012345678"
 * const eventId = await v7()  // "0192f5c7-2b8d-7abc-9def-0123456789ab" (sortable!)
 * ```
 */
/**
 * Generate a random v4 UUID (RFC 4122 variant, 122 bits of randomness).
 *
 * @returns Canonical hyphenated form, e.g. `"f0e1d2c3-b4a5-4987-9abc-def012345678"`.
 *
 * Equivalent to the browser's `crypto.randomUUID()` but routed through Rust
 * so the same code path works in every Tauri context (including mobile and
 * headless CLI builds without a webview).
 */
async function v4() {
    return await invoke('plugin:uuid|generate_v4');
}
/**
 * Generate a v7 UUID: 48-bit Unix-ms timestamp + 74 bits of randomness.
 *
 * The result is **lexicographically sortable** by creation time, making it
 * ideal for database primary keys and distributed-systems event IDs.
 *
 * @returns Canonical hyphenated form, e.g. `"0192f5c7-2b8d-7abc-9def-0123456789ab"`.
 *
 * This function has **no browser equivalent** — `crypto.randomUUID()` only
 * emits v4 UUIDs.
 */
async function v7() {
    return await invoke('plugin:uuid|generate_v7');
}

export { v4, v7 };
//# sourceMappingURL=index.js.map

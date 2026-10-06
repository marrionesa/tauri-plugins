// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * JavaScript bindings for the `tauri-plugin-hostname` plugin.
 *
 * Exposes two async functions backed by Rust `#[tauri::command]` handlers
 * registered in `src/lib.rs`:
 *
 *   - `hostname()` -> system hostname (UTF-8 string)
 *   - `username()`  -> current user's login name
 *
 * The official `tauri-plugin-os` has a `hostname()` Rust function but it is
 * NOT exposed to JavaScript by default (no `allow-hostname` permission in its
 * default set). This plugin fills that JS-visible gap and also adds
 * `username()`, which the os plugin does not have at all.
 *
 * @example
 * ```ts
 * import { hostname, username } from '@marrionesa/plugin-hostname'
 *
 * const host = await hostname() // "my-laptop"
 * const user = await username()  // "alice"
 * ```
 */
import { invoke } from '@tauri-apps/api/core'

/**
 * Returns the system's hostname as a UTF-8 string.
 *
 * Note: the official `tauri-plugin-os` has a `hostname()` Rust function but
 * it is NOT exposed to JavaScript by default (no `allow-hostname` permission
 * in its default set). This plugin fills that JS-visible gap.
 *
 * @returns The system hostname, e.g. `"my-laptop"`. Falls back to a lossy
 *          conversion (Unicode replacement characters) on the very rare
 *          misconfigured system where libc::gethostname returns non-UTF-8
 *          bytes.
 */
export async function hostname(): Promise<string> {
  return await invoke<string>('plugin:hostname|hostname')
}

/**
 * Returns the current user's username (login name) as reported by the OS.
 * On Unix this is the effective UID's password entry; on Windows it is the
 * `USERNAME` environment variable with a `GetUserNameW` fallback.
 *
 * @returns The current user's login name, e.g. `"alice"`.
 */
export async function username(): Promise<string> {
  return await invoke<string>('plugin:hostname|username')
}

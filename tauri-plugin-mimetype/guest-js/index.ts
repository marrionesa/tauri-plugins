// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * JavaScript bindings for the `tauri-plugin-mimetype` plugin.
 *
 * Exposes two async functions backed by Rust `#[tauri::command]` handlers
 * registered in `src/lib.rs`:
 *
 *   - `fromExtension(ext)` -> MIME type string for a bare extension
 *   - `fromPath(path)`      -> MIME type string for a file path
 *
 * Both functions return `string | null`: `null` indicates that `mime_guess`
 * had no entry for the supplied extension. The browser has **no equivalent**
 * for path→MIME detection — that is the gap this plugin fills.
 *
 * @example
 * ```ts
 * import { fromExtension, fromPath } from '@marrionesa/plugin-mimetype'
 *
 * await fromExtension('json')   // "application/json"
 * await fromExtension('.html') // "text/html" (leading dot tolerated)
 * await fromPath('/x/y.html')  // "text/html"
 * await fromPath('/x/y.zzz')   // null (unknown extension)
 * ```
 */
import { invoke } from '@tauri-apps/api/core'

/** Map a file extension (e.g. "json", "html") to a MIME type. Returns null if unknown. */
export async function fromExtension(extension: string): Promise<string | null> {
  return await invoke<string | null>('plugin:mimetype|from_extension', { extension })
}

/** Map a file path (e.g. "/path/to/file.json") to a MIME type via its extension. Returns null if unknown. */
export async function fromPath(path: string): Promise<string | null> {
  return await invoke<string | null>('plugin:mimetype|from_path', { path })
}

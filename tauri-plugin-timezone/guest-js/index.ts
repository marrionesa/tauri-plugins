// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * JavaScript bindings for the `tauri-plugin-timezone` plugin.
 *
 * Exposes two async functions backed by Rust `#[tauri::command]` handlers
 * registered in `src/lib.rs`:
 *
 *   - `getTimezone()` -> IANA timezone identifier (e.g. "Europe/Madrid"), or null
 *   - `getUtcOffsetSeconds()` -> current DST-aware UTC offset in seconds (east positive)
 *
 * The official `tauri-plugin-os` exposes `locale()` but NOT timezone. Browsers
 * expose `Intl.DateTimeFormat().resolvedOptions().timeZone` (IANA name only)
 * but the DST-aware UTC offset requires `Date.prototype.getTimezoneOffset()`,
 * which returns minutes and is signed inverted (positive west of UTC) —
 * error-prone. This plugin returns both values in the correct units. See
 * https://crates.io/crates/iana-time-zone and https://crates.io/crates/chrono
 * for the underlying Rust crates.
 *
 * @example
 * ```ts
 * import { getTimezone, getUtcOffsetSeconds } from '@marrionesa/plugin-timezone'
 *
 * const tz = await getTimezone()        // "Europe/Madrid"
 * const offset = await getUtcOffsetSeconds() // 7200 (UTC+2 in summer, DST-aware)
 * ```
 */
import { invoke } from '@tauri-apps/api/core'

/**
 * Returns the system's IANA timezone identifier (e.g. "Europe/Madrid", "America/New_York").
 * Returns null if the timezone cannot be determined.
 */
export async function getTimezone(): Promise<string | null> {
  return await invoke<string | null>('plugin:timezone|get_timezone')
}

/**
 * Returns the current UTC offset in seconds (east of UTC is positive).
 * DST-aware: Europe/Madrid returns 7200 in summer (UTC+2), 3600 in winter (UTC+1).
 * Note: this is the OPPOSITE sign convention from `Date.prototype.getTimezoneOffset()`,
 * which returns minutes west of UTC (negative east of UTC).
 */
export async function getUtcOffsetSeconds(): Promise<number> {
  return await invoke<number>('plugin:timezone|get_utc_offset_seconds')
}

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/**
 * REPLACE ME — JavaScript bindings for the `<name>` plugin.
 *
 * Each function maps to a Rust `#[tauri::command]` registered in
 * `src/lib.rs`. The IPC channel name follows the convention:
 *
 *     plugin:<plugin-name>|<command-name>
 *
 * The command name uses snake_case on the Rust side; Tauri automatically
 * converts to camelCase for the JS payload keys (see
 * https://v2.tauri.app/develop/calling-rust/#passing-arguments).
 *
 * @example
 * ```ts
 * import { ping } from '@marrionesa/plugin-template'
 * const reply = await ping('hello')
 * console.log(reply) // "hello"
 * ```
 */
import { invoke } from '@tauri-apps/api/core'

export interface PingRequest {
  value: string
}

export interface PingResponse {
  value: string
}

/** Echo `value` back to the caller. Smoke-test command. */
export async function ping(value: string): Promise<PingResponse> {
  return await invoke<PingResponse>('plugin:template|ping', {
    payload: { value },
  })
}

/**
 * Generate a random v4 UUID (RFC 4122 variant, 122 bits of randomness).
 *
 * @returns Canonical hyphenated form, e.g. `"f0e1d2c3-b4a5-4987-9abc-def012345678"`.
 *
 * Equivalent to the browser's `crypto.randomUUID()` but routed through Rust
 * so the same code path works in every Tauri context (including mobile and
 * headless CLI builds without a webview).
 */
export declare function v4(): Promise<string>;
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
export declare function v7(): Promise<string>;

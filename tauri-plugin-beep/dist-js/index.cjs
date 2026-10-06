'use strict';

var core = require('@tauri-apps/api/core');

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/**
 * JavaScript bindings for the `tauri-plugin-beep` plugin.
 *
 * Exposes one async function backed by a Rust `#[tauri::command]` handler
 * registered in `src/lib.rs`:
 *
 *   - `beep()` -> plays the OS-native alert sound.
 *
 * The existing `tauri-plugin-sound-player` plays mp3/ogg/wav files (requires
 * bundling audio + a media decoder); this plugin plays the OS-native alert
 * sound — zero audio files bundled, useful for terminal-emulator and tray
 * apps. See the README for the per-platform implementation details.
 *
 * @example
 * ```ts
 * import { beep } from '@marrionesa/plugin-beep'
 *
 * await beep()  // plays the OS alert sound
 * ```
 */
/**
 * Play the system alert sound.
 *
 * - Windows: `MessageBeep(MB_OK)` — the default Windows alert sound.
 * - macOS:   `afplay /System/Library/Sounds/Tink.aiff`.
 * - Linux:   `paplay /usr/share/sounds/freedesktop/stereo/bell.oga` with a
 *            terminal-bell (`\x07`) fallback if `paplay` is missing.
 * - Android/iOS: rejects with an error (no equivalent system alert API).
 *
 * @throws if the platform is unsupported (mobile) or the underlying call fails.
 */
async function beep() {
    await core.invoke('plugin:beep|beep');
}

exports.beep = beep;
//# sourceMappingURL=index.cjs.map

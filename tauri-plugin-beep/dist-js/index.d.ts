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
export declare function beep(): Promise<void>;

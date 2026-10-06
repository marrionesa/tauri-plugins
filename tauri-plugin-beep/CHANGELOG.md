# Changelog

## 0.1.0

Initial release of `tauri-plugin-beep`.

- Added `beep` command — plays the system alert sound.
  - Windows: `MessageBeep(MB_OK)` via `windows-sys` v0.59.
  - macOS: spawns `afplay /System/Library/Sounds/Tink.aiff` (preinstalled on every macOS).
  - Linux: spawns `paplay /usr/share/sounds/freedesktop/stereo/bell.oga` with a `\x07` stderr fallback if `paplay` is missing.
  - Android/iOS: returns `Err(UnsupportedPlatform)`.
- JavaScript binding exposes `beep()` async function.
- Default permission set enables the command on desktop platforms.
- Single Windows-only dependency: `windows-sys` v0.59 with `Win32_UI_WindowsAndMessaging` feature.
- Dual-licensed under `Apache-2.0 OR MIT`.

---

This file is managed by [covector](https://github.com/jbolda/covector) at release
time. Each entry is generated from the `.changes/*.md` files merged between
releases.

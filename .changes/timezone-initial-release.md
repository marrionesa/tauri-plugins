---
'tauri-plugin-timezone': minor:feat
'tauri-plugin-timezone-js': minor:feat
---

Initial release of `tauri-plugin-timezone`.

Adds two commands backed by pure-Rust crates (`iana-time-zone` v0.1, `chrono` v0.4):

- `get_timezone()` — returns the system IANA timezone (e.g. "Europe/Madrid").
- `get_utc_offset_seconds()` — returns the current DST-aware UTC offset in seconds (east of UTC is positive).

The official `tauri-plugin-os` exposes `locale()` but NOT timezone. Browsers expose `Intl.DateTimeFormat().resolvedOptions().timeZone` (IANA name only) but the DST-aware UTC offset via `Date.prototype.getTimezoneOffset()` returns minutes and is signed inverted (positive west of UTC), which is error-prone. This plugin returns both values in correct units.

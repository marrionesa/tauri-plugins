# Changelog

## 0.1.0

Initial release of `tauri-plugin-timezone`.

- Added `get_timezone` command — returns the system IANA timezone (e.g. "Europe/Madrid").
- Added `get_utc_offset_seconds` command — returns the current DST-aware UTC offset in seconds (east positive).
- JavaScript bindings expose `getTimezone()` and `getUtcOffsetSeconds()` async functions.
- Default permission set enables both commands (safe — no input, no I/O, no side effects).
- Pure-Rust dependencies: `iana-time-zone` v0.1, `chrono` v0.4.
- Dual-licensed under `Apache-2.0 OR MIT`.

---

This file is managed by [covector](https://github.com/jbolda/covector) at release
time. Each entry is generated from the `.changes/*.md` files merged between
releases.

# Changelog

All notable changes to the plugins in this repository are documented here.
Each plugin also keeps its own `tauri-plugin-<name>/CHANGELOG.md` with the same
version history.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and each package follows [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.1.0 — 2026-10-06

First public release. Six plugins, all published on crates.io and npm under the
same version.

### Added

- **`tauri-plugin-beep`** — plays the OS-native alert sound with no audio asset
  bundled. `beep()` uses `MessageBeep` on Windows, `afplay` on macOS and
  `paplay` on Linux, with a terminal-bell fallback. Mobile reports the platform
  as unsupported.
- **`tauri-plugin-hash`** — `hash_string` and `hash_file` for SHA-256, SHA-512
  and MD5. Files are streamed in 8 KB chunks, so memory use is constant
  regardless of file size.
- **`tauri-plugin-hostname`** — `hostname()` and `username()`, both exposed to
  JavaScript (the official `tauri-plugin-os` does not expose the hostname to JS
  at all, and has no username command).
- **`tauri-plugin-mimetype`** — `from_extension` and `from_path` map an
  extension or a file path to a MIME type, without any I/O.
- **`tauri-plugin-timezone`** — `get_timezone` returns the IANA identifier and
  `get_utc_offset_seconds` the DST-aware offset in **seconds, east positive**.
- **`tauri-plugin-uuid`** — `generate_v4` and `generate_v7`, the latter being
  time-ordered and therefore lexicographically sortable.

### Repository

- `_TEMPLATE/` scaffold for adding a new plugin.
- Rust workspace plus pnpm workspace, with a single `Cargo.lock`.
- CI workflows (`fmt`, `lint-rust`, `test-rust`, `mirror`) that run **on demand
  only**, plus `dependabot` for dependency updates.

[0.1.0]: https://github.com/marrionesa/tauri-plugins/releases/tag/v0.1.0

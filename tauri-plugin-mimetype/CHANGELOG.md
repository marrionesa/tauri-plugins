# Changelog

## 0.1.0

Initial release of `tauri-plugin-mimetype`.

- Added `from_extension` command — maps a file extension (e.g. "json") to a MIME type.
- Added `from_path` command — maps a file path (e.g. "/x/y.json") to a MIME type via its extension.
- JavaScript bindings expose `fromExtension()` and `fromPath()` async functions.
- Default permission set enables both commands (safe — read-only, no I/O).
- Single pure-Rust dependency: `mime_guess` v2.
- Dual-licensed under `Apache-2.0 OR MIT`.

---

This file is managed by [covector](https://github.com/jbolda/covector) at release
time. Each entry is generated from the `.changes/*.md` files merged between
releases.

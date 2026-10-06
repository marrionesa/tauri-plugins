# Changelog

## 0.1.0

Initial release of `tauri-plugin-uuid`.

- Added `generate_v4` command — produces a random RFC-4122 v4 UUID.
- Added `generate_v7` command — produces a time-ordered v7 UUID (Unix-ms
  timestamp prefix + 74 bits of randomness).
- JavaScript bindings expose `v4()` and `v7()` async functions.
- Default permission set enables both commands (safe — no user input, no I/O).
- Single pure-Rust dependency: `uuid` v1.15 with `v4` + `v7` features.
- Dual-licensed under `Apache-2.0 OR MIT`.

---

This file is managed by [covector](https://github.com/jbolda/covector) at release
time. Each entry is generated from the `.changes/*.md` files merged between
releases.

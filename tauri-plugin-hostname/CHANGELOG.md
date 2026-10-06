# Changelog

## 0.1.0

Initial release of `tauri-plugin-hostname`.

- Added `hostname` command — returns the system hostname as a UTF-8 string.
- Added `username` command — returns the current user's login name.
- JavaScript bindings expose `hostname()` and `username()` async functions.
- Default permission set enables both commands (safe — no input, no I/O, no side effects; hostname is a low-sensitivity machine identifier).
- Pure-Rust dependencies: `gethostname` v0.5, `whoami` v1.5.
- Dual-licensed under `Apache-2.0 OR MIT`.

---

This file is managed by [covector](https://github.com/jbolda/covector) at release
time. Each entry is generated from the `.changes/*.md` files merged between
releases.

# Changelog

## 0.1.0

Initial release of `tauri-plugin-hash`.

- Added `hash_string` command — computes SHA-256/SHA-512/MD5 of a UTF-8 string.
- Added `hash_file` command — streams a file in 8 KB chunks and computes the hash with constant memory.
- JavaScript bindings expose `hashString()` and `hashFile()` async functions plus a `HashAlgorithm` type (`'sha256' | 'sha512' | 'md5'`).
- Default permission set enables both commands (note: `hash_file` reads arbitrary paths — restrict via capability if needed).
- Pure-Rust dependencies: `sha2` v0.10, `md-5` v0.10, `hex` v0.4, `tokio` v1 (fs + io-util features).
- Dual-licensed under `Apache-2.0 OR MIT`.

---

This file is managed by [covector](https://github.com/jbolda/covector) at release
time. Each entry is generated from the `.changes/*.md` files merged between
releases.

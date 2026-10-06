---
'tauri-plugin-hash': minor:feat
'tauri-plugin-hash-js': minor:feat
---

Initial release of `tauri-plugin-hash`.

Adds two async commands backed by pure-Rust crypto crates (`sha2` v0.10, `md-5` v0.10, `hex` v0.4):

- `hash_string(input, algorithm)` — computes SHA-256 / SHA-512 / MD5 of a UTF-8 string.
- `hash_file(path, algorithm)` — streams a file in 8 KB chunks via tokio and computes the hash with constant memory.

Browsers' `SubtleCrypto.digest()` requires loading the entire file into memory as an ArrayBuffer — this fails for files larger than available RAM. This plugin streams, so it handles multi-GB files with constant memory.

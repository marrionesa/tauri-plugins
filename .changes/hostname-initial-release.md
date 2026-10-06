---
'tauri-plugin-hostname': minor:feat
'tauri-plugin-hostname-js': minor:feat
---

Initial release of `tauri-plugin-hostname`.

Adds two commands backed by pure-Rust crates (`gethostname` v0.5, `whoami` v1.5):

- `hostname()` — returns the system hostname as a UTF-8 string.
- `username()` — returns the current user's login name.

The official `tauri-plugin-os` has a `hostname()` Rust function but it is **NOT exposed to JavaScript by default** (no `allow-hostname` permission in its default set). This plugin fills that JS-visible gap and also adds `username()`, which the os plugin does not have at all.

Rejection risk: HIGH if submitted to the official monorepo (maintainers may say "send a PR to the os plugin instead"). Useful as a standalone third-party plugin regardless.

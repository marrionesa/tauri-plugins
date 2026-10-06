---
'tauri-plugin-mimetype': minor:feat
'tauri-plugin-mimetype-js': minor:feat
---

Initial release of `tauri-plugin-mimetype`.

Adds two commands backed by the pure-Rust `mime_guess` crate (v2):

- `from_extension(ext)` — maps a file extension to a MIME type.
- `from_path(path)` — maps a file path to a MIME type via its extension.

Both commands take string input, return `Option<String>` (null on unknown), perform no I/O, and have no side effects. The default permission set enables both by default.

Browsers have no equivalent for path→MIME detection; the JS `mime-types` npm package adds ~10 KB to the bundle. This plugin is 0 KB of app JS.

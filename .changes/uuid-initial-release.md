---
'tauri-plugin-uuid': minor:feat
'tauri-plugin-uuid-js': minor:feat
---

Initial release of `tauri-plugin-uuid`.

Adds two commands backed by the pure-Rust `uuid` crate (v1.15):

- `generate_v4()` — random RFC-4122 v4 UUID. Equivalent to the browser's
  `crypto.randomUUID()`, but callable from any Tauri context (incl. mobile
  and headless CLI builds).
- `generate_v7()` — time-ordered v7 UUID (Unix-ms timestamp prefix + 74 bits
  of randomness). **No browser equivalent** — this is the gap the plugin
  fills.

Both commands take no input, perform no I/O, and have no side effects. The
default permission set enables both by default.

JavaScript bindings expose `v4()` and `v7()` async functions under
`@marrionesa/plugin-uuid`.

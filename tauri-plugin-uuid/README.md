# tauri-plugin-uuid

Generate **v4 and v7 UUIDs** natively from Rust, exposed to JavaScript.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Why this plugin exists

Browsers expose [`crypto.randomUUID()`](https://developer.mozilla.org/en-US/docs/Web/API/Crypto/randomUUID),
but it only produces **v4** UUIDs. **v7 UUIDs** (48-bit Unix-millisecond
timestamp prefix + 74 bits of randomness) are **lexicographically sortable by
creation time** — perfect for database primary keys and distributed-systems
event IDs — but are not available in any browser API.

This plugin fills that gap by routing through the pure-Rust
[`uuid`](https://crates.io/crates/uuid) crate, which has no platform bindings
and works on every Tauri target (desktop + mobile).

## Install

_This plugin requires a Rust version of at least **1.90**_

There are three general methods of installation:

1. Use crates.io and npm (easiest, and requires you to trust that our publishing pipeline worked)
2. Pull sources directly from GitHub using git tags / revision hashes (most secure)
3. Git submodule install this repo in your Tauri project and then use file protocol to ingest the source (most secure, but inconvenient to use)

Install the Rust crate by adding the following to your `Cargo.toml`:

`src-tauri/Cargo.toml`

```toml
[dependencies]
tauri-plugin-uuid = "0.1.0"
# alternatively with Git:
tauri-plugin-uuid = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-uuid
# or
npm add @marrionesa/plugin-uuid
# or
yarn add @marrionesa/plugin-uuid
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_uuid::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { v4, v7 } from '@marrionesa/plugin-uuid'

// Random v4 (equivalent to crypto.randomUUID())
const userId = await v4()
// "f0e1d2c3-b4a5-4987-9abc-def012345678"

// Time-ordered v7 (no browser equivalent!)
const eventId = await v7()
// "0192f5c7-2b8d-7abc-9def-0123456789ab"
//   ^^^^^^^^^^ first 13 hex chars = Unix-ms timestamp, sortable
```

## API

| Function | Returns           | Description                                                                                |
| -------- | ----------------- | ------------------------------------------------------------------------------------------ |
| `v4()`   | `Promise<string>` | Random v4 UUID (RFC 4122). 122 bits of randomness.                                         |
| `v7()`   | `Promise<string>` | v7 UUID with Unix-ms timestamp prefix + 74 bits of randomness. Lexicographically sortable. |

Both functions take no arguments and never throw.

## Permissions

The plugin ships a default permission set enabling both `generate_v4` and
`generate_v7` commands. Both are safe to enable by default: they take no user
input, perform no I/O, and have no side effects. To restrict access, declare a
capability in `src-tauri/capabilities/<name>.json`:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["uuid:allow-generate-v7"]
}
```

See [`permissions/`](./permissions/) for the full list.

## Why not just use the `uuid` npm package?

You can — it's a fine package. But the JS `uuid` package:

- Adds ~6 KB to your bundle (minified+gzipped).
- Re-implements v7 in JS, which is slower than the Rust `uuid` crate.
- Cannot be shared with Rust code (e.g. Tauri commands that need to generate
  UUIDs server-side without crossing the IPC boundary).

This plugin is **0 KB of JS app code** (only the tiny `invoke` wrapper) and
lets both Rust and JS share the same generation pipeline.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.

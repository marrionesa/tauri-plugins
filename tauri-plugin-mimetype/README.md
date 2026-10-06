# tauri-plugin-mimetype

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-mimetype.svg)](https://crates.io/crates/tauri-plugin-mimetype)
[![docs.rs](https://img.shields.io/docsrs/tauri-plugin-mimetype.svg)](https://docs.rs/tauri-plugin-mimetype)
[![npm](https://img.shields.io/npm/v/@marrionesa%2Fplugin-mimetype.svg)](https://www.npmjs.com/package/@marrionesa/plugin-mimetype)
[![downloads](https://img.shields.io/crates/d/tauri-plugin-mimetype.svg)](https://crates.io/crates/tauri-plugin-mimetype)
[![license](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)

Map a file extension or file path to a MIME type, using the pure-Rust [`mime_guess`](https://crates.io/crates/mime_guess) crate.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Why this plugin exists

Browsers expose **no API** for mapping a file path or extension to a MIME
type — there is no `navigator.mimeFromPath()` or equivalent. The closest
JavaScript option is the [`mime-types`](https://www.npmjs.com/package/mime-types)
npm package, which re-implements the lookup table in JS and adds **~10 KB**
(minified+gzipped) to the bundle.

This plugin fills that gap by routing through the pure-Rust `mime_guess`
crate, which has no platform bindings, performs no I/O, and runs entirely
in-memory. The result is **0 KB of app JS** beyond the tiny `invoke` wrapper,
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
tauri-plugin-mimetype = "0.1.0"
# alternatively with Git:
tauri-plugin-mimetype = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-mimetype
# or
npm add @marrionesa/plugin-mimetype
# or
yarn add @marrionesa/plugin-mimetype
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_mimetype::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { fromExtension, fromPath } from '@marrionesa/plugin-mimetype'

await fromExtension('json') // "application/json"
await fromExtension('.html') // "text/html"  (leading dot is tolerated)
await fromPath('/path/to/file.html') // "text/html"
await fromPath('/x/y.unknown') // null (extension not in the table)
```

## API

| Function             | Returns                   | Description                                                                                                                                                                |
| -------------------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `fromExtension(ext)` | `Promise<string \| null>` | Map a bare file extension (e.g. `"json"`, `"html"`, `"tar.gz"`) to a MIME type. A leading dot is tolerated. Returns `null` if unknown.                                     |
| `fromPath(path)`     | `Promise<string \| null>` | Map a file path (e.g. `"/x/y.html"`) to a MIME type via its extension. The path is **not** opened or statted — only its extension is inspected. Returns `null` if unknown. |

Both functions are pure: same input always yields the same output, no I/O, no
side effects, no thrown errors (an unknown extension resolves to `null`,
not an exception).

## Permissions

The plugin ships a default permission set enabling both `from_extension` and
`from_path` commands. Both are safe to enable by default: they take a single
string argument, perform no I/O (`from_path` only inspects the string — it
does not open the file), and have no side effects.

To restrict access — for example, to allow only `from_extension` and not
`from_path` — declare a capability in `src-tauri/capabilities/<name>.json`:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["mimetype:allow-from-extension"]
}
```

See [`permissions/`](./permissions/) for the full list of identifiers
(`allow-from-extension`, `deny-from-extension`, `allow-from-path`,
`deny-from-path`).

## Why not just use the `mime-types` npm package?

You can — it's a fine package. But the JS `mime-types` package:

- Adds **~10 KB** to your bundle (minified+gzipped) for a lookup table that
  ships with every Tauri runtime's Rust toolchain anyway.
- Cannot be shared with Rust code (e.g. a Tauri command that needs the same
  MIME type to set a `Content-Type` header before sending bytes over IPC).
- Has no notion of compound extensions like `tar.gz`; `mime_guess` does.

This plugin is **0 KB of JS app code** (only the tiny `invoke` wrapper) and
lets both Rust and JS share the same lookup table.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.

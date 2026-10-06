# Tauri v2 Plugins

[![Tauri v2](https://img.shields.io/badge/Tauri-v2-24C8DB.svg)](https://v2.tauri.app)
[![license](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)
[![Rust 1.90+](https://img.shields.io/badge/rust-1.90%2B-orange.svg)](https://www.rust-lang.org)
[![plugins](https://img.shields.io/badge/plugins-6-success.svg)](#plugins)
[![npm](https://img.shields.io/badge/npm-%40marrionesa-CB3837.svg)](https://www.npmjs.com/org/marrionesa)
[![crates.io](https://img.shields.io/badge/crates.io-6%20crates-FC8D62.svg)](https://crates.io/search?q=tauri-plugin-)

Six small, focused [Tauri v2](https://v2.tauri.app) plugins. Each one solves a
single problem that a webview cannot solve well on its own, ships its own
`#[tauri::command]` handlers **and** its JavaScript bindings, and is published
independently on **crates.io** and **npm**.

Every plugin follows the conventions the Tauri team uses in its own
[`tauri-apps/plugins-workspace`](https://github.com/tauri-apps/plugins-workspace)
monorepo, so any of them is structured the same way as the official plugins.

## Plugins

| Plugin                                | What it does                                               | Commands                                 | Platforms                         | Install                                                                 |
| ------------------------------------- | ---------------------------------------------------------- | ---------------------------------------- | --------------------------------- | ----------------------------------------------------------------------- |
| [`hostname`](./tauri-plugin-hostname) | System hostname and current username                       | `hostname`, `username`                   | all                               | `cargo add tauri-plugin-hostname` · `npm i @marrionesa/plugin-hostname` |
| [`uuid`](./tauri-plugin-uuid)         | v4 **and v7** (time-ordered) UUIDs                         | `generate_v4`, `generate_v7`             | all                               | `cargo add tauri-plugin-uuid` · `npm i @marrionesa/plugin-uuid`         |
| [`hash`](./tauri-plugin-hash)         | SHA-256 / SHA-512 / MD5 of strings and files, **streamed** | `hash_string`, `hash_file`               | all                               | `cargo add tauri-plugin-hash` · `npm i @marrionesa/plugin-hash`         |
| [`mimetype`](./tauri-plugin-mimetype) | Extension / path → MIME type                               | `from_extension`, `from_path`            | all                               | `cargo add tauri-plugin-mimetype` · `npm i @marrionesa/plugin-mimetype` |
| [`timezone`](./tauri-plugin-timezone) | IANA timezone + DST-aware UTC offset                       | `get_timezone`, `get_utc_offset_seconds` | all                               | `cargo add tauri-plugin-timezone` · `npm i @marrionesa/plugin-timezone` |
| [`beep`](./tauri-plugin-beep)         | The OS-native alert sound, with **no audio asset bundled** | `beep`                                   | Windows/macOS full, Linux partial | `cargo add tauri-plugin-beep` · `npm i @marrionesa/plugin-beep`         |

## Why these plugins exist

Each one fills a gap that has no good webview-only answer:

- **`uuid`** — `crypto.randomUUID()` only produces v4. **v7 is time-ordered**, so
  ids sort by creation time (ideal for database keys and event logs), and no
  browser API can generate one.
- **`hash`** — `SubtleCrypto.digest()` requires the whole input in memory as an
  `ArrayBuffer`, which fails for large files. This streams in 8 KB chunks with
  constant memory, so multi-GB files hash without blowing up the renderer.
- **`hostname`** — the official `tauri-plugin-os` has a Rust-only `hostname()` and
  no username at all. This exposes both to JavaScript.
- **`timezone`** — `Date.prototype.getTimezoneOffset()` returns **minutes, signed
  inverted** (positive west). This returns the offset in **seconds, east
  positive**, plus the IANA name, so the sign convention stops being a bug farm.
- **`mimetype`** — no browser equivalent for path → MIME. Zero KB of app
  JavaScript, versus pulling in an npm lookup table.
- **`beep`** — plays the OS alert sound without bundling an audio file or a media
  decoder, and respects the user's system sound theme.

## Quick start

Pick a plugin, add both halves, then register it in Rust:

```rust
// src-tauri/Cargo.toml:  tauri-plugin-uuid = "0.1.0"
tauri::Builder::default()
    .plugin(tauri_plugin_uuid::init())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
```

```ts
// npm i @marrionesa/plugin-uuid
import { v7 } from '@marrionesa/plugin-uuid'

const id = await v7() // time-ordered, sortable
```

Each plugin needs its permission enabled in your capability file — see the
README of the plugin you are using for the exact identifier.

## Repository layout

```
tauri-plugins/
├── _TEMPLATE/                 # copy-and-rename scaffold for a new plugin
├── tauri-plugin-beep/         # one directory per plugin, each self-contained
├── tauri-plugin-hash/         #   crate + npm package + permissions + CI
├── tauri-plugin-hostname/
├── tauri-plugin-mimetype/
├── tauri-plugin-timezone/
├── tauri-plugin-uuid/
├── Cargo.toml                 # Rust workspace (one lock file, shared tooling)
├── package.json               # pnpm workspace root (prettier)
├── .github/workflows/         # CI: fmt, lint-rust, test-rust, mirror
├── .changes/                  # covector config + change files
└── CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md
```

The plugin directories are the unit of publishing: each contains a complete,
self-contained crate **and** npm package. The root `Cargo.toml` exists only so
that `cargo fmt --all`, `cargo clippy --workspace` and a single `Cargo.lock`
work in CI; it is never published.

## Development

Everything CI enforces can be run locally, from the repository root:

```bash
cargo fmt --all -- --check                                      # rustfmt
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace                                          # unit tests
npx prettier --check .                                          # JS/TS/MD/YAML
npx @taplo/cli@0.5.2 fmt --check                                # TOML
```

Per plugin, the packaging checks are:

```bash
cargo publish --dry-run   # inside the plugin dir
npm pack --dry-run        # inside the plugin dir
```

The workflows run **on demand only** (`workflow_dispatch`): open the **Actions**
tab and use _Run workflow_.

## Scaffolding a new plugin

```bash
# 1. Copy the template
cp -r _TEMPLATE tauri-plugin-<name>

# 2. Rename the placeholders
#    - crate name:            tauri-plugin-template  -> tauri-plugin-<name>
#    - npm package:           @marrionesa/plugin-template -> @marrionesa/plugin-<name>
#    - plugin name (Builder): "template"              -> "<name>"
#    - android package:       com.plugin.template     -> com.plugin.<name_snake>
#    - Swift target:          tauri-plugin-template   -> tauri-plugin-<name>
#    - COMMANDS in build.rs:  ["ping"]                -> your actual commands
#    - add the crate to `members` in the root Cargo.toml

# 3. Build both halves
cd tauri-plugin-<name>
cargo build --release
npm install && npm run build
```

## Contributing

Bug reports and pull requests are welcome — please read
[`CONTRIBUTING.md`](./CONTRIBUTING.md) first. For security issues, follow
[`SECURITY.md`](./SECURITY.md) instead of opening a public issue.

## License

Dual-licensed under **Apache-2.0 OR MIT**, matching the Tauri convention; use
whichever you prefer. See [`LICENSE_APACHE-2.0`](./LICENSE_APACHE-2.0) and
[`LICENSE_MIT`](./LICENSE_MIT).

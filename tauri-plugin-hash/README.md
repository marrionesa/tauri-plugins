# tauri-plugin-hash

Stream-compute **SHA-256, SHA-512, and MD5** hashes of strings and files from Rust, exposed to JavaScript.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Why this plugin exists

Browsers expose [`SubtleCrypto.digest()`](https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest),
which can compute SHA-256 / SHA-384 / SHA-512 — but it requires loading the
**entire input into memory as an `ArrayBuffer`** before hashing begins. For
files larger than the available RAM (or the browser's per-tab memory limit,
typically 2 GB on 64-bit Chromium) this throws `QuotaExceededError` or simply
crashes the renderer.

This plugin fills that gap by streaming the file in **8 KB chunks** through a
Rust [`tokio::fs::File`](https://docs.rs/tokio/latest/tokio/fs/struct.File.html)
reader, feeding each chunk into an incremental hasher. Memory usage is
**constant** (~8 KB + the hash state) regardless of file size — multi-GB ISOs,
database dumps, and video archives hash without breaking a sweat.

The hashing crates ([`sha2`](https://crates.io/crates/sha2),
[`md-5`](https://crates.io/crates/md-5)) are pure Rust with no platform bindings,
so the plugin works on every Tauri target (desktop + mobile).

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
tauri-plugin-hash = "0.1.0"
# alternatively with Git:
tauri-plugin-hash = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-hash
# or
npm add @marrionesa/plugin-hash
# or
yarn add @marrionesa/plugin-hash
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_hash::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { hashString, hashFile } from '@marrionesa/plugin-hash'

// Hash a UTF-8 string
const docHash = await hashString('hello world', 'sha256')
// "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"

// Stream a 4 GB Ubuntu ISO without loading it into memory
const isoHash = await hashFile('/path/to/ubuntu-24.04.iso', 'sha256')
```

## API

| Function                       | Returns           | Description                                                                  |
| ------------------------------ | ----------------- | ---------------------------------------------------------------------------- |
| `hashString(input, algorithm)` | `Promise<string>` | Hex digest of the UTF-8-encoded input.                                       |
| `hashFile(path, algorithm)`    | `Promise<string>` | Hex digest of the file at `path`, streamed in 8 KB chunks (constant memory). |

**Type**

```typescript
export type HashAlgorithm = 'sha256' | 'sha512' | 'md5'
```

`algorithm` is matched **case-insensitively** on the Rust side.

| Algorithm | Output length (hex chars) | Notes                                                                         |
| --------- | ------------------------- | ----------------------------------------------------------------------------- |
| `sha256`  | 64                        | Recommended for new code.                                                     |
| `sha512`  | 128                       | Use when collision / length-extension resistance matters more than bandwidth. |
| `md5`     | 32                        | Legacy / non-security checksums only. Never use for passwords or signatures.  |

## Permissions

The plugin ships a default permission set enabling both `hash_string` and
`hash_file` commands. `hash_string` is safe by default — it takes a UTF-8
string and performs no I/O. `hash_file` reads **arbitrary paths the caller
passes** — to restrict which files JavaScript is allowed to hash, deny
`allow-hash-file` in a capability:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["hash:allow-hash-string", "hash:deny-hash-file"]
}
```

See [`permissions/`](./permissions/) for the full list.

## Why not just use `SubtleCrypto` in the webview?

You can — `crypto.subtle.digest('SHA-256', ...)` works fine for **small strings
and small files**. But it:

- Requires loading the **entire** input into memory as an `ArrayBuffer` before
  hashing begins — fails for files > available RAM.
- Cannot hash files larger than the browser's per-tab memory limit (typically
  2 GB on 64-bit Chromium) without chunking, and `SubtleCrypto` exposes no
  streaming API for SHA-256 / SHA-512.
- Has no MD5 (browser engines deliberately omit it).

This plugin streams in 8 KB chunks, so memory is constant regardless of file
size, and works for any file the OS can `open(2)`.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.

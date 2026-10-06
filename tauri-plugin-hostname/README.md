# tauri-plugin-hostname

Read the **system hostname** and the **current username** from Rust, exposed to
JavaScript.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Why this plugin exists

The official [`tauri-plugin-os`](https://v2.tauri.app/plugin/os/) crate ships a
Rust-only `os::hostname()` function — but it is **NOT exposed to JavaScript by
default**. The os plugin's default permission set does **not** include an
`allow-hostname` permission (you can verify this by inspecting
`plugins/os/permissions/` in the upstream `tauri-apps/plugins-workspace` repo),
so the function is callable only from Rust code that links against the os
plugin directly.

This plugin fills that JS-visible gap by re-exporting hostname via its own
`plugin:hostname|hostname` IPC channel — and additionally exposes
`username()`, which the os plugin does not provide at all.

If the os plugin later adds `allow-hostname` (and a `username()` command), this
plugin can be deprecated in favour of the upstream APIs without breaking any
consumer code — the JavaScript function signatures are identical.

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
tauri-plugin-hostname = "0.1.0"
# alternatively with Git:
tauri-plugin-hostname = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-hostname
# or
npm add @marrionesa/plugin-hostname
# or
yarn add @marrionesa/plugin-hostname
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_hostname::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { hostname, username } from '@marrionesa/plugin-hostname'

const host = await hostname() // "my-laptop"
const user = await username() // "alice"
```

## API

| Function     | Returns           | Description                                                                                                                                                      |
| ------------ | ----------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `hostname()` | `Promise<string>` | The system's hostname as a UTF-8 string. Falls back to a lossy conversion on the very rare misconfigured system where libc::gethostname returns non-UTF-8 bytes. |
| `username()` | `Promise<string>` | The current user's login name as reported by the OS (`getpwuid_r` on Unix, `USERNAME` env var with `GetUserNameW` fallback on Windows).                          |

Both functions take no arguments and never throw.

## Permissions

The plugin ships a default permission set enabling both `hostname` and
`username` commands. Both are safe to enable by default: they take no user
input, perform no I/O, and have no side effects. To restrict access, declare a
capability in `src-tauri/capabilities/<name>.json`:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["hostname:allow-hostname"]
}
```

Or to deny access from a particular window/iframe:

```json
{
  "identifier": "untrusted-frame",
  "windows": ["preview"],
  "permissions": ["hostname:deny-hostname", "hostname:deny-username"]
}
```

See [`permissions/`](./permissions/) for the full list.

### Sensitivity note

The hostname is a **low-sensitivity machine identifier** — it does not on its
own reveal the user's identity, but it does uniquely identify the machine
across sessions and can be used for fingerprinting when combined with other
signals. The username is similarly a low-sensitivity user identifier. Do NOT
log either to a remote telemetry endpoint without explicit user consent.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.

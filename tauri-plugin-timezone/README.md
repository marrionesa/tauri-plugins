# tauri-plugin-timezone

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-timezone.svg)](https://crates.io/crates/tauri-plugin-timezone)
[![docs.rs](https://img.shields.io/docsrs/tauri-plugin-timezone.svg)](https://docs.rs/tauri-plugin-timezone)
[![npm](https://img.shields.io/npm/v/@marrionesa%2Fplugin-timezone.svg)](https://www.npmjs.com/package/@marrionesa/plugin-timezone)
[![downloads](https://img.shields.io/crates/d/tauri-plugin-timezone.svg)](https://crates.io/crates/tauri-plugin-timezone)
[![license](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)

Read the **system's IANA timezone and the current UTC offset in seconds**, from Rust.

| Platform | Supported |
| -------- | --------- |
| Linux    | ✓         |
| Windows  | ✓         |
| macOS    | ✓         |
| Android  | ✓         |
| iOS      | ✓         |

## Why this plugin exists

The official [`tauri-plugin-os`](https://v2.tauri.app/plugin/os/) exposes
`locale()` but **not** timezone. Browsers expose
[`Intl.DateTimeFormat().resolvedOptions().timeZone`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Intl/DateTimeFormat/resolvedOptions#timezone)
(IANA name only), but the DST-aware UTC offset must be obtained via
[`Date.prototype.getTimezoneOffset()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/getTimezoneOffset),
which returns **minutes** and is **signed inverted** (positive WEST of UTC,
negative east). That convention has bitten countless developers.

This plugin returns both values in the correct units:

- **IANA name** (e.g. `"Europe/Madrid"`) via the pure-Rust
  [`iana-time-zone`](https://crates.io/crates/iana-time-zone) crate, which
  dispatches to `/etc/timezone` on Linux, the registry on Windows,
  `CFTimeZone` on macOS/iOS, and `java.util.TimeZone` on Android.
- **UTC offset in seconds, east positive** via
  [`chrono`](https://crates.io/crates/chrono)'s `Local::now().offset().local_minus_utc()`,
  which is DST-aware (returns `7200` for `Europe/Madrid` in summer, `3600` in winter).

Both commands are read-only, take no input, perform no I/O, and have no side
effects — safe to enable by default.

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
tauri-plugin-timezone = "0.1.0"
# alternatively with Git:
tauri-plugin-timezone = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-timezone
# or
npm add @marrionesa/plugin-timezone
# or
yarn add @marrionesa/plugin-timezone
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_timezone::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { getTimezone, getUtcOffsetSeconds } from '@marrionesa/plugin-timezone'

const tz = await getTimezone() // "Europe/Madrid"
const offset = await getUtcOffsetSeconds() // 7200 (UTC+2 in summer, DST-aware)
//                                                3600 (UTC+1 in winter)
```

## API

| Function                | Returns                   | Description                                                                                                                     |
| ----------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------------- |
| `getTimezone()`         | `Promise<string \| null>` | IANA timezone identifier (e.g. `"Europe/Madrid"`), or `null` if the OS cannot determine it.                                     |
| `getUtcOffsetSeconds()` | `Promise<number>`         | Current DST-aware UTC offset in seconds. **East of UTC is positive** — opposite sign from `Date.prototype.getTimezoneOffset()`. |

Both functions take no arguments and never throw. `getTimezone()` returns
`null` (not an error) on unsupported platforms where `iana-time-zone` has no
backend.

## Permissions

The plugin ships a default permission set enabling both `get_timezone` and
`get_utc_offset_seconds` commands. Both are safe to enable by default: they
take no user input, perform no I/O, and have no side effects. To restrict
access, declare a capability in `src-tauri/capabilities/<name>.json`:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["timezone:allow-get-timezone"]
}
```

See [`permissions/`](./permissions/) for the full list.

## Why not just use `Intl` / `getTimezoneOffset()`?

You can — they work. But the browser APIs have two sharp edges:

- `Intl.DateTimeFormat().resolvedOptions().timeZone` gives the IANA name but
  **no offset**. You need a second call to get the offset.
- `Date.prototype.getTimezoneOffset()` returns **minutes west of UTC**
  (positive west, negative east) — the opposite of every other convention in
  computing (POSIX `tm_gmtoff`, ISO 8601, RFC 3339 all use east-positive).
  Every developer who has used it has at some point written the sign backwards.

This plugin returns both pieces in the units you actually want:

- Offset in **seconds** (not minutes) — matches POSIX `tm_gmtoff` and what
  `chrono` natively produces.
- **East-positive** sign — matches ISO 8601 / RFC 3339.
- DST-aware — uses the system clock's _current_ offset, not a fixed one
  derived from the IANA name.

And it's **0 KB of JS app code** beyond the tiny `invoke` wrapper.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.

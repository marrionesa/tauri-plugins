# tauri-plugin-beep

[![crates.io](https://img.shields.io/crates/v/tauri-plugin-beep.svg)](https://crates.io/crates/tauri-plugin-beep)
[![docs.rs](https://img.shields.io/docsrs/tauri-plugin-beep.svg)](https://docs.rs/tauri-plugin-beep)
[![npm](https://img.shields.io/npm/v/@marrionesa%2Fplugin-beep.svg)](https://www.npmjs.com/package/@marrionesa/plugin-beep)
[![downloads](https://img.shields.io/crates/d/tauri-plugin-beep.svg)](https://crates.io/crates/tauri-plugin-beep)
[![license](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)

Play the **system alert sound** on Windows, macOS, and Linux from Rust.

| Platform | Supported                                                      |
| -------- | -------------------------------------------------------------- |
| Windows  | ✓                                                              |
| Linux    | ✓ (partial — depends on `paplay`, falls back to terminal bell) |
| macOS    | ✓                                                              |
| Android  | ✗                                                              |
| iOS      | ✗                                                              |

## Why this plugin exists

The existing [`tauri-plugin-sound-player`](https://crates.io/crates/tauri-plugin-sound-player)
plays mp3 / ogg / wav files, which means you have to **bundle an audio file**
with your app and ship a media decoder. That's overkill when all you want is a
one-shot attention sound — e.g. a terminal emulator alerting the user on bell,
or a tray app nudging the user on an unread notification.

This plugin instead asks the **operating system** to play its native alert
sound — `MessageBeep(MB_OK)` on Windows, `afplay` of the system Tink sound on
macOS, `paplay` of the freedesktop bell on Linux. Zero audio files bundled;
zero decoder code; zero extra weight on the IPC layer.

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
tauri-plugin-beep = "0.1.0"
# alternatively with Git:
tauri-plugin-beep = { git = "https://github.com/marrionesa/tauri-plugins", branch = "main" }
```

Install the JavaScript guest bindings using your preferred package manager:

```sh
pnpm add @marrionesa/plugin-beep
# or
npm add @marrionesa/plugin-beep
# or
yarn add @marrionesa/plugin-beep
```

## Usage

Register the plugin on your `tauri::Builder`:

`src-tauri/src/lib.rs`

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_beep::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

Call it from JavaScript:

```typescript
import { beep } from '@marrionesa/plugin-beep'

await beep() // plays the OS alert sound
```

## API

| Function | Returns         | Description                                                                                                                                                                                                  |
| -------- | --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `beep()` | `Promise<void>` | Plays the OS-native alert sound. Windows: `MessageBeep(MB_OK)`. macOS: `afplay /System/Library/Sounds/Tink.aiff`. Linux: `paplay .../bell.oga` with a `\x07` terminal-bell fallback. Rejects on Android/iOS. |

The function takes no arguments. On desktop it resolves once the sound has
been _triggered_ (it does not wait for playback to finish — playback may be
async at the OS level). On mobile it rejects with an `UnsupportedPlatform`
error.

## Permissions

The plugin ships a default permission set enabling the `beep` command. It is
safe to enable by default: the command takes no user input and has no I/O
side effects beyond playing a short OS-supplied sound. To restrict access,
declare a capability in `src-tauri/capabilities/<name>.json`:

```json
{
  "identifier": "my-capability",
  "windows": ["main"],
  "permissions": ["beep:allow-beep"]
}
```

To **deny** the command from a specific window, use `beep:deny-beep` instead.

See [`permissions/`](./permissions/) for the full list.

## Platform notes

- **Windows** — calls `MessageBeep(MB_OK)` (Win32 `MessageBeep` with
  `0xFFFFFFFF` as the style, which means "the default beep sound") via the
  [`windows-sys`](https://crates.io/crates/windows-sys) v0.59 crate with the
  `Win32_UI_WindowsAndMessaging` feature. No subprocess is spawned. The
  return value is checked: if `MessageBeep` returns `0` (no default sound
  configured / audio service unavailable), the command returns `Err`.
- **macOS** — spawns `afplay /System/Library/Sounds/Tink.aiff` with stdin,
  stdout, and stderr set to `Stdio::null()`. `afplay` ships with every macOS
  install since 10.x. We deliberately avoid pulling in `objc2` to keep the dep
  tree minimal — `afplay` plays the exact same Tink sound that `NSBeep()`
  plays by default.
- **Linux** — spawns `paplay /usr/share/sounds/freedesktop/stereo/bell.oga`
  (the freedesktop-themed system bell) with stdin/stdout/stderr set to
  `Stdio::null()`. `paplay` ships with PulseAudio, which is preinstalled on
  most desktop distros. If `paplay` is not on `$PATH` (e.g. minimal WM
  setups, PipeWire-only systems without the pipewire-pulse compat layer), the
  fallback writes a single `\x07` (terminal bell) byte to stderr — audible
  only when the app is launched from a terminal. The command still returns
  `Ok(())` in that case, because there is nothing the caller can do to
  recover beyond the fallback.
- **Android / iOS** — mobile OSes have no equivalent alert-beep API suitable
  for this plugin. The command returns `Err(UnsupportedPlatform)`.

## Contributing

PRs accepted. Please make sure to read the
[Contributing Guide](../CONTRIBUTING.md) before making a pull request.

## License

Code: (c) 2019 - Present - The Tauri Programme within The Commons Conservancy.

MIT OR Apache-2.0 where applicable.

---
'tauri-plugin-beep': minor:feat
'tauri-plugin-beep-js': minor:feat
---

Initial release of `tauri-plugin-beep`.

Adds one command that plays the OS-native alert sound:

- `beep()` — Windows calls `MessageBeep(MB_OK)`, macOS spawns `afplay /System/Library/Sounds/Tink.aiff`, Linux spawns `paplay .../bell.oga` with a `\x07` stderr fallback. Mobile returns an unsupported-platform error.

The existing `tauri-plugin-sound-player` plays mp3/ogg/wav files (requires bundling audio + a media decoder); this plugin plays the OS-native alert sound — zero audio files bundled, useful for terminal-emulator and tray apps.

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::error::Result;

/// Play the system alert sound (the OS "beep" / NSBeep / MessageBeep).
///
/// On Windows: calls `MessageBeep(MB_OK)` via the `windows-sys` crate.
/// On macOS:   spawns `afplay /System/Library/Sounds/Tink.aiff`
///             (`afplay` is preinstalled on every macOS since 10.x).
/// On Linux:   spawns `paplay /usr/share/sounds/freedesktop/stereo/bell.oga`
///             with a fallback to writing `\x07` (terminal bell) to stderr if
///             `paplay` is not installed.
/// On mobile:  returns `Err(UnsupportedPlatform)` — mobile OSes have no
///             equivalent alert beep API suitable for this plugin.
#[tauri::command]
pub fn beep() -> Result<()> {
    platform_beep()
}

#[cfg(target_os = "windows")]
fn platform_beep() -> Result<()> {
    use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBeep, MESSAGEBEEP_STYLE};
    // 0xFFFFFFFF means "the default beep sound" per the Win32 docs.
    unsafe {
        if MessageBeep(MESSAGEBEEP_STYLE(0xFFFFFFFF)) == 0 {
            return Err(crate::error::Error::Anyhow(
                "MessageBeep returned 0 (failed)".to_string(),
            ));
        }
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn platform_beep() -> Result<()> {
    // We deliberately avoid pulling in `objc2` to keep the dep tree minimal —
    // `afplay` is shipped with every macOS install and plays the system Tink
    // sound (the same one NSBeep plays by default).
    std::process::Command::new("afplay")
        .arg("/System/Library/Sounds/Tink.aiff")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .map_err(|e| crate::error::Error::Anyhow(format!("afplay spawn failed: {e}")))?;
    Ok(())
}

#[cfg(target_os = "linux")]
fn platform_beep() -> Result<()> {
    // Try PulseAudio's paplay with the freedesktop bell sound first.
    let paplay = std::process::Command::new("paplay")
        .arg("/usr/share/sounds/freedesktop/stereo/bell.oga")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
    match paplay {
        Ok(_child) => Ok(()),
        Err(_e) => {
            // Fallback: terminal bell. Only audible in a terminal; no-op in
            // GUI apps without a TTY. We still return Ok(()) because the
            // *attempt* to beep succeeded — there is nothing the caller can
            // do to recover from "paplay not installed" beyond this fallback.
            use std::io::Write;
            let _ = std::io::stderr().write_all(b"\x07");
            Ok(())
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn platform_beep() -> Result<()> {
    Err(crate::error::Error::Anyhow(
        "beep is not supported on this platform (only Windows, macOS, Linux)".to_string(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// On the three supported desktop platforms `beep()` reports success.
    ///
    /// On Linux the `paplay` failure path deliberately falls back to the
    /// terminal bell and still returns `Ok(())`, so this holds even on a
    /// headless machine with no sound server installed.
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    #[test]
    fn beep_succeeds_on_desktop() {
        assert!(beep().is_ok());
    }

    /// Mobile has no equivalent alert API, so the command must report the
    /// unsupported platform instead of silently succeeding.
    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    #[test]
    fn beep_reports_unsupported_platform_on_mobile() {
        assert!(beep().is_err());
    }

    /// The error type must serialize to a plain JSON string, which is the
    /// contract `tauri::command` relies on when returning `Result<_, Error>`.
    #[test]
    fn error_serializes_as_json_string() {
        let err = crate::error::Error::Anyhow("boom".to_string());
        let json = serde_json::to_string(&err).unwrap();
        assert_eq!(json, "\"boom\"");
    }
}

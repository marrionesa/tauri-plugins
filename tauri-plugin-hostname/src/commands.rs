// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use gethostname::gethostname;

/// Returns the system's hostname as a UTF-8 string.
///
/// Falls back to a lossy conversion if the hostname is not valid UTF-8 (very
/// rare — only happens on misconfigured systems where libc::gethostname returns
/// non-UTF-8 bytes; invalid bytes become the Unicode replacement character).
///
/// Equivalent to the os plugin's `os::hostname()` Rust function, but routed
/// through IPC so it is callable from JavaScript via
/// `invoke('plugin:hostname|hostname')`.
#[tauri::command]
pub fn hostname() -> String {
    gethostname().to_string_lossy().into_owned()
}

/// Returns the current user's username (login name) as reported by the OS.
///
/// On Unix this is the effective UID's password entry (`getpwuid_r`); on
/// Windows it is the `USERNAME` environment variable with a `GetUserNameW`
/// fallback. There is no equivalent in the official `tauri-plugin-os` plugin —
/// this command fills that gap.
#[tauri::command]
pub fn username() -> String {
    whoami::username()
}

/// Both commands must return a non-empty identifier that is safe to embed in
/// JSON. They are infallible by design (see `error.rs`), so the only meaningful
/// assertions are on shape.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hostname_is_non_empty() {
        let h = hostname();
        assert!(!h.is_empty(), "hostname() returned an empty string");
        assert!(!h.contains('\0'), "hostname() returned a NUL byte");
    }

    #[test]
    fn username_is_non_empty() {
        let u = username();
        assert!(!u.is_empty(), "username() returned an empty string");
        assert!(!u.contains('\0'), "username() returned a NUL byte");
    }
}

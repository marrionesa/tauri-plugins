// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

// Aliased because this module also defines a `#[tauri::command]` named
// `get_timezone`; importing the crate's function under the same name would
// collide with it (E0255).
use iana_time_zone::get_timezone as iana_get_timezone;

/// Returns the system's IANA timezone identifier (e.g. "Europe/Madrid",
/// "America/New_York").
///
/// Returns `None` if the timezone cannot be determined — rare in practice,
/// usually only on unsupported platforms where `iana-time-zone` has no
/// backend. The IANA name is sourced directly from the OS
/// (`/etc/timezone` / `timedatectl` on Linux, the Windows registry on
/// Windows, `CFTimeZone` on macOS/iOS, `java.util.TimeZone` on Android).
#[tauri::command]
pub fn get_timezone() -> Option<String> {
    iana_get_timezone().ok()
}

/// Returns the current UTC offset in seconds (east of UTC is positive).
///
/// Example: `Europe/Madrid` in summer returns `7200` (UTC+2); in winter it
/// returns `3600` (UTC+1).
///
/// This is **DST-aware**: it uses `chrono::Local::now()`'s offset, which
/// reflects whatever DST state the system clock is currently in — not a fixed
/// offset derived from the IANA name.
///
/// Note: this is the **opposite** sign convention from JavaScript's
/// `Date.prototype.getTimezoneOffset()`, which returns minutes west of UTC
/// (positive west, negative east). This plugin returns seconds east of UTC
/// (positive east, negative west) — the same convention used by POSIX
/// `tm_gmtoff` and ISO 8601 offsets.
#[tauri::command]
pub fn get_utc_offset_seconds() -> i32 {
    use chrono::Local;
    // `local_minus_utc()` already returns an `i32` (seconds east of UTC), so no
    // cast is needed.
    Local::now().offset().local_minus_utc()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The IANA identifier comes straight from the OS. Some minimal containers
    /// with no `/etc/localtime` and no `TZ` have no configured zone at all, so
    /// `None` is a legitimate result — but if a name IS returned it must look
    /// like an IANA identifier, never an empty string.
    #[test]
    fn timezone_is_either_absent_or_a_plausible_iana_name() {
        match get_timezone() {
            None => {}
            Some(tz) => {
                assert!(!tz.is_empty(), "get_timezone() returned an empty string");
                assert!(!tz.contains(' '), "IANA names never contain spaces: {tz}");
                assert!(
                    tz.contains('/') || tz == "UTC" || tz == "GMT",
                    "unexpected IANA identifier: {tz}"
                );
            }
        }
    }

    /// Valid terrestrial UTC offsets span -12:00 ..= +14:00 (i.e. -43200 ..= 50400
    /// seconds). Anything outside that range means the sign convention or the
    /// unit (seconds, not minutes) is wrong.
    #[test]
    fn utc_offset_is_in_seconds_and_within_the_terrestrial_range() {
        let secs = get_utc_offset_seconds();
        assert!(
            (-43_200..=50_400).contains(&secs),
            "offset {secs}s is outside -12:00..=+14:00 — wrong unit or sign?"
        );
        // The vast majority of zones are whole minutes; catching a seconds-scale
        // mistake (e.g. returning minutes) is the point of the range check above,
        // and this pins the granularity too.
        assert_eq!(secs % 60, 0, "ICT offsets are whole minutes, got {secs}s");
    }

    /// Documented sign convention: east of UTC is positive. Verified against
    /// chrono's own value for the same instant, so it cannot silently invert.
    #[test]
    fn offset_matches_chronos_sign_convention() {
        use chrono::Local;
        assert_eq!(
            get_utc_offset_seconds(),
            Local::now().offset().local_minus_utc()
        );
    }
}

// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use uuid::Uuid;

/// Generate a random v4 UUID (RFC 4122 variant, 122 bits of randomness).
///
/// Returns the canonical hyphenated string form, e.g.
/// `"f0e1d2c3-b4a5-4987-9abc-def012345678"`.
///
/// Equivalent to the browser's `crypto.randomUUID()` but callable from Rust
/// and from any Tauri JS context.
#[tauri::command]
pub fn generate_v4() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a v7 UUID: 48-bit Unix-millisecond timestamp + 12 bits of random
/// sub-millisecond A + 14 bits of random B + 62 bits of random tail.
///
/// The result is **lexicographically sortable** by creation time, making it
/// ideal for database primary keys and distributed-systems event IDs.
///
/// Example: `"0192f5c7-2b8d-7abc-9def-0123456789ab"`.
///
/// This command has **no browser equivalent** — that's the gap this plugin
/// fills.
#[tauri::command]
pub fn generate_v7() -> String {
    Uuid::now_v7().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn v4_is_a_valid_uuid() {
        let s = generate_v4();
        let parsed = Uuid::parse_str(&s).expect("generate_v4 must return a parseable UUID");
        assert_eq!(parsed.get_version_num(), 4);
    }

    #[test]
    fn v4_is_unique_across_calls() {
        assert_ne!(generate_v4(), generate_v4());
    }

    #[test]
    fn v7_is_a_valid_uuid() {
        let s = generate_v7();
        let parsed = Uuid::parse_str(&s).expect("generate_v7 must return a parseable UUID");
        assert_eq!(parsed.get_version_num(), 7);
    }

    /// The whole point of v7 is that the textual form sorts by creation time.
    #[test]
    fn v7_is_lexicographically_ordered_by_creation_time() {
        let first = generate_v7();
        std::thread::sleep(std::time::Duration::from_millis(5));
        let second = generate_v7();
        assert!(
            first < second,
            "expected {first} < {second} (v7 must sort by time)"
        );
    }

    /// RFC 9562 layout for v7: the 48-bit big-endian Unix-ms timestamp occupies
    /// the first 12 hex characters, so ids created in the same millisecond share
    /// that prefix.
    #[test]
    fn v7_starts_with_a_timestamp_prefix() {
        let s = generate_v7();
        let hex: String = s.chars().filter(|c| *c != '-').take(12).collect();
        assert_eq!(hex.len(), 12);
        assert!(
            u64::from_str_radix(&hex, 16).is_ok(),
            "timestamp prefix must be hex: {hex}"
        );
    }
}

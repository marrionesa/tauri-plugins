// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

/// Map a file extension (without leading dot, e.g. "json", "html", "tar.gz")
/// to a MIME type.
///
/// Returns `None` (serialized as JSON `null`) if the extension is unknown to
/// the `mime_guess` lookup table.
///
/// A leading dot is tolerated and stripped, so `"json"` and `".json"` produce
/// the same result.
#[tauri::command]
pub fn from_extension(extension: String) -> Option<String> {
    let ext = extension.trim_start_matches('.');
    mime_guess::from_ext(ext).first_raw().map(String::from)
}

/// Map a file path (e.g. "/path/to/file.json") to a MIME type using its
/// extension.
///
/// Returns `None` (serialized as JSON `null`) if the path's extension is
/// unknown to the `mime_guess` lookup table.
///
/// This command **inspects only the string** — it does not open, stat, or
/// read the file. The path does not need to exist on disk.
#[tauri::command]
pub fn from_path(path: String) -> Option<String> {
    mime_guess::from_path(&path).first_raw().map(String::from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn known_extensions_map_to_their_mime_type() {
        assert_eq!(
            from_extension("json".into()).as_deref(),
            Some("application/json")
        );
        assert_eq!(from_extension("html".into()).as_deref(), Some("text/html"));
        assert_eq!(from_extension("png".into()).as_deref(), Some("image/png"));
    }

    /// A leading dot is documented as tolerated, so `"json"` and `".json"` must
    /// produce the same result.
    #[test]
    fn leading_dot_is_stripped() {
        assert_eq!(
            from_extension(".json".into()),
            from_extension("json".into())
        );
    }

    #[test]
    fn unknown_extension_returns_none() {
        assert_eq!(from_extension("definitely-not-a-real-ext".into()), None);
    }

    #[test]
    fn path_maps_through_its_extension() {
        assert_eq!(
            from_path("/tmp/file.json".into()).as_deref(),
            Some("application/json")
        );
        assert_eq!(
            from_path("/tmp/file.html".into()).as_deref(),
            Some("text/html")
        );
    }

    /// Documented behaviour: only the string is inspected, so a path that does
    /// not exist on disk is still resolved.
    #[test]
    fn path_does_not_need_to_exist() {
        assert_eq!(
            from_path("/nope/does/not/exist.json".into()).as_deref(),
            Some("application/json")
        );
    }

    #[test]
    fn path_with_unknown_extension_returns_none() {
        assert_eq!(
            from_path("/tmp/file.definitely-not-a-real-ext".into()),
            None
        );
    }
}

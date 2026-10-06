// Copyright 2019-2024 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use md5::Md5;
use sha2::{Digest, Sha256, Sha512};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::error::{Error, Result};

/// Compute the hash of a UTF-8 string.
/// `algorithm` must be one of: "sha256", "sha512", "md5" (case-insensitive).
/// Returns the hex-encoded digest.
#[tauri::command]
pub async fn hash_string(input: String, algorithm: String) -> Result<String> {
    let data = input.into_bytes();
    match algorithm.to_lowercase().as_str() {
        "sha256" => {
            let mut h = Sha256::new();
            h.update(&data);
            Ok(hex::encode(h.finalize()))
        }
        "sha512" => {
            let mut h = Sha512::new();
            h.update(&data);
            Ok(hex::encode(h.finalize()))
        }
        "md5" => {
            let mut h = Md5::new();
            h.update(&data);
            Ok(hex::encode(h.finalize()))
        }
        other => Err(Error::Anyhow(format!(
            "unsupported algorithm: {other} (expected sha256|sha512|md5)"
        ))),
    }
}

/// Compute the hash of a file's contents, streamed in 8 KB chunks.
/// `algorithm` must be one of: "sha256", "sha512", "md5" (case-insensitive).
/// Returns the hex-encoded digest.
#[tauri::command]
pub async fn hash_file(path: String, algorithm: String) -> Result<String> {
    let mut file = File::open(&path).await.map_err(Error::from)?;
    let mut buf = [0u8; 8192];
    macro_rules! drain {
        ($h:expr) => {{
            let mut h = $h;
            loop {
                let n = file.read(&mut buf).await.map_err(Error::from)?;
                if n == 0 {
                    break;
                }
                h.update(&buf[..n]);
            }
            hex::encode(h.finalize())
        }};
    }
    match algorithm.to_lowercase().as_str() {
        "sha256" => Ok(drain!(Sha256::new())),
        "sha512" => Ok(drain!(Sha512::new())),
        "md5" => Ok(drain!(Md5::new())),
        other => Err(Error::Anyhow(format!(
            "unsupported algorithm: {other} (expected sha256|sha512|md5)"
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Digests of b"hello world", cross-checked against Node's
    // `crypto.createHash(<alg>).update(<input>).digest('hex')`.
    const SHA256_HELLO: &str = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    const SHA512_HELLO: &str = "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f";
    const MD5_HELLO: &str = "5eb63bbbe01eeed093cb22bb8f5acdc3";

    // `hash_file` reads in 8 KB chunks, so this 20 000-byte payload is larger
    // than one chunk and exercises the streaming loop (proving chunk boundaries
    // do not corrupt the digest). Reference digests come from Node's
    // `crypto.createHash(...)` over the byte pattern `i % 256` for i in 0..20000.
    const SHA256_PAYLOAD: &str = "290c84b9b148f3bc4dc2c6cbc847910f611e446e722eae6969438db9f4aecd57";
    const SHA512_PAYLOAD: &str = "be3f0b23969471ba0fcabdad6c58372f838b77146784347acd3c4bdc28c2a7dd749fa562767d20a733df4874694db024a60aabb9905be4e3b42d3c636d70b4d1";
    const MD5_PAYLOAD: &str = "c1707f7eba3bee6447922342feca0c9d";

    fn payload_multi_chunk() -> Vec<u8> {
        (0..20_000u32).map(|i| (i % 256) as u8).collect()
    }

    /// Writes `contents` to a unique file in the system temp directory and
    /// returns its path. The caller is responsible for removing it.
    async fn write_temp_file(contents: &[u8]) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "tauri-plugin-hash-test-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ));
        tokio::fs::write(&path, contents).await.unwrap();
        path
    }

    #[tokio::test]
    async fn sha256_matches_reference_vector() {
        assert_eq!(
            hash_string("hello world".into(), "sha256".into())
                .await
                .unwrap(),
            SHA256_HELLO
        );
    }

    #[tokio::test]
    async fn sha512_matches_reference_vector() {
        assert_eq!(
            hash_string("hello world".into(), "sha512".into())
                .await
                .unwrap(),
            SHA512_HELLO
        );
    }

    #[tokio::test]
    async fn md5_matches_reference_vector() {
        assert_eq!(
            hash_string("hello world".into(), "md5".into())
                .await
                .unwrap(),
            MD5_HELLO
        );
    }

    /// The algorithm name is documented as case-insensitive.
    #[tokio::test]
    async fn algorithm_is_case_insensitive() {
        assert_eq!(
            hash_string("hello world".into(), "SHA256".into())
                .await
                .unwrap(),
            SHA256_HELLO
        );
        assert_eq!(
            hash_string("hello world".into(), "Md5".into())
                .await
                .unwrap(),
            MD5_HELLO
        );
    }

    /// The empty string must still hash, to the well-known empty-input digest.
    #[tokio::test]
    async fn empty_input_is_supported() {
        assert_eq!(
            hash_string(String::new(), "sha256".into()).await.unwrap(),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[tokio::test]
    async fn unsupported_algorithm_is_rejected() {
        let err = hash_string("hello".into(), "crc32".into())
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("unsupported algorithm"),
            "got: {err}"
        );
    }

    /// A multi-chunk file must produce the same digest as hashing the whole
    /// buffer in one go, for every supported algorithm.
    #[tokio::test]
    async fn file_hashes_match_the_string_hashes_across_chunk_boundaries() {
        let path = write_temp_file(&payload_multi_chunk()).await;
        let p = path.to_string_lossy().to_string();

        assert_eq!(
            hash_file(p.clone(), "sha256".into()).await.unwrap(),
            SHA256_PAYLOAD
        );
        assert_eq!(
            hash_file(p.clone(), "sha512".into()).await.unwrap(),
            SHA512_PAYLOAD
        );
        assert_eq!(
            hash_file(p.clone(), "md5".into()).await.unwrap(),
            MD5_PAYLOAD
        );

        tokio::fs::remove_file(&path).await.unwrap();
    }

    #[tokio::test]
    async fn missing_file_reports_an_io_error() {
        let err = hash_file("/definitely/not/here.bin".into(), "sha256".into())
            .await
            .unwrap_err();
        assert!(
            matches!(err, Error::Io(_)),
            "expected Error::Io, got {err:?}"
        );
    }

    #[tokio::test]
    async fn unsupported_algorithm_is_rejected_for_files_too() {
        let path = write_temp_file(b"hello world").await;
        let err = hash_file(path.to_string_lossy().to_string(), "crc32".into())
            .await
            .unwrap_err();
        assert!(
            err.to_string().contains("unsupported algorithm"),
            "got: {err}"
        );
        tokio::fs::remove_file(&path).await.unwrap();
    }
}

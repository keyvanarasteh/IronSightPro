//! SHA-256 hash computation for binary integrity verification.
//! STEP 1: Streaming hash to avoid loading entire file into memory.

use sha2::{Digest, Sha256};
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

/// Chunk size for streaming hash (64 KiB).
const HASH_CHUNK_SIZE: usize = 64 * 1024;

/// Result of hashing a binary file.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HashResult {
    pub sha256: String,
    pub file_size: u64,
}

/// Compute SHA-256 hash of a file using streaming reads (STEP 1).
/// Never loads the entire file into memory.
pub fn compute_sha256(path: &Path) -> Result<HashResult, std::io::Error> {
    let file = fs::File::open(path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let mut hasher = Sha256::new();
    let mut reader = BufReader::with_capacity(HASH_CHUNK_SIZE, file);
    let mut buf = vec![0u8; HASH_CHUNK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buf)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buf[..bytes_read]);
    }

    let hash = hasher.finalize();
    Ok(HashResult {
        sha256: hex::encode(hash),
        file_size,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn hash_known_content() {
        let dir = std::env::temp_dir().join("ironsight_test_hash");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join("test.bin");
        let mut f = fs::File::create(&path).unwrap();
        f.write_all(b"hello world").unwrap();

        let result = compute_sha256(&path).unwrap();
        // SHA-256 of "hello world"
        assert_eq!(
            result.sha256,
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        assert_eq!(result.file_size, 11);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn hash_missing_file() {
        let result = compute_sha256(Path::new("/nonexistent/path/file.bin"));
        assert!(result.is_err());
    }
}

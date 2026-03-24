//! Shannon entropy calculation for packed/encrypted binary detection.
//!
//! Entropy range:
//! - 0.0–5.0: Normal application
//! - 5.0–7.0: Compressed/obfuscated
//! - 7.0–7.5: Probably packed
//! - 7.5+: Likely encrypted malware

use std::fs;
use std::path::Path;

/// Result of entropy analysis.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EntropyResult {
    pub entropy: f64,
    pub file_size: u64,
    pub risk_level: EntropyRisk,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EntropyRisk {
    Low,       // 0–5.0
    Medium,    // 5.0–7.0
    High,      // 7.0–7.5
    Critical,  // 7.5+
}

impl EntropyRisk {
    pub fn from_entropy(e: f64) -> Self {
        if e < 5.0 {
            Self::Low
        } else if e < 7.0 {
            Self::Medium
        } else if e < 7.5 {
            Self::High
        } else {
            Self::Critical
        }
    }
}

/// Compute Shannon entropy of a file (bits per byte, 0.0–8.0).
pub fn compute_entropy(path: &Path) -> Result<EntropyResult, std::io::Error> {
    let data = fs::read(path)?;
    let entropy = shannon_entropy(&data);
    Ok(EntropyResult {
        entropy,
        file_size: data.len() as u64,
        risk_level: EntropyRisk::from_entropy(entropy),
    })
}

/// Shannon entropy of a byte slice (0.0–8.0 bits).
pub fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut freq = [0u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in &freq {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn entropy_of_zeros() {
        // All same byte → entropy = 0.0
        let data = vec![0u8; 1024];
        let e = shannon_entropy(&data);
        assert!((e - 0.0).abs() < 0.001, "All zeros should have entropy ~0.0, got {e}");
    }

    #[test]
    fn entropy_of_random() {
        // Full range of bytes → entropy close to 8.0
        let mut data = Vec::new();
        for _ in 0..100 {
            for b in 0..=255u8 {
                data.push(b);
            }
        }
        let e = shannon_entropy(&data);
        assert!(e > 7.9, "Uniform distribution should have entropy ~8.0, got {e}");
    }

    #[test]
    fn entropy_of_text() {
        let data = b"Hello, this is a normal English text file with some repeated words. \
                      This text has normal entropy because letters are not uniformly distributed. \
                      The entropy should be somewhere in the normal range for text content.";
        let e = shannon_entropy(data);
        assert!(e > 3.0 && e < 6.0, "English text entropy should be 3-6, got {e}");
    }

    #[test]
    fn risk_level_classification() {
        assert_eq!(EntropyRisk::from_entropy(3.0), EntropyRisk::Low);
        assert_eq!(EntropyRisk::from_entropy(6.0), EntropyRisk::Medium);
        assert_eq!(EntropyRisk::from_entropy(7.2), EntropyRisk::High);
        assert_eq!(EntropyRisk::from_entropy(7.8), EntropyRisk::Critical);
    }

    #[test]
    fn entropy_of_empty() {
        let e = shannon_entropy(&[]);
        assert!((e - 0.0).abs() < 0.001);
    }

    #[test]
    fn compute_entropy_file() {
        let dir = std::env::temp_dir().join("ironsight_test_entropy");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join("test.txt");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(b"aaaaaaaaaa").unwrap(); // Low entropy

        let result = compute_entropy(&path).unwrap();
        assert!(result.entropy < 1.0);
        assert_eq!(result.risk_level, EntropyRisk::Low);

        let _ = std::fs::remove_dir_all(&dir);
    }
}

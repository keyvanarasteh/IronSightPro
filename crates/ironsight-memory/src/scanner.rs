//! Pattern scanning — search process memory for byte patterns (strings, signatures).
//!
//! On Linux, reads `/proc/<pid>/mem` with appropriate permissions.
//! STEP 1: Chunked reading to prevent OOM on large regions.
//! STEP 2: Error reporting via Result types.
//! STEP 4: Per-region entropy calculation.

use regex::bytes::Regex;
use serde::{Deserialize, Serialize};

/// Chunk size for reading process memory (4 MiB).
const READ_CHUNK_SIZE: usize = 4 * 1024 * 1024;

/// Maximum region size to scan (64 MiB).
const MAX_REGION_SIZE: u64 = 64 * 1024 * 1024;

/// A match found in memory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub offset: u64,
    pub region_start: u64,
    pub matched_bytes: Vec<u8>,
    pub matched_text: Option<String>,
    pub context: String,
}

/// Scan result for a process — STEP 2: includes errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub pid: u32,
    pub matches: Vec<PatternMatch>,
    pub regions_scanned: usize,
    pub regions_skipped: usize,
    pub errors: Vec<String>,
    pub total_bytes_scanned: u64,
}

/// Built-in suspicious patterns to scan for.
pub struct SuspiciousPatterns;

impl SuspiciousPatterns {
    /// Common strings found in malware / reverse shells.
    pub fn patterns() -> Vec<(&'static str, &'static str)> {
        vec![
            (r"/bin/sh", "Shell invocation — possible command execution"),
            (r"/bin/bash", "Bash invocation — possible command execution"),
            (r"cmd\.exe", "Windows cmd — possible command execution"),
            (r"powershell", "PowerShell invocation"),
            (r"wget\s+http", "wget download — possible payload fetch"),
            (r"curl\s+http", "curl download — possible payload fetch"),
            (r"nc\s+-[elp]", "Netcat listener — possible reverse shell"),
            (r"socket\(\s*AF_INET", "Raw socket creation"),
            (r"PRIVMSG\s+#", "IRC PRIVMSG — possible C2 communication"),
            (r"POST\s+/gate", "HTTP POST to /gate — common C2 beacon path"),
            (r"Mozilla/[45]\.\d", "User-Agent spoofing in memory"),
            (r"BEGIN\s+RSA\s+PRIVATE\s+KEY", "RSA private key in memory"),
            (r"password[=:]\s*\S+", "Plaintext password in memory"),
            (r"api[_-]?key[=:]\s*\S+", "API key in memory"),
        ]
    }
}

/// Calculate Shannon entropy of a byte buffer — STEP 4.
pub fn entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    let mut freq = [0u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }
    let len = data.len() as f64;
    freq.iter()
        .filter(|&&count| count > 0)
        .map(|&count| {
            let p = count as f64 / len;
            -p * p.log2()
        })
        .sum()
}

/// Scan a byte buffer for suspicious patterns.
pub fn scan_buffer(data: &[u8], base_address: u64) -> Vec<PatternMatch> {
    let mut matches = Vec::new();

    for (pattern_str, _description) in SuspiciousPatterns::patterns() {
        let re = match Regex::new(pattern_str) {
            Ok(r) => r,
            Err(_) => continue,
        };

        for m in re.find_iter(data) {
            let offset = m.start() as u64;
            let matched = m.as_bytes().to_vec();
            let text = String::from_utf8(matched.clone()).ok();

            // Context: 16 bytes before and after
            let ctx_start = m.start().saturating_sub(16);
            let ctx_end = (m.end() + 16).min(data.len());
            let ctx_bytes = &data[ctx_start..ctx_end];
            let context = format_context(ctx_bytes);

            matches.push(PatternMatch {
                offset: base_address + offset,
                region_start: base_address,
                matched_bytes: matched,
                matched_text: text,
                context,
            });
        }
    }

    matches
}

/// Scan a specific process's readable memory regions — STEP 1: chunked reading.
#[cfg(target_os = "linux")]
pub fn scan_process(pid: u32) -> ScanResult {
    use crate::maps;
    use std::io::{Read, Seek, SeekFrom};

    let mut result = ScanResult {
        pid,
        matches: Vec::new(),
        regions_scanned: 0,
        regions_skipped: 0,
        errors: Vec::new(),
        total_bytes_scanned: 0,
    };

    let regions = match maps::read_maps(pid) {
        Ok(r) => r,
        Err(e) => {
            result.errors.push(format!("Cannot read maps: {e}"));
            return result;
        }
    };

    let mem_path = format!("/proc/{pid}/mem");
    let mut file = match std::fs::File::open(&mem_path) {
        Ok(f) => f,
        Err(e) => {
            result.errors.push(format!("Cannot open /proc/{pid}/mem: {e}"));
            return result;
        }
    };

    for region in &regions {
        // Only scan readable regions, skip very large ones
        if !region.permissions.read || region.size() > MAX_REGION_SIZE {
            result.regions_skipped += 1;
            continue;
        }

        // Skip [vdso], [vsyscall], [vvar]
        if let Some(ref name) = region.pathname {
            if name.starts_with("[v") {
                result.regions_skipped += 1;
                continue;
            }
        }

        if file.seek(SeekFrom::Start(region.start)).is_err() {
            result.regions_skipped += 1;
            continue;
        }

        // ── STEP 1: Chunked reading ─────────────────────────────────────
        let region_size = region.size() as usize;
        let mut chunk_buf = vec![0u8; READ_CHUNK_SIZE.min(region_size)];
        let mut offset = 0usize;

        while offset < region_size {
            let to_read = READ_CHUNK_SIZE.min(region_size - offset);
            let buf = &mut chunk_buf[..to_read];

            match file.read_exact(buf) {
                Ok(_) => {
                    let base = region.start + offset as u64;
                    let matches = scan_buffer(buf, base);
                    result.matches.extend(matches);
                    result.total_bytes_scanned += to_read as u64;
                }
                Err(e) => {
                    result.errors.push(format!(
                        "Read error at region 0x{:x}+0x{:x}: {e}",
                        region.start, offset
                    ));
                    break;
                }
            }
            offset += to_read;
        }

        result.regions_scanned += 1;
    }

    result
}

#[cfg(not(target_os = "linux"))]
pub fn scan_process(pid: u32) -> ScanResult {
    ScanResult {
        pid,
        matches: Vec::new(),
        regions_scanned: 0,
        regions_skipped: 0,
        errors: vec!["Memory scanning not supported on this platform".into()],
        total_bytes_scanned: 0,
    }
}

/// Format bytes as a hex + ASCII context string.
fn format_context(data: &[u8]) -> String {
    let hex: Vec<String> = data.iter().map(|b| format!("{b:02x}")).collect();
    let ascii: String = data
        .iter()
        .map(|&b| if b.is_ascii_graphic() || b == b' ' { b as char } else { '.' })
        .collect();
    format!("{} | {}", hex.join(" "), ascii)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_detects_shell_invocation() {
        let data = b"normal data here /bin/sh -c whoami more data";
        let matches = scan_buffer(data, 0x1000);
        assert!(!matches.is_empty(), "Should detect /bin/sh");
        assert!(matches[0].matched_text.as_deref() == Some("/bin/sh"));
    }

    #[test]
    fn scan_detects_password() {
        let data = b"config password=s3cretP4ss! end";
        let matches = scan_buffer(data, 0x2000);
        assert!(!matches.is_empty(), "Should detect plaintext password");
    }

    #[test]
    fn scan_clean_buffer() {
        let data = b"This is perfectly normal application data with nothing suspicious.";
        let matches = scan_buffer(data, 0x3000);
        assert!(matches.is_empty(), "Clean data should have no matches");
    }

    #[test]
    fn context_formatting() {
        let data = b"Hello\x00World";
        let ctx = format_context(data);
        assert!(ctx.contains("48 65 6c 6c 6f")); // "Hello" in hex
        assert!(ctx.contains("Hello.World")); // ASCII with null as dot
    }

    #[test]
    fn suspicious_patterns_nonempty() {
        let patterns = SuspiciousPatterns::patterns();
        assert!(patterns.len() > 10, "Should have multiple suspicious patterns");
    }

    // ── STEP 4: Entropy tests ────────────────────────────────────────────

    #[test]
    fn entropy_empty_is_zero() {
        assert_eq!(entropy(&[]), 0.0);
    }

    #[test]
    fn entropy_uniform_is_zero() {
        let data = vec![0xAAu8; 1000];
        assert!(entropy(&data) < 0.01, "Uniform data should have ~0 entropy");
    }

    #[test]
    fn entropy_random_is_high() {
        // Simulated "random" data with all byte values
        let data: Vec<u8> = (0u8..=255).cycle().take(1024).collect();
        let e = entropy(&data);
        assert!(e > 7.9, "Random data should have high entropy, got {e}");
    }

    #[test]
    fn entropy_text_is_moderate() {
        let data = b"Hello world, this is a normal text string with some letters.";
        let e = entropy(data);
        assert!(e > 3.0 && e < 5.5, "Text should have moderate entropy, got {e}");
    }
}

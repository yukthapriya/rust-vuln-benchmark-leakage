// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0067
// CVE: N/A
// CWE: N/A
// Crate: lzf
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/lzf
// Extraction Method: repo_search
// GitHub Repo: badboy/lzf-rs
// Commit: a3d8e4b
// File: src/compress.rs


return Err(LzfError::NoCompressionPossible);
             }
 
            loop {
                 len += 1;
                while len < maxlen && data[ref_offset + len] == data[current_offset + len] {
                    len += 1;
                }
                break;
             }
 
             len -= 2; /* len is now #octets - 1 */
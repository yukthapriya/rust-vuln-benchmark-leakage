// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0006
// CVE: CVE-2024-58266
// CWE: N/A
// Crate: shlex
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/shlex
// Extraction Method: repo_search
// GitHub Repo: comex/rust-shlex
// Commit: 4a0724b
// File: fuzz/fuzz_targets/fuzz_next.rs


fuzz_target!(|data: &[u8]| {
     if let Ok(s) = std::str::from_utf8(data) {
         let mut sh = Shlex::new(s);
        while let Some(word) = sh.next() {}
     }
 });
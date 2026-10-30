// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0120
// CVE: CVE-2020-36440
// CWE: N/A
// Crate: libsbc
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/libsbc
// Extraction Method: repo_search
// GitHub Repo: mvertescher/libsbc-rs
// Commit: f7efbdf
// File: src/lib.rs


unsafe impl<R> Send for Decoder<R>
 where
        R: Read,
 {
 }
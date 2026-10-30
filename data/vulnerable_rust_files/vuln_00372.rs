// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0077
// CVE: CVE-2023-53157
// CWE: N/A
// Crate: rosenpass
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rosenpass
// Extraction Method: commit_diff
// GitHub Repo: rosenpass/rosenpass
// Commit: 9343985
// File: rosenpass/src/lib.rs


Oqs,
     #[error("error from external library while calling OQS")]
     OqsExternalLib,
    #[error("buffer size mismatch, required {required_size} but only found {actual_size}")]
     BufferSizeMismatch {
         required_size: usize,
         actual_size: usize,
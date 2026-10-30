// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0379
// CVE: N/A
// CWE: N/A
// Crate: fast-float
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/fast-float
// Extraction Method: repo_search
// GitHub Repo: aldanor/fast-float-rust
// Commit: 7dc383f
// File: src/common.rs


#[inline]
     pub fn check_len(&self, n: usize) -> bool {
        unsafe { self.ptr.add(n) <= self.end }
     }
 
     #[inline]
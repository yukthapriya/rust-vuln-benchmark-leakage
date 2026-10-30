// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0102
// CVE: CVE-2020-36209
// CWE: N/A
// Crate: late-static
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/late-static
// Extraction Method: repo_search
// GitHub Repo: Richard-W/late-static
// Commit: 185c5b0
// File: src/lib.rs


}
 
 unsafe impl<T: Send> core::marker::Send for LateStatic<T> {}
unsafe impl<T: Send> core::marker::Sync for LateStatic<T> {}
 
 impl<T> LateStatic<T> {
     /// Construct a LateStatic.
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0067
// CVE: N/A
// CWE: N/A
// Crate: fehler
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/fehler
// Extraction Method: repo_search
// GitHub Repo: withoutboats/fehler
// Commit: 0bde837
// File: src/lib.rs


/// This macro is equivalent to `Err($err)?`.
 #[macro_export]
 macro_rules! throw {
    ($err:expr)   => (::std::result::Result::<(), _>::Err($err)?);
 }
 
 /// Construct an ad-hoc exception from a string.
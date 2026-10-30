// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0030
// CVE: CVE-2021-28031
// CWE: N/A
// Crate: scratchpad
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/scratchpad
// Extraction Method: repo_search
// GitHub Repo: okready/scratchpad
// Commit: 18abeda
// File: src/lib.rs


pub use traits::*;
 pub use utility::*;
 
 #[cfg(test)]
 mod tests;
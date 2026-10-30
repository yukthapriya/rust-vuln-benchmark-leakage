// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0049
// CVE: N/A
// CWE: N/A
// Crate: scratchpad
// Type: overflow
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
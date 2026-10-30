// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0101
// CVE: CVE-2020-36208
// CWE: N/A
// Crate: conquer-once
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/conquer-once
// Extraction Method: repo_search
// GitHub Repo: oliver-giersch/conquer-once
// Commit: f75a52e
// File: src/lib.rs


#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
 #![deny(missing_docs)]
#![deny(clippy::missing_safety_doc)]
 
 #[cfg(test)]
 #[macro_use]
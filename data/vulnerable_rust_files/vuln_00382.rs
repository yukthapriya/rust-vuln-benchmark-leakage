// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0089
// CVE: N/A
// CWE: N/A
// Crate: atomic-polyfill
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/atomic-polyfill
// Extraction Method: commit_diff
// GitHub Repo: embassy-rs/atomic-polyfill
// Commit: 48e55c1
// File: src/lib.rs


#![no_std]
 #![cfg_attr(reexport_core, forbid(unsafe_code))]
 
 #[cfg(reexport_core)]
 pub use core::sync::atomic::*;
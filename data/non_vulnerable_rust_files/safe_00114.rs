// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0101 (FIXED)
// Crate: conquer-once
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: oliver-giersch/conquer-once
// Commit: f75a52e
// File: src/lib.rs


#![cfg_attr(all(not(test), not(feature = "std")), no_std)]
 #![deny(missing_docs)]
 
 #[cfg(test)]
 #[macro_use]
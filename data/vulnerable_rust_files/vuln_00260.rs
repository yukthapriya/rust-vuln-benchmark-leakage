// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0146
// CVE: N/A
// CWE: N/A
// Crate: twoway
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/twoway
// Extraction Method: commit_diff
// GitHub Repo: bluss/twoway
// Commit: e99b3c7
// File: src/lib.rs


#![cfg_attr(not(feature = "use_std"), no_std)]
 #![cfg_attr(feature = "pattern", feature(pattern))]
 
 //!
 //! Fast substring search for strings and byte strings, using the [two-way algorithm][tw].
 //!
// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0146 (FIXED)
// Crate: twoway
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: bluss/twoway
// Commit: e99b3c7
// File: src/lib.rs


#![cfg_attr(not(feature = "use_std"), no_std)]
 #![cfg_attr(feature = "pattern", feature(pattern))]
 
//! **This crate is deprecated. Use crate `memchr` instead.**
 //!
 //! Fast substring search for strings and byte strings, using the [two-way algorithm][tw].
 //!
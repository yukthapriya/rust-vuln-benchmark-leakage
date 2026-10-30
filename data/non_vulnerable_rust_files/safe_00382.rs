// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0089 (FIXED)
// Crate: atomic-polyfill
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: embassy-rs/atomic-polyfill
// Commit: 48e55c1
// File: src/lib.rs


#![no_std]
 #![cfg_attr(reexport_core, forbid(unsafe_code))]
#![doc = include_str!("../README.md")]
 
 #[cfg(reexport_core)]
 pub use core::sync::atomic::*;
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0014
// CVE: N/A
// CWE: N/A
// Crate: cortex-m-rt
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cortex-m-rt
// Extraction Method: repo_search
// GitHub Repo: rust-embedded/cortex-m
// Commit: edc79e2
// File: cortex-m-rt/src/lib.rs


//! }
 //! EOF
 //!
//! $ cargo rustc --target thumbv7m-none-eabi -- -C link-arg=-nostartfiles -C link-arg=-Tlink.x
//!
 //! $ file target/thumbv7m-none-eabi/debug/app
 //! app: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, (..)
 //! ```
 //!
 //! # Optional features
 //!
 //! ## `device`
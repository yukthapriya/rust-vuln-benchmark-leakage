// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0052
// CVE: N/A
// CWE: N/A
// Crate: async-std
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/async-std
// Extraction Method: repo_search
// GitHub Repo: async-rs/async-std
// Commit: 340933b
// File: src/lib.rs


//! # Async version of the Rust standard library
 //!
 //! `async-std` is a foundation of portable Rust software, a set of minimal and battle-tested
 //! <span
 //!   class="module-item stab portability"
 //!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! ><code>unstable</code></span>
 //! are available only when the `unstable` Cargo feature is enabled:
 //!
 //! ```toml
 //! <span
 //!   class="module-item stab portability"
 //!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! ><code>attributes</code></span>
 //! are available only when the `attributes` Cargo feature is enabled:
 //!
 //! ```toml
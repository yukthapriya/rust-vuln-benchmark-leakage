// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0035
// CVE: CVE-2022-35922
// CWE: N/A
// Crate: websocket
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/websocket
// Extraction Method: repo_search
// GitHub Repo: websockets-rs/rust-websocket
// Commit: 572693f
// File: src/lib.rs


//! other.
 //!
 //! You'll find many modules with `::sync` and `::async` submodules that separate these
//! behaviours. Since it get's tedious to add these on when appropriate a top-level
 //! convenience module called `websocket::sync` and `websocket::async` has been added that
 //! groups all the sync and async stuff, respectively.
 //!
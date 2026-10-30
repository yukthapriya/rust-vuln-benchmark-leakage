// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0035 (FIXED)
// Crate: websocket
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: websockets-rs/rust-websocket
// Commit: 572693f
// File: src/lib.rs


//! other.
 //!
 //! You'll find many modules with `::sync` and `::async` submodules that separate these
//! behaviours. Since it gets tedious to add these on when appropriate, a top-level
 //! convenience module called `websocket::sync` and `websocket::async` has been added that
 //! groups all the sync and async stuff, respectively.
 //!
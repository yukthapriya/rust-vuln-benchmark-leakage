// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0054
// CVE: N/A
// CWE: N/A
// Crate: wee_alloc
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/wee_alloc
// Extraction Method: repo_search
// GitHub Repo: rustwasm/wee_alloc
// Commit: 0379ad3
// File: wee_alloc/src/lib.rs


// Use `wee_alloc` as the global allocator.
 #[global_allocator]
 static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
 ```
 
 ## Does `wee_alloc` require nightly Rust?
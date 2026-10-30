// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0004
// CVE: CVE-2021-25901
// CWE: N/A
// Crate: lazy-init
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/lazy-init
// Extraction Method: repo_search
// GitHub Repo: khuey/lazy-init
// Commit: 90b2352
// File: src/lib.rs


//! memory barrier).
 
 use std::cell::UnsafeCell;
 use std::sync::Mutex;
 use std::sync::atomic::{AtomicBool, Ordering};
 
     }
 }
 
 #[cfg(test)]
 extern crate scoped_pool;
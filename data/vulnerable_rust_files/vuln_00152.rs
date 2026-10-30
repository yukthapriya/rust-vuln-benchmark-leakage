// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0148
// CVE: CVE-2020-36466
// CWE: N/A
// Crate: cgc
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cgc
// Extraction Method: repo_search
// GitHub Repo: playXE/cgc
// Commit: da4f656
// File: src/main.rs


extern crate gc_rs;
 
use gc_rs::*;
 
 fn main() {
     let mut gc = GarbageCollector::new(None);
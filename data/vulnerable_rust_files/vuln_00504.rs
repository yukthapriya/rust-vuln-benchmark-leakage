// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0029
// CVE: N/A
// CWE: N/A
// Crate: totally-safe
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/totally-safe
// Extraction Method: repo_search
// GitHub Repo: viktorlott/totally-safe
// Commit: 7bb5ef6
// File: examples/safety-is-important.rs


let [a, b] = value.as_mut_alias_array();
 
    assert_eq!(a, b);
 
     *a.1 = Some(b.0);
 
    assert_eq!(value, copied_value);
 
     mem::forget(copied_value);
 
    let mut m = Box::new(100);
    let n = m.copy();
 
    println!("{m:?} {n:?}");
 }
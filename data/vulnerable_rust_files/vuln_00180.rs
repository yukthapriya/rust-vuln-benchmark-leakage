// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0028
// CVE: CVE-2021-28028
// CWE: N/A
// Crate: toodee
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/toodee
// Extraction Method: repo_search
// GitHub Repo: antonmarsden/toodee
// Commit: e6e16d5
// File: src/tests.rs


toodee.reserve_exact(20);
         assert_eq!(toodee.capacity(), 20)
     }
 }
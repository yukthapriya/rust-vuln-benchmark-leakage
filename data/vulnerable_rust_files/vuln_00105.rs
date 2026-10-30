// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0091
// CVE: CVE-2020-35711
// CWE: N/A
// Crate: arc-swap
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/arc-swap
// Extraction Method: repo_search
// GitHub Repo: vorner/arc-swap
// Commit: 8e48095
// File: benches/track.rs


// Perform the benchmarks
         batch(c, name, &shared_number);
 
        // Ask the threads to terminate, so they don't disturb any other banchmarks
         stop.store(true, Ordering::Relaxed);
     })
     .unwrap();
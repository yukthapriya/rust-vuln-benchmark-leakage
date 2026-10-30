// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0122
// CVE: CVE-2020-36442
// CWE: N/A
// Crate: beef
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/beef
// Extraction Method: repo_search
// GitHub Repo: maciejhirsz/beef
// Commit: 6bb5597
// File: src/traits.rs


U: Capacity,
         {
             // Convert to `String::into_raw_parts` once stabilized
            let mut owned = ManuallyDrop::new(owned);
             let (fat, cap) = U::store(owned.len(), owned.capacity());
 
             (
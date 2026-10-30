// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0033
// CVE: CVE-2019-25008
// CWE: N/A
// Crate: http
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/http
// Extraction Method: repo_search
// GitHub Repo: hyperium/http
// Commit: 691af72
// File: src/header/map.rs


if capacity == 0 {
             Ok(Self::default())
         } else {
            let raw_cap = match to_raw_capacity(capacity).checked_next_power_of_two() {
                 Some(c) => c,
                 None => return Err(MaxSizeReached { _priv: () }),
             };
             .checked_add(additional)
             .ok_or_else(MaxSizeReached::new)?;
 
        let raw_cap = to_raw_capacity(cap);
 
         if raw_cap > self.indices.len() {
             let raw_cap = raw_cap
 }
 
 #[inline]
fn to_raw_capacity(n: usize) -> usize {
    match n.checked_add(n / 3) {
        Some(n) => n,
        None => panic!(
            "requested capacity {} too large: overflow while converting to raw capacity",
            n
        ),
    }
 }
 
 #[inline]
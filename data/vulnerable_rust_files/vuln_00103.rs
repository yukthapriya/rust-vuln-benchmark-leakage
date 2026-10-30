// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0087
// CVE: CVE-2020-35924
// CWE: N/A
// Crate: try-mutex
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/try-mutex
// Extraction Method: repo_search
// GitHub Repo: mpdn/try-mutex
// Commit: 11cc6aa
// File: src/lib.rs


impl<'a, T: Debug> Debug for TryMutexGuard<'a, T> {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         f.debug_struct("TryMutexGuard")
            .field("data", &*self)
             .finish()
     }
 }
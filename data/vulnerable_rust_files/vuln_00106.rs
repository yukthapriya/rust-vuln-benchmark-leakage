// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0092
// CVE: CVE-2020-35928
// CWE: N/A
// Crate: concread
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/concread
// Extraction Method: repo_search
// GitHub Repo: kanidm/concread
// Commit: 8394e6e
// File: src/internals/lincowcell/mod.rs


}
 }
 
 impl<T, R, U> LinCowCell<T, R, U>
 where
     T: LinCowCellCapable<R, U>,
 
         assert!(GC_COUNT.load(Ordering::Acquire) >= 50);
     }
 }
 
 #[cfg(test)]
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0036
// CVE: CVE-2019-25010
// CWE: N/A
// Crate: failure
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/failure
// Extraction Method: repo_search
// GitHub Repo: rust-lang-nursery/failure
// Commit: 6a12641
// File: src/compat.rs


}
     }
 
     impl From<Error> for Box<StdError + Send + Sync> {
         fn from(error: Error) -> Box<StdError + Send + Sync> {
             Box::new(Compat { error })
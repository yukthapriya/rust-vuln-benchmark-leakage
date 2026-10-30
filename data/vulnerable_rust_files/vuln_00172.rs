// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0014
// CVE: CVE-2021-26308
// CWE: N/A
// Crate: marc
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/marc
// Extraction Method: repo_search
// GitHub Repo: blackbeam/rust-marc
// Commit: 33184b9
// File: src/indicator.rs


///
     /// Will panic in case of non-utf8 indicator.
     pub fn second(&self) -> &str {
        std::str::from_utf8(&self.0[..1]).expect("non-utf8 indicator")
     }
 }
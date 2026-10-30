// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0147
// CVE: N/A
// CWE: N/A
// Crate: daemonize
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/daemonize
// Extraction Method: repo_search
// GitHub Repo: knsd/daemonize
// Commit: 862a320
// File: daemonize/src/lib.rs


pub first_child_exit_code: i32,
 }
 
/// Chiled process execution outcome.
 #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
 #[non_exhaustive]
 pub struct Child<T> {
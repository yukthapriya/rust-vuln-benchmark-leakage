// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0147 (FIXED)
// Crate: daemonize
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: knsd/daemonize
// Commit: 862a320
// File: daemonize/src/lib.rs


pub first_child_exit_code: i32,
 }
 
/// Child process execution outcome.
 #[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
 #[non_exhaustive]
 pub struct Child<T> {
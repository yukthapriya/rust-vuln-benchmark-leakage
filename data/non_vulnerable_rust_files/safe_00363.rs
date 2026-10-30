// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0067 (FIXED)
// Crate: fehler
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: withoutboats/fehler
// Commit: 0bde837
// File: src/lib.rs


/// This macro is equivalent to `Err($err)?`.
 #[macro_export]
 macro_rules! throw {
    ($err:expr)   => (return std::result::Result::Err(std::convert::From::from($err)))
 }
 
 /// Construct an ad-hoc exception from a string.
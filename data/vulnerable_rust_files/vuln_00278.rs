// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0023
// CVE: N/A
// CWE: N/A
// Crate: static_type_map
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/static_type_map
// Extraction Method: repo_search
// GitHub Repo: malobre/erased_set
// Commit: 2fabc28
// File: src/lib.rs


impl_erased_set! {
     /// A set of erased types.
     ///
    /// This set can store a single instance of any type that implements [`Any`].
     #[derive(Debug, Default)]
     pub struct ErasedSet: Any;
 }
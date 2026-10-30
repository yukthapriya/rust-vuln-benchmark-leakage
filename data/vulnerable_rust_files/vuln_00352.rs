// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0045
// CVE: N/A
// CWE: N/A
// Crate: memoffset
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/memoffset
// Extraction Method: repo_search
// GitHub Repo: Gilnaa/memoffset
// Commit: 8561baa
// File: src/offset_of.rs


/// ```
 ///
 /// ## Note
/// Due to macro_rules limitations, this macro will accept structs with a single field as well as unions.
 /// This is not a stable guarantee, and future versions of this crate might fail
 /// on any use of this macro with a struct, without a semver bump.
 #[macro_export(local_inner_macros)]
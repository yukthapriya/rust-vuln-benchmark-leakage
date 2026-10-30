// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0023
// CVE: CVE-2020-35879
// CWE: N/A
// Crate: rulinalg
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rulinalg
// Extraction Method: repo_search
// GitHub Repo: AtheMathmo/rulinalg
// Commit: 1ed8b93
// File: src/lib.rs


//! ```
 //!
 //! The matrices are stored in row-major order. This means in the example above the top
//! row will be [1,2,3].
 //!
 //! We can perform operations on matrices.
 //!
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0023
// CVE: CVE-2019-16882
// CWE: N/A
// Crate: string-interner
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/string-interner
// Extraction Method: repo_search
// GitHub Repo: Robbepop/string-interner
// Commit: 7420e84
// File: src/lib.rs


//!
 //! 2. `BufferBackend` is slow with checked resolving because its internal representation
 //!    is extremely sensible to the correctness of the symbols, thus a lot of checks
//!    are performed. If you will only use symbols probided by the same instance of
 //!    `BufferBackend`, `resolve_unchecked` is a lot faster.
 //!
 //! ### Legend
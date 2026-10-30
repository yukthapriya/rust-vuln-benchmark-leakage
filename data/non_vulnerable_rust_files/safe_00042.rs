// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0023 (FIXED)
// Crate: string-interner
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Robbepop/string-interner
// Commit: 7420e84
// File: src/lib.rs


//!
 //! 2. `BufferBackend` is slow with checked resolving because its internal representation
 //!    is extremely sensible to the correctness of the symbols, thus a lot of checks
//!    are performed. If you will only use symbols provided by the same instance of
 //!    `BufferBackend`, `resolve_unchecked` is a lot faster.
 //!
 //! ### Legend
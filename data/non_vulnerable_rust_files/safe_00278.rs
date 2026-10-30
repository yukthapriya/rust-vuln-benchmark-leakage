// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0023 (FIXED)
// Crate: static_type_map
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: malobre/erased_set
// Commit: 2fabc28
// File: src/lib.rs


impl_erased_set! {
     /// A set of erased types.
     ///
    /// This set can store a single instance of any type that implements [`Any`](::core::any::Any).
     #[derive(Debug, Default)]
     pub struct ErasedSet: Any;
 }
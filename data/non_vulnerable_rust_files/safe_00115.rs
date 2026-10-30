// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0102 (FIXED)
// Crate: late-static
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Richard-W/late-static
// Commit: 185c5b0
// File: src/lib.rs


}
 
 unsafe impl<T: Send> core::marker::Send for LateStatic<T> {}
unsafe impl<T: Sync> core::marker::Sync for LateStatic<T> {}
 
 impl<T> LateStatic<T> {
     /// Construct a LateStatic.
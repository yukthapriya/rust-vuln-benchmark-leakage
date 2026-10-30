// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0040 (FIXED)
// Crate: arenavec
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ibabushkin/arenavec
// Commit: f931efb
// File: src/common.rs


impl<T, H> SliceVec<T, H> {
     /// Create an immutable iterator over the elements of the vector.
    pub fn iter(&self) -> slice::Iter<'_, T> {
         self.slice.iter()
     }
 
     /// Create an mutable iterator over the elements of the vector.
    pub fn iter_mut(&mut self) -> slice::IterMut<'_, T> {
         self.slice.iter_mut()
     }
 }
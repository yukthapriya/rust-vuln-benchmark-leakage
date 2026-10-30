// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0053
// CVE: N/A
// CWE: N/A
// Crate: arenavec
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/arenavec
// Extraction Method: repo_search
// GitHub Repo: ibabushkin/arenavec
// Commit: f931efb
// File: src/common.rs


impl<T, H> SliceVec<T, H> {
     /// Create an immutable iterator over the elements of the vector.
    pub fn iter<'a>(&'a self) -> slice::Iter<'a, T> {
         self.slice.iter()
     }
 
     /// Create an mutable iterator over the elements of the vector.
    pub fn iter_mut<'a>(&'a mut self) -> slice::IterMut<'a, T> {
         self.slice.iter_mut()
     }
 }
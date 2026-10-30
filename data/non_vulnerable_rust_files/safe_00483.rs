// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0003 (FIXED)
// Crate: fast-float
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: aldanor/fast-float-rust
// Commit: 7dc383f
// File: src/common.rs


#[inline]
     pub fn check_len(&self, n: usize) -> bool {
        let len = self.end as usize - self.ptr as usize;
        n <= len
     }
 
     #[inline]
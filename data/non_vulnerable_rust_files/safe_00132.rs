// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0120 (FIXED)
// Crate: libsbc
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mvertescher/libsbc-rs
// Commit: f7efbdf
// File: src/lib.rs


unsafe impl<R> Send for Decoder<R>
 where
        R: Read + Send,
 {
 }
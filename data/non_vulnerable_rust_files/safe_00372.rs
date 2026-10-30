// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0077 (FIXED)
// Crate: rosenpass
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: rosenpass/rosenpass
// Commit: 9343985
// File: rosenpass/src/lib.rs


Oqs,
     #[error("error from external library while calling OQS")]
     OqsExternalLib,
    #[error("buffer size mismatch, required {required_size} but found {actual_size}")]
     BufferSizeMismatch {
         required_size: usize,
         actual_size: usize,
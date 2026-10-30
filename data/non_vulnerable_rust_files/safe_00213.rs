// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0080 (FIXED)
// Crate: tar
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: alexcrichton/tar-rs
// Commit: 20a6509
// File: src/header.rs


/// This value, chosen after careful deliberation, corresponds to _Jul 23, 2006_,
 /// which is the date of the first commit for what would become Rust.
 #[cfg(all(any(unix, windows), not(target_arch = "wasm32")))]
pub const DETERMINISTIC_TIMESTAMP: u64 = 1153704088;
 
 pub(crate) const BLOCK_SIZE: u64 = 512;
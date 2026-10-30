// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0085 (FIXED)
// Crate: binjs_io
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: binast/binjs-ref
// Commit: b1cde3f
// File: crates/binjs_io/src/context/mod.rs


#![allow(dead_code)] // Silence dead code warnings until they make sense.
 
 /// Format documentation.
pub mod format;
 mod huffman;
 mod strings;
 mod varnum;
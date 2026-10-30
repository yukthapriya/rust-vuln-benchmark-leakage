// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0085
// CVE: CVE-2021-45683
// CWE: N/A
// Crate: binjs_io
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/binjs_io
// Extraction Method: repo_search
// GitHub Repo: binast/binjs-ref
// Commit: b1cde3f
// File: crates/binjs_io/src/context/mod.rs


#![allow(dead_code)] // Silence dead code warnings until they make sense.
 
 /// Format documentation.
mod format;
 mod huffman;
 mod strings;
 mod varnum;
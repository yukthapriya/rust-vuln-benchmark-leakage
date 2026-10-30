// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0148 (FIXED)
// Crate: cgc
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: playXE/cgc
// Commit: da4f656
// File: src/main.rs


extern crate cgc;
 
use cgc::*;
 
 fn main() {
     let mut gc = GarbageCollector::new(None);
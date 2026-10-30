// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0095 (FIXED)
// Crate: wasmtime
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: bytecodealliance/wasmtime
// Commit: 7d857e6
// File: crates/component-util/src/lib.rs


impl DiscriminantSize {
     /// Calculate the size of discriminant needed to represent a variant with the specified number of cases.
     pub const fn from_count(count: usize) -> Option<Self> {
        if count <= 1 << 8 {
             Some(Self::Size1)
        } else if count <= 1 << 16 {
             Some(Self::Size2)
        } else if count as u64 <= 1 << 32 {
             Some(Self::Size4)
         } else {
             None
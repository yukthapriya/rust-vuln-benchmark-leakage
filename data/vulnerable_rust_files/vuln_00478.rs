// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0442
// CVE: N/A
// CWE: N/A
// Crate: wasmtime-jit-debug
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/wasmtime-jit-debug
// Extraction Method: repo_search
// GitHub Repo: bytecodealliance/wasmtime
// Commit: 7d857e6
// File: crates/component-util/src/lib.rs


impl DiscriminantSize {
     /// Calculate the size of discriminant needed to represent a variant with the specified number of cases.
     pub const fn from_count(count: usize) -> Option<Self> {
        if count <= 0xFF {
             Some(Self::Size1)
        } else if count <= 0xFFFF {
             Some(Self::Size2)
        } else if count <= 0xFFFF_FFFF {
             Some(Self::Size4)
         } else {
             None
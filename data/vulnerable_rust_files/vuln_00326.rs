// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0101
// CVE: CVE-2022-23636
// CWE: N/A
// Crate: wasmtime
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/wasmtime
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
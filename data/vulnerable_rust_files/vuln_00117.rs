// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0104
// CVE: CVE-2020-36211
// CWE: N/A
// Crate: gfwx
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/gfwx
// Extraction Method: repo_search
// GitHub Repo: Devolutions/gfwx-rs
// Commit: ba8cbdf
// File: src/header/mod.rs


// Issue https://github.com/rust-num/num-derive/issues/20
#![allow(clippy::useless_attribute)]
 
 use std::{io, usize};
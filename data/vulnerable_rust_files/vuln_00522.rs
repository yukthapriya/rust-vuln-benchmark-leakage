// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0054
// CVE: N/A
// CWE: N/A
// Crate: array-queue
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/array-queue
// Extraction Method: crate_versions
// GitHub Repo: array-queue
// Commit: N/A
// File: lib.rs


extern crate arrayvec;

mod array_queue;
mod error;

pub use array_queue::ArrayQueue;

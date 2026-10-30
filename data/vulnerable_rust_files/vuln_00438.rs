// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0376
// CVE: CVE-2024-47609
// CWE: N/A
// Crate: tonic
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/tonic
// Extraction Method: crate_versions
// GitHub Repo: tonic
// Commit: N/A
// File: lib.rs


#![cfg(feature = "broken")]

use criterion::*;

mod benchmarks;

criterion_group!(
    benches,
    benchmarks::request_response::bench_throughput,
    benchmarks::request_response_diverse_types::bench_throughput,
);
criterion_main!(benches);

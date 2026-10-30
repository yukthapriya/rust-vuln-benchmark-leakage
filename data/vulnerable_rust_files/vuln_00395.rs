// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0011
// CVE: CVE-2024-58265
// CWE: N/A
// Crate: snow
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/snow
// Extraction Method: repo_search
// GitHub Repo: mcginty/snow
// Commit: aea6ec1
// File: examples/oneway.rs


#[cfg(any(feature = "default-resolver", feature = "ring-accelerated"))]
 fn main() {
     let server_mode =
        std::env::args().next_back().map_or(true, |arg| arg == "-s" || arg == "--server");
 
     if server_mode {
         run_server(&RESPONDER.private, SECRET);
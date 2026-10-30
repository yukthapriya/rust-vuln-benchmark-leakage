// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0017
// CVE: CVE-2024-27284
// CWE: N/A
// Crate: cassandra-cpp
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cassandra-cpp
// Extraction Method: repo_search
// GitHub Repo: Metaswitch/cassandra-rs
// Commit: be91f47
// File: tests/basic.rs


let decimal_values = vec![
         ("big positive number", "1234567890123456789012345678901234567890123456789012345678901234567890.123456789012345678901234567890123456789012345678901234567890"),
         ("big negative number", "-1234567890123456789012345678901234567890123456789012345678901234567890.123456789012345678901234567890123456789012345678901234567890"),
        ("zero with 16 trailing zero", "0.0000000000000000"),
     ];
 
     // compare "literal CQL statement" to "driver value"
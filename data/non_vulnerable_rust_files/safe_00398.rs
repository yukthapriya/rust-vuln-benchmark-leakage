// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0017 (FIXED)
// Crate: cassandra-cpp
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Metaswitch/cassandra-rs
// Commit: be91f47
// File: tests/basic.rs


let decimal_values = vec![
         ("big positive number", "1234567890123456789012345678901234567890123456789012345678901234567890.123456789012345678901234567890123456789012345678901234567890"),
         ("big negative number", "-1234567890123456789012345678901234567890123456789012345678901234567890.123456789012345678901234567890123456789012345678901234567890"),
        ("zero with 4 trailing zero", "0.0000"),
     ];
 
     // compare "literal CQL statement" to "driver value"
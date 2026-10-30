// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0369
// CVE: CVE-2024-39697
// CWE: N/A
// Crate: phonenumber
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/phonenumber
// Extraction Method: repo_search
// GitHub Repo: whisperfish/rust-phonenumber
// Commit: 1d11bec
// File: src/formatter.rs


use crate::formatter::Mode;
     use crate::parser;
 
     #[test]
     fn us() {
         assert_eq!(
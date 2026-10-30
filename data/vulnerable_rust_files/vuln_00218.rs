// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0088
// CVE: CVE-2021-45686
// CWE: N/A
// Crate: csv-sniffer
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/csv-sniffer
// Extraction Method: repo_search
// GitHub Repo: jblondin/csv-sniffer
// Commit: 01eb1f8
// File: examples/worldbank.rs


use std::path::Path;
 
use csv::Terminator;
 use csv_sniffer::metadata::*;
 
 fn main() {
             num_preamble_rows: 4,
         },
         quote: Quote::Some(b'"'),
        doublequote_escapes: true,
        comment: Comment::Disabled,
        escape: Escape::Disabled,
        terminator: Terminator::CRLF,
         flexible: false,
     };
     let mut reader = dialect.open_path(data_filepath).unwrap();
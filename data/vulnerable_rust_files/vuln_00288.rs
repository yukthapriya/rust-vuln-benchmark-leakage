// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0044
// CVE: N/A
// CWE: N/A
// Crate: markdown
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/markdown
// Extraction Method: repo_search
// GitHub Repo: johannhof/markdown.rs
// Commit: c87ec8c
// File: src/html.rs


fn escape(text: &str, replace_entities: bool) -> String {
     lazy_static! {
        static ref AMPERSAND: Regex = Regex::new(r"&amp;(?P<x>\w+;)").unwrap();
     }
 
     let replaced = text
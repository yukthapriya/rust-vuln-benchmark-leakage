// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0044 (FIXED)
// Crate: markdown
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: johannhof/markdown.rs
// Commit: c87ec8c
// File: src/html.rs


fn escape(text: &str, replace_entities: bool) -> String {
     lazy_static! {
        static ref AMPERSAND: Regex = Regex::new(r"&amp;(?P<x>\S+;)").unwrap();
     }
 
     let replaced = text
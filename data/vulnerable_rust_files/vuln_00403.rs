// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0331
// CVE: N/A
// CWE: N/A
// Crate: puccinier
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/puccinier
// Extraction Method: repo_search
// GitHub Repo: catppuccin/toolbox
// Commit: 59bc0f6
// File: whiskers/src/frontmatter.rs


use std::collections::HashMap;
 
 #[derive(Debug)]
     // we consider a template to possibly have frontmatter iff:
     // * line 0 is "---"
     // * there is another "---" on another line
    let template = template.trim();
    let sep = "---";
    if !template.starts_with(sep) {
         return None;
     }
 
     template[sep.len()..]
        .split_once(sep)
        .map(|(a, b)| (a.trim(), b.trim()))
 }
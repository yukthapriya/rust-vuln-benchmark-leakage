// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0026
// CVE: CVE-2021-27671
// CWE: N/A
// Crate: comrak
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/comrak
// Extraction Method: repo_search
// GitHub Repo: kivikakk/comrak
// Commit: 7d1b3c8
// File: fuzz/fuzz_targets/gfm.rs


use comrak::{markdown_to_html, options, Options};
 
// Note that what I'm targetting here isn't exactly the same
 // as --gfm, but rather an approximation of what cmark-gfm
 // options are routinely used by Commonmarker users.
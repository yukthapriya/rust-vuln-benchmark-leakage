// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0071
// CVE: CVE-2021-3013
// CWE: N/A
// Crate: grep-cli
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/grep-cli
// Extraction Method: repo_search
// GitHub Repo: BurntSushi/ripgrep
// Commit: 5748f81
// File: crates/printer/src/lib.rs


*/
 
 #![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
 
 pub use crate::{
     color::{ColorError, ColorSpecs, UserColorSpec, default_color_specs},
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0054
// CVE: N/A
// CWE: N/A
// Crate: directories
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/directories
// Extraction Method: repo_search
// GitHub Repo: dirs-dev/directories-rs
// Commit: 0a890f0
// File: src/mac.rs


data_local_dir: data_local_dir,
             executable_dir: None,
             preference_dir: preference_dir,
            runtime_dir:    None
         };
         Some(base_dirs)
     } else {
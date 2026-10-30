// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0009
// CVE: CVE-2021-25906
// CWE: N/A
// Crate: basic_dsp_matrix
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/basic_dsp_matrix
// Extraction Method: repo_search
// GitHub Repo: liebharc/basic_dsp
// Commit: 9c2d5e4
// File: examples/bench_tables.rs


print!("{:?}, ", value);
     }
     println!("");
    let mut sorted: Vec<_> = results.keys().map(|x| x.clone()).collect();
     sorted.sort();
     for key in &sorted {
         print!("{:?} [ns], ", key);
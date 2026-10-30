// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0009 (FIXED)
// Crate: basic_dsp_matrix
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: liebharc/basic_dsp
// Commit: 9c2d5e4
// File: examples/bench_tables.rs


print!("{:?}, ", value);
     }
     println!("");
    let mut sorted: Vec<_> = results.keys().map(|x| (*x)).collect();
     sorted.sort();
     for key in &sorted {
         print!("{:?} [ns], ", key);
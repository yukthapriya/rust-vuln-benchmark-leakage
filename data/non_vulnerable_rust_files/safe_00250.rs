// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0131 (FIXED)
// Crate: brotli-sys
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: bitemyapp/brotli2-rs
// Commit: 98cdd69
// File: examples/all-read-write-roundtrips.rs


for &data in datas.iter() {
         check(data)
     }
    let mut rng = rand::thread_rng();
     for _ in 0..3 {
         let rnum: usize = rng.gen_range(1, 100*1024*1024);
         let mut buf = vec![0; rnum];
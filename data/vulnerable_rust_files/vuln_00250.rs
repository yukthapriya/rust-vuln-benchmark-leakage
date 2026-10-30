// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0131
// CVE: CVE-2020-36846
// CWE: N/A
// Crate: brotli-sys
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/brotli-sys
// Extraction Method: repo_search
// GitHub Repo: bitemyapp/brotli2-rs
// Commit: 98cdd69
// File: examples/all-read-write-roundtrips.rs


for &data in datas.iter() {
         check(data)
     }
    let mut rng = rand::XorShiftRng::from_seed([1; 16]);
     for _ in 0..3 {
         let rnum: usize = rng.gen_range(1, 100*1024*1024);
         let mut buf = vec![0; rnum];
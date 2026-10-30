// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0133
// CVE: CVE-2020-36453
// CWE: N/A
// Crate: scottqueue
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/scottqueue
// Extraction Method: repo_search
// GitHub Repo: rossdylan/rust-scottqueue
// Commit: 4c8a90e
// File: src/tlqueue.rs


drop(tx);
             });
         }
        for i in 0..nmsgs {
             let msg = rx.recv().unwrap();
             assert!(!end_set.contains(&msg));
             end_set.insert(msg);
     #[bench]
     fn pop_bench(b: &mut Bencher) {
         let q = super::Queue::new();
        for i in 0..10000 {
             q.push(i);
         }
         b.iter(|| q.pop());
     }
 }
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0106
// CVE: CVE-2020-36214
// CWE: N/A
// Crate: multiqueue2
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/multiqueue2
// Extraction Method: repo_search
// GitHub Repo: abbychau/multiqueue2
// Commit: 0b090af
// File: benches/multiqueue_benchmarks.rs


handle.join().unwrap();
                         }
                         
                        let expected_sum: usize = (0..MESSAGE_COUNT).sum::<usize>() * streams * consumers_per_stream;
                         assert_eq!(counter.load(Ordering::Relaxed), expected_sum);
                     });
                 },
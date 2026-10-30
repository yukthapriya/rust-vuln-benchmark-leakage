// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0106 (FIXED)
// Crate: multiqueue2
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: abbychau/multiqueue2
// Commit: 0b090af
// File: benches/multiqueue_benchmarks.rs


handle.join().unwrap();
                         }
                         
                        let expected_sum: usize = (0..MESSAGE_COUNT).sum::<usize>() * streams;
                         assert_eq!(counter.load(Ordering::Relaxed), expected_sum);
                     });
                 },
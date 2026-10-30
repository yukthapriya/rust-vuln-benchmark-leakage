// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0091 (FIXED)
// Crate: arc-swap
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: vorner/arc-swap
// Commit: 8e48095
// File: benches/track.rs


// Perform the benchmarks
         batch(c, name, &shared_number);
 
        // Ask the threads to terminate, so they don't disturb any other benchmarks
         stop.store(true, Ordering::Relaxed);
     })
     .unwrap();
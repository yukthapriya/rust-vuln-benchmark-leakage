// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0028 (FIXED)
// Crate: toodee
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: antonmarsden/toodee
// Commit: e6e16d5
// File: src/tests.rs


toodee.reserve_exact(20);
         assert_eq!(toodee.capacity(), 20)
     }

    #[test]
    fn remove_col_zero() {
        let mut toodee: TooDee<u32> = TooDee::init(2, 2, 0);
        toodee.remove_col(0);
    }
 }
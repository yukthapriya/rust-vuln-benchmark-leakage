// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0087 (FIXED)
// Crate: try-mutex
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mpdn/try-mutex
// Commit: 11cc6aa
// File: src/lib.rs


impl<'a, T: Debug> Debug for TryMutexGuard<'a, T> {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         f.debug_struct("TryMutexGuard")
            .field("data", &**self)
             .finish()
     }
 }
// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0111 (FIXED)
// Crate: may_queue
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Xudong-Huang/may
// Commit: 8cc314b
// File: src/os/unix/net.rs


///     }
     /// }
     /// ```
    pub fn incoming(&self) -> Incoming<'_> {
         Incoming { listener: self }
     }
 }
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0111
// CVE: CVE-2020-36217
// CWE: N/A
// Crate: may_queue
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/may_queue
// Extraction Method: repo_search
// GitHub Repo: Xudong-Huang/may
// Commit: 8cc314b
// File: src/os/unix/net.rs


///     }
     /// }
     /// ```
    pub fn incoming(&self) -> Incoming {
         Incoming { listener: self }
     }
 }
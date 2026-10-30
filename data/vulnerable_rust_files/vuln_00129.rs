// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0117
// CVE: CVE-2020-36437
// CWE: N/A
// Crate: conqueue
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/conqueue
// Extraction Method: repo_search
// GitHub Repo: longshorej/conqueue
// Commit: 079ef99
// File: src/lib.rs


}
 }
 
unsafe impl<T> Sync for QueueSender<T> {}
 
unsafe impl<T> Send for QueueSender<T> {}
 
 /// A `QueueReceiver` is used to pop previously
 /// pushed items from the queue.
     }
 }
 
unsafe impl<T> Send for QueueReceiver<T> {}
 
 pub struct Queue;
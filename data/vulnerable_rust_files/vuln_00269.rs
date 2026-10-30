// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0006
// CVE: N/A
// CWE: N/A
// Crate: thread_local
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/thread_local
// Extraction Method: repo_search
// GitHub Repo: Amanieu/thread_local-rs
// Commit: 33f151b
// File: src/lib.rs


fn size_hint<T: Send>(&self, thread_local: &ThreadLocal<T>) -> (usize, Option<usize>) {
         let total = thread_local.values.load(Ordering::Relaxed);
        (total - self.yielded, None)
     }
 
     fn size_hint_frozen<T: Send>(&self, thread_local: &ThreadLocal<T>) -> (usize, Option<usize>) {
         let total = thread_local.values.load(Ordering::Relaxed);
         let remaining = total - self.yielded;
         (remaining, Some(remaining))
     }
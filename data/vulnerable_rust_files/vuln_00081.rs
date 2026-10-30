// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0047
// CVE: CVE-2020-35900
// CWE: N/A
// Crate: array-queue
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/array-queue
// Extraction Method: repo_search
// GitHub Repo: raviqqe/array-queue
// Commit: d8931b7
// File: src/array_queue.rs


if self.is_empty() {
             None
         } else {
            let x = replace(&mut self.array[self.start], MaybeUninit::uninit());
             self.start = self.index(1);
             self.length -= 1;
 
         if self.is_empty() {
             None
         } else {
            let x = replace(&mut self.array[self.length - 1], MaybeUninit::uninit());
             self.length -= 1;
 
             // SAFETY: An element exists at the last index.
 
         assert_eq!(unsafe { BAR_SUM }, 32);
     }
 }
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0119
// CVE: CVE-2020-36439
// CWE: N/A
// Crate: ticketed_lock
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ticketed_lock
// Extraction Method: repo_search
// GitHub Repo: kvark/ticketed_lock
// Commit: 734a076
// File: src/lib.rs


}
 
 
/// The read-only guard of data, allowing `&T` dereferences.
 pub struct WriteLockGuard<T> {
     _inner: raw::LockGuard,
     data: Arc<UnsafeCell<T>>,
         }
     }
 
    /// Acquire a write-only ticket.
     pub fn write(&mut self) -> WriteTicket<T> {
         WriteTicket {
             inner: self.inner.write(),
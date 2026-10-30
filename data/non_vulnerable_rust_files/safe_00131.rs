// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0119 (FIXED)
// Crate: ticketed_lock
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: kvark/ticketed_lock
// Commit: 734a076
// File: src/lib.rs


}
 
 
/// The read/write guard of data, allowing `&mut T` dereferences.
 pub struct WriteLockGuard<T> {
     _inner: raw::LockGuard,
     data: Arc<UnsafeCell<T>>,
         }
     }
 
    /// Acquire a read/write ticket.
     pub fn write(&mut self) -> WriteTicket<T> {
         WriteTicket {
             inner: self.inner.write(),
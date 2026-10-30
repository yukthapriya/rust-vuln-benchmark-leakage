// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0113 (FIXED)
// Crate: atomic-option
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: reem/rust-atomic-option
// Commit: 9db3244
// File: src/lib.rs


/// An atomic version of `Option<Box<T>>`, useful for moving owned objects
 /// between threads in a wait-free manner.
pub struct AtomicOption<T> {
     // Contains the address of a Box<T>, or 0 to indicate None
     inner: AtomicUsize,
     phantom: PhantomData<Option<Box<T>>>
 }
 
unsafe impl<T> Sync for AtomicOption<T> {}
 
impl<T> AtomicOption<T> {
     /// Create a new AtomicOption storing the specified data.
     ///
     /// ```
     }
 }
 
impl<T> From<Option<Box<T>>> for AtomicOption<T> {
     #[inline]
     fn from(opt: Option<Box<T>>) -> AtomicOption<T> {
         match opt {
     }
 }
 
impl<T> Drop for AtomicOption<T> {
     fn drop(&mut self) {
         let _ = self.take(Ordering::SeqCst);
     }
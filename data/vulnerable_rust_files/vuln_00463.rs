// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0425
// CVE: N/A
// CWE: N/A
// Crate: get-size
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/get-size
// Extraction Method: repo_search
// GitHub Repo: DKerp/get-size
// Commit: db41812
// File: src/lib.rs


impl<T> GetSize for Rc<T> where T: GetSize + 'static {
     fn get_heap_size(&self) -> usize {
        GetSize::get_size(&**self)
     }
 
     fn get_heap_size_with_tracker<TR: GetSizeTracker>(
         let addr = Rc::as_ptr(&strong_ref);
 
         if tracker.track(addr, strong_ref) {
            (GetSize::get_size(&**self), tracker)
         } else {
             (0, tracker)
         }
 
 impl<T> GetSize for Arc<T> where T: GetSize + 'static {
     fn get_heap_size(&self) -> usize {
        GetSize::get_size(&**self)
     }
 
     fn get_heap_size_with_tracker<TR: GetSizeTracker>(
         let addr = Arc::as_ptr(&strong_ref);
 
         if tracker.track(addr, strong_ref) {
            (GetSize::get_size(&**self), tracker)
         } else {
             (0, tracker)
         }
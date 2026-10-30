// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0427 (FIXED)
// Crate: get-size-derive
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: DKerp/get-size
// Commit: db41812
// File: src/lib.rs


impl<T> GetSize for Rc<T> where T: GetSize + 'static {
     fn get_heap_size(&self) -> usize {
        let tracker = StandardTracker::default();

        let (total, _) = GetSize::get_heap_size_with_tracker(self, tracker);

        total
     }
 
     fn get_heap_size_with_tracker<TR: GetSizeTracker>(
         let addr = Rc::as_ptr(&strong_ref);
 
         if tracker.track(addr, strong_ref) {
            GetSize::get_size_with_tracker(&**self, tracker)
         } else {
             (0, tracker)
         }
 
 impl<T> GetSize for Arc<T> where T: GetSize + 'static {
     fn get_heap_size(&self) -> usize {
        let tracker = StandardTracker::default();

        let (total, _) = GetSize::get_heap_size_with_tracker(self, tracker);

        total
     }
 
     fn get_heap_size_with_tracker<TR: GetSizeTracker>(
         let addr = Arc::as_ptr(&strong_ref);
 
         if tracker.track(addr, strong_ref) {
            GetSize::get_size_with_tracker(&**self, tracker)
         } else {
             (0, tracker)
         }
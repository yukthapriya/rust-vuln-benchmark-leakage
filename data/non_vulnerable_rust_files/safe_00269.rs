// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0006 (FIXED)
// Crate: thread_local
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Amanieu/thread_local-rs
// Commit: 33f151b
// File: src/lib.rs


fn size_hint<T: Send>(&self, thread_local: &ThreadLocal<T>) -> (usize, Option<usize>) {
         let total = thread_local.values.load(Ordering::Relaxed);

        // NOTE: `saturating_sub` is required here to avoid integer underflow during
        // concurrent insertion and iteration. The shortest dangerous interleaving is:
        //
        // - Thread A inserts, pausing after `present = true` but *before* `values = 1`
        // - Thread B iterates, sees `present = true`, and sets `yielded = 1`
        // - Thread B calls `size_hint` and sees `values = 0` and `yielded = 1`
        (total.saturating_sub(self.yielded), None)
     }
 
     fn size_hint_frozen<T: Send>(&self, thread_local: &ThreadLocal<T>) -> (usize, Option<usize>) {
         let total = thread_local.values.load(Ordering::Relaxed);

        // NOTE: this method assumes no concurrent insertion to `thread_local`,
        // so this subtraction cannot underflow as in `size_hint` above.
         let remaining = total - self.yielded;
         (remaining, Some(remaining))
     }
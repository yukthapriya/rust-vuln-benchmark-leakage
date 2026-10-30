// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0117 (FIXED)
// Crate: conqueue
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: longshorej/conqueue
// Commit: 079ef99
// File: src/lib.rs


}
 }
 
unsafe impl<T: Send> Sync for QueueSender<T> {}
 
unsafe impl<T: Send> Send for QueueSender<T> {}
 
 /// A `QueueReceiver` is used to pop previously
 /// pushed items from the queue.
     }
 }
 
unsafe impl<T: Send> Send for QueueReceiver<T> {}
 
 pub struct Queue;
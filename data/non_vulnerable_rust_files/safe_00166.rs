// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0004 (FIXED)
// Crate: lazy-init
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: khuey/lazy-init
// Commit: 90b2352
// File: src/lib.rs


//! memory barrier).
 
 use std::cell::UnsafeCell;
use std::fmt;
 use std::sync::Mutex;
 use std::sync::atomic::{AtomicBool, Ordering};
 
     }
 }
 
impl<T> fmt::Debug for Lazy<T>
    where T: fmt::Debug + Sync
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(v) = self.get() {
            f.write_fmt(format_args!("Lazy({:?})", v))
        } else {
            f.write_str("Lazy(<uninitialized>)")
        }
    }
}

 #[cfg(test)]
 extern crate scoped_pool;
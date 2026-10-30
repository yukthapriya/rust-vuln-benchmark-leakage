// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0137 (FIXED)
// Crate: lever
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: vertexclique/lever
// Commit: 93d5f36
// File: src/sync/atomics.rs


use crate::sync::arcunique::ArcUnique;
use anyhow::Result;
 use std::convert::TryFrom;
 use std::ops::Deref;
 use std::sync::atomic::{AtomicPtr, Ordering};
         Arc::into_raw(total) as *mut T
     }
 
    fn strongest_failure_ordering(order: Ordering) -> Ordering {
        use Ordering::*;
        match order {
            Release => Relaxed,
            Relaxed => Relaxed,
            SeqCst => SeqCst,
            Acquire => Acquire,
            AcqRel => Acquire,
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }

     fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
        match self.ptr.compare_exchange(
            current,
            new,
            order,
            Self::strongest_failure_ordering(order),
        ) {
            Ok(x) => x,
            Err(x) => x,
        }
     }
 
     fn take(&self) -> Arc<T> {
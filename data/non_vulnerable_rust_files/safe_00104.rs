// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0088 (FIXED)
// Crate: magnetic
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: johnshaw/magnetic
// Commit: 2e7fbe5
// File: src/buffer/array.rs


//! Buffer backed by an array
 
use std::{cell::UnsafeCell, mem::MaybeUninit};
 
 use super::Buffer;
 
 /// Holds data locally in an array (no heap allocation)
pub struct ArrayBuffer<T, const CAP: usize>([UnsafeCell<MaybeUninit<T>>; CAP]);
 
 impl<T, const CAP: usize> ArrayBuffer<T, CAP> {
     /// Create a new `ArrayBuffer`. This method will return an error if the capacity is not valid.
 }
 
 unsafe impl<T: Send, const CAP: usize> Send for ArrayBuffer<T, CAP> {}
unsafe impl<T, const CAP: usize> Sync for ArrayBuffer<T, CAP> {}
 
 impl<T, const CAP: usize> Buffer<T> for ArrayBuffer<T, CAP> {
     #[inline(always)]
     }
 
     #[inline(always)]
    fn at(&self, idx: usize) -> *const UnsafeCell<MaybeUninit<T>> {
        &self.0[idx % CAP] as * const _
     }
 }
 
         let size = buf.size();
         for i in 0..size {
             assert_eq!(buf.at(i), buf.at(i));
             assert_eq!(buf.at(i), buf.at(size + i));
             assert!(buf.at(i) != buf.at(i + 1));
         }
     }
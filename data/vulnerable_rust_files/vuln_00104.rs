// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0088
// CVE: CVE-2020-35925
// CWE: N/A
// Crate: magnetic
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/magnetic
// Extraction Method: repo_search
// GitHub Repo: johnshaw/magnetic
// Commit: 2e7fbe5
// File: src/buffer/array.rs


//! Buffer backed by an array
 
use std::mem::MaybeUninit;
 
 use super::Buffer;
 
 /// Holds data locally in an array (no heap allocation)
pub struct ArrayBuffer<T, const CAP: usize>([MaybeUninit<T>; CAP]);
 
 impl<T, const CAP: usize> ArrayBuffer<T, CAP> {
     /// Create a new `ArrayBuffer`. This method will return an error if the capacity is not valid.
 }
 
 unsafe impl<T: Send, const CAP: usize> Send for ArrayBuffer<T, CAP> {}
 
 impl<T, const CAP: usize> Buffer<T> for ArrayBuffer<T, CAP> {
     #[inline(always)]
     }
 
     #[inline(always)]
    fn at(&self, idx: usize) -> *const T {
        self.0[idx % CAP].as_ptr()
    }

    fn at_mut(&mut self, idx: usize) -> *mut T {
        self.0[idx % CAP].as_mut_ptr()
     }
 }
 
         let size = buf.size();
         for i in 0..size {
             assert_eq!(buf.at(i), buf.at(i));
            assert_eq!(buf.at_mut(i), buf.at_mut(i));
             assert_eq!(buf.at(i), buf.at(size + i));
            assert_eq!(buf.at_mut(i), buf.at_mut(size + i));
             assert!(buf.at(i) != buf.at(i + 1));
            assert!(buf.at_mut(i) != buf.at_mut(i + 1));
         }
     }
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0034
// CVE: CVE-2020-35886
// CWE: N/A
// Crate: arr
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/arr
// Extraction Method: repo_search
// GitHub Repo: sjep/array
// Commit: 34380d8
// File: src/lib.rs


use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
 use std::ops::{Index, IndexMut, Range};
 
use crate::zeroable::Zeroable;
 
 pub struct Array<T> {
     size: usize,
     ptr: *mut T,
 }
 
unsafe impl<T> Sync for Array<T>{}
unsafe impl<T> Send for Array<T>{}
 
 impl<T> Array<T> {
     /// Create an immutable iterator over elements in Array.
         unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
     }
 
    //pub fn to_slice_mut<'a>(&'a mut self) -> &'a mut [T] {
    //    slice::

     /// The length of the array (number of elements T)
     pub fn len(&self) -> usize {
         self.size
         };
         for i in 0..size {
             unsafe {
                (*(ptr.wrapping_offset(i as isize))) = template.clone();
             }
         }
         Self{size, ptr}
     type Output = T;
 
     fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_ref()
        }.unwrap()
     }
 }
 
 impl<T> IndexMut<usize> for Array<T> {
 
     fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {

        unsafe {
            self.ptr.wrapping_offset(idx as isize).as_mut()
        }.unwrap()
     }
 }
 
     fn drop(&mut self) {
         let objsize = std::mem::size_of::<T>();
         let layout = Layout::from_size_align(self.size * objsize, 8).unwrap();
         unsafe {
             dealloc(self.ptr as *mut u8, layout);
         }
     }
         }
         assert_eq!(arr[4], 5);
     }
 }
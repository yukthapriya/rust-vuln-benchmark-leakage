// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0034 (FIXED)
// Crate: arr
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: sjep/array
// Commit: 34380d8
// File: src/lib.rs


use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
 use std::ops::{Index, IndexMut, Range};
 
pub use crate::zeroable::Zeroable;
 
 pub struct Array<T> {
     size: usize,
     ptr: *mut T,
 }
 
unsafe impl<T: Sync> Sync for Array<T>{}
unsafe impl<T: Send> Send for Array<T>{}
 
 impl<T> Array<T> {
     /// Create an immutable iterator over elements in Array.
         unsafe { std::slice::from_raw_parts_mut(self.ptr, self.size) }
     }
 
     /// The length of the array (number of elements T)
     pub fn len(&self) -> usize {
         self.size
         };
         for i in 0..size {
             unsafe {
                ptr.wrapping_offset(i as isize).write(template.clone());
             }
         }
         Self{size, ptr}
     type Output = T;
 
     fn index<'a>(&'a self, idx: usize) -> &'a Self::Output {
        &self.to_slice()[idx]
     }
 }
 
 impl<T> IndexMut<usize> for Array<T> {
 
     fn index_mut<'a>(&'a mut self, idx: usize) -> &'a mut Self::Output {
        &mut self.to_slice_mut()[idx]
     }
 }
 
     fn drop(&mut self) {
         let objsize = std::mem::size_of::<T>();
         let layout = Layout::from_size_align(self.size * objsize, 8).unwrap();

         unsafe {
            for i in 0..(self.size as isize) {
                std::ptr::drop_in_place(self.ptr.wrapping_offset(i));
            }
             dealloc(self.ptr as *mut u8, layout);
         }
     }
         }
         assert_eq!(arr[4], 5);
     }

    #[test]
    fn test_sync() {
        let arr = Array::new_from_template(1, &&1usize);

        std::thread::spawn(move || {
            // shouldn't be allowed!
            let _ = arr[0].clone();
        }).join().unwrap();
    }
 }
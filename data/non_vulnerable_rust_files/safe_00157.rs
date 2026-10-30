// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0158 (FIXED)
// Crate: slice-deque
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: gnzlbg/slice_deque
// Commit: 1c8edcb
// File: src/lib.rs


///         assert_eq!(s.len(), cap - len);
     ///         // We can write to them and for example bump the tail of
     ///         // the deque:
    ///         s[0] = std::mem::MaybeUninit::new(4);
    ///         s[1] = std::mem::MaybeUninit::new(5);
     ///     }
     ///     d.move_tail(2);
     /// }
     /// assert_eq!(d, sdeq![1, 2, 3, 4, 5]);
     /// # }
     /// ```
    pub unsafe fn tail_head_slice(&mut self) -> &mut [mem::MaybeUninit<T>] {
         let ptr = self.as_mut_slice().as_mut_ptr().add(self.len());
        slice::from_raw_parts_mut(ptr as _, self.capacity() - self.len())
     }
 
     /// Attempts to reserve capacity for inserting at least `additional`
 
         for i in 0..slice.len() {
             // segfault:
            slice[i] = mem::MaybeUninit::new(0);
         }
     }
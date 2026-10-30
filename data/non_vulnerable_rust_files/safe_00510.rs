// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0042 (FIXED)
// Crate: static-alloc
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: 197g/static-alloc
// Commit: 2c78217
// File: static-alloc/src/unsync/bump.rs


let ptr = NonNull::new(unsafe { alloc::alloc::alloc(layout) })
             .unwrap_or_else(|| alloc::alloc::handle_alloc_error(layout));
         let ptr = ptr::slice_from_raw_parts_mut(ptr.as_ptr(), capacity);
        unsafe { ptr::write(ptr as *mut Cell<usize>, Cell::new(0)) };
         unsafe { alloc::boxed::Box::from_raw(ptr as *mut MemBump) }
     }
 }
     /// This is how many *bytes* can be allocated
     /// within this node.
     pub const fn capacity(&self) -> usize {
        self.data.get().len()
     }
 
     /// Get a raw pointer to the data.
         let requested = layout.size();
 
         // Ensure no overflows when calculating offets within.
        assert!(expect_consumed <= length, "{}/{}", expect_consumed, length);
 
         let available = length.checked_sub(expect_consumed).unwrap();
         let ptr_to = base_ptr.wrapping_add(expect_consumed);
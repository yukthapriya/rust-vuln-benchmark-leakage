// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0136 (FIXED)
// Crate: toolshed
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ratel-rust/toolshed
// Commit: 554d3d9
// File: src/arena.rs


/// a well formatted string.
     pub fn alloc_str_zero_end<'a>(&'a self, val: &str) -> *const u8 {
         let len_with_zero = val.len() + 1;
        let ptr = self.require(len_with_zero);
 
         unsafe {
             use std::ptr::copy_nonoverlapping;
 
             copy_nonoverlapping(val.as_ptr(), ptr, val.len());
             *ptr.offset(val.len() as isize) = 0;
             ptr
         assert_eq!(arena.alloc_str("doge to the moon!"), "doge to the moon!");
         assert_eq!(arena.offset.get(), 32);
     }

    #[test]
    fn alloc_str_zero_end() {
        let arena = Arena::new();
        let ptr = arena.alloc_str_zero_end("abcdefghijk");
        let allocated = unsafe { ::std::slice::from_raw_parts(ptr, 12) };

        assert_eq!(arena.offset.get(), 16);
        assert_eq!(allocated, &[b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', 0]);
    }
 }
// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0136
// CVE: CVE-2020-36456
// CWE: N/A
// Crate: toolshed
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/toolshed
// Extraction Method: repo_search
// GitHub Repo: ratel-rust/toolshed
// Commit: 554d3d9
// File: src/arena.rs


/// a well formatted string.
     pub fn alloc_str_zero_end<'a>(&'a self, val: &str) -> *const u8 {
         let len_with_zero = val.len() + 1;
        let offset = self.require(len_with_zero);
 
         unsafe {
             use std::ptr::copy_nonoverlapping;
 
            let ptr = self.ptr.get().offset(offset as isize);
             copy_nonoverlapping(val.as_ptr(), ptr, val.len());
             *ptr.offset(val.len() as isize) = 0;
             ptr
         assert_eq!(arena.alloc_str("doge to the moon!"), "doge to the moon!");
         assert_eq!(arena.offset.get(), 32);
     }
 }
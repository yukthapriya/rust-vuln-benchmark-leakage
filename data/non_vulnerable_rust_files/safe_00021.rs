// REAL FIXED CODE
// RustSec ID: RUSTSEC-2018-0018 (FIXED)
// Crate: smallvec
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: servo/rust-smallvec
// Commit: 3395246
// File: src/lib.rs


}
 
     pub fn leak<'a>(self) -> &'a mut [T] {
        if !self.spilled() {
            panic!("SmallVec::leak() called on inline (stack) SmallVec, which cannot be safely leaked");
        }
         let mut me = ManuallyDrop::new(self);
         unsafe { core::slice::from_raw_parts_mut(me.as_mut_ptr(), me.len()) }
     }
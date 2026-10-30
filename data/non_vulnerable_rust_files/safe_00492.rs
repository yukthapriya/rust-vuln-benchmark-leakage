// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0016 (FIXED)
// Crate: pared
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: radekvit/pared
// Commit: fdc4de8
// File: src/erased_ptr.rs


#[cfg_attr(coverage_nightly, coverage(off))]
     fn dyn_ptr() {
         // We want to check that the pointers actually ARE compatible
        #![allow(ambiguous_wide_pointer_comparisons)]
 
         let debug: &dyn core::fmt::Debug = &"Hello!";
         let ptr = TypeErasedPtr::new(debug as *const dyn core::fmt::Debug);
     #[cfg_attr(coverage_nightly, coverage(off))]
     fn debug() {
         let ptr = TypeErasedPtr::new(&1);
        let _msg = format!("{:?}", ptr);
     }
 }
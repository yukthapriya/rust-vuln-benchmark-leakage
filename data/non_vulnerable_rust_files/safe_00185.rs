// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0033 (FIXED)
// Crate: stack_dst
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: thepowersgang/stack_dst-rs
// Commit: 4064d20
// File: src/value/trait_impls.rs


(**self).next_back()
     }
 
    // Unstable in MSRV
    //fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
    //    (**self).nth_back(n)
    //}
 }
 d! { ::core::iter::ExactSizeIterator;
     fn len(&self) -> usize { (**self).len() }
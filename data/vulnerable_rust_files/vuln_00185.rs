// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0033
// CVE: CVE-2021-28034
// CWE: N/A
// Crate: stack_dst
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/stack_dst
// Extraction Method: repo_search
// GitHub Repo: thepowersgang/stack_dst-rs
// Commit: 4064d20
// File: src/value/trait_impls.rs


(**self).next_back()
     }
 
    // Included because it can be
    fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
        (**self).nth_back(n)
    }
 }
 d! { ::core::iter::ExactSizeIterator;
     fn len(&self) -> usize { (**self).len() }
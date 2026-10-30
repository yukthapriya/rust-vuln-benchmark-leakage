// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2018-0003
// CVE: CVE-2018-20991
// CWE: N/A
// Crate: smallvec
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/smallvec
// Extraction Method: repo_search
// GitHub Repo: servo/rust-smallvec
// Commit: 3395246
// File: src/lib.rs


}
 
     pub fn leak<'a>(self) -> &'a mut [T] {
         let mut me = ManuallyDrop::new(self);
         unsafe { core::slice::from_raw_parts_mut(me.as_mut_ptr(), me.len()) }
     }
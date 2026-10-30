// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0028
// CVE: N/A
// CWE: N/A
// Crate: buf_redux
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/buf_redux
// Extraction Method: repo_search
// GitHub Repo: abonander/buf_redux
// Commit: f6fbee6
// File: src/lib.rs


use std::cell::RefCell;
 use std::io::prelude::*;
 use std::io::SeekFrom;
use std::{cmp, error, fmt, io, mem, ptr};
 
 #[cfg(all(feature = "nightly", test))]
 mod benches;
 
     // copy the fields out and forget `self` to avoid dropping twice
     fn into_inner_(self) -> (W, Buffer) {
         unsafe {
             // safe because we immediately forget `self`
            let inner = ptr::read(&self.inner);
            let buf = ptr::read(&self.buf);
            mem::forget(self);
             (inner, buf)
         }
     }
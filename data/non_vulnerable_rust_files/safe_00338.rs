// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0028 (FIXED)
// Crate: buf_redux
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: abonander/buf_redux
// Commit: f6fbee6
// File: src/lib.rs


use std::cell::RefCell;
 use std::io::prelude::*;
 use std::io::SeekFrom;
use std::mem::ManuallyDrop;
use std::{cmp, error, fmt, io, ptr};
 
 #[cfg(all(feature = "nightly", test))]
 mod benches;
 
     // copy the fields out and forget `self` to avoid dropping twice
     fn into_inner_(self) -> (W, Buffer) {
        let s = ManuallyDrop::new(self);
         unsafe {
             // safe because we immediately forget `self`
            let inner = ptr::read(&s.inner);
            let buf = ptr::read(&s.buf);
             (inner, buf)
         }
     }
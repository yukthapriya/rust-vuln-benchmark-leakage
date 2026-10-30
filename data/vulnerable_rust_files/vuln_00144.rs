// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0137
// CVE: CVE-2020-36457
// CWE: N/A
// Crate: lever
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/lever
// Extraction Method: repo_search
// GitHub Repo: vertexclique/lever
// Commit: 93d5f36
// File: src/sync/atomics.rs


use crate::sync::arcunique::ArcUnique;
use anyhow::*;
 use std::convert::TryFrom;
 use std::ops::Deref;
 use std::sync::atomic::{AtomicPtr, Ordering};
         Arc::into_raw(total) as *mut T
     }
 
     fn compare_and_swap(&self, current: *mut T, new: *mut T, order: Ordering) -> *mut T {
        self.ptr.compare_and_swap(current, new, order)
     }
 
     fn take(&self) -> Arc<T> {
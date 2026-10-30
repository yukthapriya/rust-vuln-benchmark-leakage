// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0046
// CVE: CVE-2021-29937
// CWE: N/A
// Crate: telemetry
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/telemetry
// Extraction Method: repo_search
// GitHub Repo: Yoric/telemetry.rs
// Commit: 2820cf1
// File: src/keyed.rs


use std::marker::PhantomData;
 use std::mem::size_of;
 
use misc::{Flatten, LinearBuckets, SerializationFormat, vec_resize, vec_with_size};
 use task::{BackEnd, Op, KeyedRawStorage};
 use service::{Service, PrivateAccess};
 use indexing::*;
         match self.values.entry(key) {
             Occupied(mut e) => {
                 let mut vec = e.get_mut();
                vec_resize(&mut vec, value as usize + 1, 0);
                 vec[value as usize] += 1;
             }
             Vacant(e) => {
                 let mut vec = Vec::new();
                vec_resize(&mut vec, value as usize + 1, 0);
                 vec[value as usize] = 1;
                 e.insert(vec);
             }
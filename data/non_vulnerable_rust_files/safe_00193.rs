// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0046 (FIXED)
// Crate: telemetry
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Yoric/telemetry.rs
// Commit: 2820cf1
// File: src/keyed.rs


use std::marker::PhantomData;
 use std::mem::size_of;
 
use misc::{Flatten, LinearBuckets, SerializationFormat, vec_with_size};
 use task::{BackEnd, Op, KeyedRawStorage};
 use service::{Service, PrivateAccess};
 use indexing::*;
         match self.values.entry(key) {
             Occupied(mut e) => {
                 let mut vec = e.get_mut();
                vec.resize(value as usize + 1, 0);
                 vec[value as usize] += 1;
             }
             Vacant(e) => {
                 let mut vec = Vec::new();
                vec.resize(value as usize + 1, 0);
                 vec[value as usize] = 1;
                 e.insert(vec);
             }
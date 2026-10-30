// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0104 (FIXED)
// Crate: gfwx
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Devolutions/gfwx-rs
// Commit: ba8cbdf
// File: src/header/mod.rs


// FromPrimitive and ToPrimitive causes clippy error, so we disable it until
// https://github.com/rust-num/num-derive/issues/20 is fixed
#![cfg_attr(feature = "cargo-clippy", allow(clippy::useless_attribute))]
 
 use std::{io, usize};
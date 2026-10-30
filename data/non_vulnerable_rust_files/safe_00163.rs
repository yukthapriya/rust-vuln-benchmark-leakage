// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0168 (FIXED)
// Crate: mach
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: fitzgen/mach
// Commit: 1c5e6ef
// File: src/lib.rs


#![allow(non_camel_case_types)]
 #![allow(non_upper_case_globals)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![cfg_attr(feature = "rustc-dep-of-std", feature(no_core))]
#![cfg_attr(feature = "rustc-dep-of-std", no_core)]
#![cfg_attr(not(feature = "rustc-dep-of-std"), no_std)]
 #![allow(
     clippy::stutter,
     clippy::cast_sign_loss,
 #[cfg(not(any(target_os = "macos", target_os = "ios")))]
 compile_error!("mach requires MacOSX or iOS");
 
 #[cfg(feature = "rustc-dep-of-std")]
 extern crate rustc_std_workspace_core as core;
 
 extern crate libc;
 
#[allow(unused_imports)]
use core::{clone, cmp, default, fmt, hash, marker, mem, option};
 
 pub mod boolean;
 pub mod bootstrap;
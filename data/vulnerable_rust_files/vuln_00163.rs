// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0168
// CVE: N/A
// CWE: N/A
// Crate: mach
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/mach
// Extraction Method: repo_search
// GitHub Repo: fitzgen/mach
// Commit: 1c5e6ef
// File: src/lib.rs


#![allow(non_camel_case_types)]
 #![allow(non_upper_case_globals)]
#![cfg_attr(not(feature = "std"), no_std)]
 #![allow(
     clippy::stutter,
     clippy::cast_sign_loss,
 #[cfg(not(any(target_os = "macos", target_os = "ios")))]
 compile_error!("mach requires MacOSX or iOS");
 
#[cfg(feature = "std")]
extern crate core;

 #[cfg(feature = "rustc-dep-of-std")]
 extern crate rustc_std_workspace_core as core;
 
 extern crate libc;
 
use core::mem;
 
 pub mod boolean;
 pub mod bootstrap;
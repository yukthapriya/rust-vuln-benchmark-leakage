// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0002
// CVE: CVE-2023-50711
// CWE: N/A
// Crate: vmm-sys-util
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/vmm-sys-util
// Extraction Method: crate_versions
// GitHub Repo: vmm-sys-util
// Commit: N/A
// File: lib.rs


// Copyright 2019 Intel Corporation. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

//! Collection of modules that provides helpers and utilities used by multiple
//! [rust-vmm](https://github.com/rust-vmm/community) components.

#![deny(missing_docs)]
#[cfg(unix)]
#[macro_use]
extern crate bitflags;
extern crate libc;
#[cfg(feature = "with-serde")]
extern crate serde;
#[cfg(feature = "with-serde")]
extern crate serde_derive;

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;

pub mod errno;
pub mod fam;
pub mod rand;
pub mod syscall;
pub mod tempfile;

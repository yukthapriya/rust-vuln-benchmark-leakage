// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0004
// CVE: CVE-2020-35859
// CWE: N/A
// Crate: lucet-runtime-internals
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/lucet-runtime-internals
// Extraction Method: crate_versions
// GitHub Repo: lucet-runtime-internals
// Commit: N/A
// File: lib.rs


//! `lucet-runtime` is a library for loading, running, and monitoring ahead-of-time compiled
//! WebAssembly modules in lightweight sandboxes. It is intended to work with modules compiled by
//! [`lucetc`](https://github.com/fastly/lucet/tree/master/lucetc).

#![deny(bare_trait_objects)]

#[macro_use]
pub mod error;
#[macro_use]
pub mod hostcall_macros;
pub use lucet_runtime_macros::lucet_hostcall;

#[macro_use]
#[cfg(test)]
pub mod test_helpers;

pub mod alloc;
pub mod c_api;
pub mod context;
pub mod embed_ctx;
pub mod instance;
pub mod module;
pub mod region;
pub mod sysdeps;
pub mod val;
pub mod vmctx;

/// The size of a page in WebAssembly heaps.
pub const WASM_PAGE_SIZE: u32 = 64 * 1024;

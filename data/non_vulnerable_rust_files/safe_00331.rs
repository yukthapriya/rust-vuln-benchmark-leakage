// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0014 (FIXED)
// Crate: cortex-m-rt
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-embedded/cortex-m
// Commit: edc79e2
// File: cortex-m-rt/src/lib.rs


//! }
 //! EOF
 //!
//! $ RUSTFLAGS="-C link-arg=-Tlink.x" cargo build --target thumbv7m-none-eabi
 //! $ file target/thumbv7m-none-eabi/debug/app
 //! app: ELF 32-bit LSB executable, ARM, EABI5 version 1 (SYSV), statically linked, (..)
 //! ```
 //!
//! To avoid typing the long command, you can create a `.cargo/config.toml` file:
//!
//! ```toml
//! [target.thumbv7m-none-eabi]
//! rustflags = ["-C", "link-arg=-Tlink.x"]
//!
//! [build]
//! target = "thumbv7m-none-eabi"
//! ```
//!
//! With this configuration, a simple `cargo build` is enough.
//!
//! For Cortex-M4 devices, use the target thumbv7em-none-eabi instead.
//!
 //! # Optional features
 //!
 //! ## `device`
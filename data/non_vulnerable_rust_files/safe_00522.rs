// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0054 (FIXED)
// Crate: array-queue
// Type: unknown (FIXED)
// Extraction Method: crate_versions
// GitHub Repo: array-queue
// Commit: N/A
// File: lib.rs


//! A queue backed by a fixed-size array.

#![no_std]

#[cfg(test)]
extern crate alloc;

mod array_queue;
mod error;

pub use array_queue::ArrayQueue;

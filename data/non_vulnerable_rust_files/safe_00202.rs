// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0066 (FIXED)
// Crate: evm-core
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: rust-blockchain/evm
// Commit: 0fbf596
// File: interpreter/src/runtime.rs


Access,
 }
 
/// The distinction between `RuntimeBaseBackend` and `RuntimeBackend` is for the implementation of
 /// overlays.
 pub trait RuntimeBackend: RuntimeBaseBackend {
 	/// Get original storage value of address at index.
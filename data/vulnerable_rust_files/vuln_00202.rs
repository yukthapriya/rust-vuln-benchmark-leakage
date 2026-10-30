// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0066
// CVE: N/A
// CWE: N/A
// Crate: evm-core
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/evm-core
// Extraction Method: repo_search
// GitHub Repo: rust-blockchain/evm
// Commit: 0fbf596
// File: interpreter/src/runtime.rs


Access,
 }
 
/// The distinguish between `RuntimeBaseBackend` and `RuntimeBackend` is for the implementation of
 /// overlays.
 pub trait RuntimeBackend: RuntimeBaseBackend {
 	/// Get original storage value of address at index.